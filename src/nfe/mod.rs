pub mod cancelar;
pub mod dest;
pub mod det;
pub mod entity;
mod pag;
pub mod tools;
mod total;

use crate::common::doc_type;
use crate::Response;
use actix_web::http;
use actix_web::{post, web, Responder, Result};
use anyhow::Error as AnyError;
use anyhow::Result as AnyResult;
use det::*;
use dfe::nfe::autorizacao::emit;
use dfe::nfe::types::autorizacao4::*;
use entity::*;
use pag::*;
use serde_json::json;
use total::*;

#[post("/nfe/emitir")]
pub async fn emitir(post: web::Json<NFeApi>, req: http::Method) -> Result<impl Responder> {
    if req == http::Method::OPTIONS {
        return Ok(web::Json(Response {
            error: 0,
            msg: "Preflight request".to_string(),
            data: None,
        }));
    }

    if req != http::Method::POST {
        return Ok(web::Json(Response {
            error: 1,
            msg: "Método de requisição não permitido.".to_string(),
            data: None,
        }));
    }

    let mut inf_adic_process: Option<InfAdic> = None;
    if post.inf_adic.len() > 1 {
        let inf_adic = InfAdic {
            inf_cpl: Some(post.inf_adic.clone()),
            inf_ad_fisco: None,
        };
        inf_adic_process = Some(inf_adic);
    }

    let det_processed: Vec<Det> = DETBuilder.process(&post.det);
    let dest = match dest_builder(&post.dest) {
        Ok(d) => d,
        Err(e) => {
            return Ok(web::Json(Response {
                error: 1,
                msg: format!("Erro ao montar destinatário: {:?}", e),
                data: None,
            }));
        }
    };

    let pagamento = match PagBuilder.process(&post.pag) {
        Ok(p) => p,
        Err(e) => {
            return Ok(web::Json(Response {
                error: 1,
                msg: format!("Erro ao processar a tag pagamento: {:?}", e),
                data: None,
            }));
        }
    };

    let teste = emit(NFe {
        cert_path: post.cert_path.clone(),
        cert_pass: post.cert_pass.clone(),
        ide: Ide {
            c_uf: post.ide.c_uf,
            serie: post.ide.serie,
            n_nf: post.ide.n_nf,
            id_dest: post.ide.id_dest,
            c_mun_fg: post.ide.c_mun_fg.clone(),
            tp_emis: post.ide.tp_emis,
            tp_amb: post.ide.tp_amb,
            ind_final: post.ide.ind_final,
            ind_pres: post.ide.ind_pres,
            mod_: post.ide.mod_,
            tp_imp: post.ide.tp_imp,
            ..Default::default()
        },
        emit: Emit {
            cnpj: Some(post.emit.cnpj.clone()),
            ie: Some(post.emit.ie.clone()),
            crt: post.emit.crt,
            x_nome: post.emit.x_nome.clone(),
            x_fant: Some(post.emit.x_fant.clone()),
            x_lgr: post.emit.x_lgr.clone(),
            nro: post.emit.nro.clone(),
            x_bairro: post.emit.x_bairro.clone(),
            c_mun: post.emit.c_mun.clone(),
            x_mun: post.emit.x_mun.clone(),
            uf: post.emit.uf.clone(),
            cep: post.emit.cep.clone(),
            ..Default::default()
        },
        dest,
        det: det_processed.clone(),
        total: TOTALBuilder.process(&det_processed),
        transp: Transp {
            mod_frete: post.transp.mod_frete.clone(),
            ..Default::default()
        },
        pag: pagamento,
        inf_adic: inf_adic_process,
    })
    .await;

    match teste {
        Err(e) => {
            let msg = format!("{:?}", e); // Mostra toda a cadeia de erros
                                          //println!("Erro ao emitir NFe: {}", msg);
            return Ok(web::Json(Response {
                error: 1,
                msg,
                data: None,
            }));
        }
        Ok(teste) => {
            let teste: dfe::nfe::autorizacao::Response = teste;
            let protocolo: dfe::nfe::autorizacao::TagInfProt = teste.protocolo;
            let inf_prot = protocolo.inf_prot;
            let xml = teste.xml;

            let response_data = json!({
                "protocolo": {
                    "chave": inf_prot.ch_nfe,
                    "data": inf_prot.dh_recbto,
                    "n_prot": inf_prot.n_prot,
                    "dig_val": inf_prot.dig_val,
                    "c_stat": inf_prot.c_stat,
                    "x_motivo": inf_prot.x_motivo
                },
                "xml": xml
            });

            match save_xml_file(&post.emit.cnpj, &post.xml_save_path, &inf_prot.ch_nfe, &xml) {
                Ok(created_files) => {
                    if created_files.error == 1 {
                        return Ok(web::Json(Response {
                            error: 1,
                            msg: format!(
                                "Erro ao tentar salvar o XML e ou diretorios: {:?}",
                                created_files.msg
                            ),
                            data: Some(response_data.to_string()),
                        }));
                    }
                }
                Err(resp) => {
                    return Ok(web::Json(Response {
                        error: 1,
                        msg: format!("Erro ao tentar salvar o XML e ou diretorios: {:?}", resp),
                        data: Some(response_data.to_string()),
                    }));
                }
            }
            return Ok(web::Json(Response {
                error: 0,
                msg: "Resposta do WebService:".to_string(),
                data: Some(response_data.to_string()),
            }));
        }
    }
}

fn dest_builder(dest: &DestApi) -> AnyResult<Dest, AnyError> {
    let doc_type = doc_type(&dest.doc);

    match doc_type.as_str() {
        "CPF" => Ok(Dest {
            cpf: Some(dest.doc.clone()),
            x_nome: Some(dest.x_nome.clone()),
            x_lgr: Some(dest.x_lgr.clone()),
            nro: Some(dest.nro.clone()),
            x_bairro: Some(dest.x_bairro.clone()),
            c_mun: Some(dest.c_mun.clone()),
            x_mun: Some(dest.x_mun.clone()),
            uf: Some(dest.uf.clone()),
            cep: Some(dest.cep.clone()),
            ind_ie_dest: Some(dest.ind_ie_dest),
            ..Default::default()
        }),

        "CNPJ" => {
            if dest.ind_ie_dest == 9 || dest.ind_ie_dest == 2 {
                Ok(Dest {
                    cnpj: Some(dest.doc.clone()),
                    x_nome: Some(dest.x_nome.clone()),
                    x_lgr: Some(dest.x_lgr.clone()),
                    nro: Some(dest.nro.clone()),
                    x_bairro: Some(dest.x_bairro.clone()),
                    c_mun: Some(dest.c_mun.clone()),
                    x_mun: Some(dest.x_mun.clone()),
                    uf: Some(dest.uf.clone()),
                    cep: Some(dest.cep.clone()),
                    ind_ie_dest: Some(dest.ind_ie_dest),
                    ie: None,
                    ..Default::default()
                })
            } else {
                // remover caracteres especiais e espaços da inscricao estadual
                let ie = dest.ie.as_ref().map(|s| {
                    s.replace(".", "")
                        .replace("/", "")
                        .replace("-", "")
                        .replace(" ", "")
                });
                if ie.is_none() || ie.as_ref().unwrap().is_empty() {
                    return Err(AnyError::msg(
                        "Inscrição Estadual não informada ou inválida.",
                    ));
                }
                Ok(Dest {
                    cnpj: Some(dest.doc.clone()),
                    x_nome: Some(dest.x_nome.clone()),
                    x_lgr: Some(dest.x_lgr.clone()),
                    nro: Some(dest.nro.clone()),
                    x_bairro: Some(dest.x_bairro.clone()),
                    c_mun: Some(dest.c_mun.clone()),
                    x_mun: Some(dest.x_mun.clone()),
                    uf: Some(dest.uf.clone()),
                    cep: Some(dest.cep.clone()),
                    ind_ie_dest: Some(dest.ind_ie_dest),
                    ie: ie,
                    ..Default::default()
                })
            }
        }
        "ESTRANGEIRO" => Ok(Dest {
            x_nome: Some(dest.x_nome.clone()),
            x_lgr: Some(dest.x_lgr.clone()),
            nro: Some(dest.nro.clone()),
            x_bairro: Some(dest.x_bairro.clone()),
            c_mun: Some(dest.c_mun.clone()),
            x_mun: Some(dest.x_mun.clone()),
            uf: Some(dest.uf.clone()),
            cep: Some(dest.cep.clone()),
            ind_ie_dest: Some(dest.ind_ie_dest),
            ..Default::default()
        }),
        _ => Err(AnyError::msg("Tipo de documento não suportado.")),
    }
}

fn save_xml_file(
    cnpj: &str,
    dir: &str,
    chave: &str,
    xml: &str,
) -> Result<Response, actix_web::Error> {
    // create dir save_path/cnpj if not exists
    let save_path = format!("{}/{}", dir, cnpj);
    if !std::path::Path::new(&save_path).exists() {
        match std::fs::create_dir_all(&save_path) {
            Ok(_) => {}
            Err(e) => {
                return Ok(Response {
                    error: 1,
                    msg: format!("Erro ao criar o diretorio: {}, {}", &save_path, e),
                    data: None,
                });
            }
        }
    }
    // create inside dir save_path/cnpj/ another dir with the date YYYYMM if not exists
    let date = chrono::Local::now().format("%Y%m").to_string();
    let save_path = format!("{}/{}", save_path, date);
    if !std::path::Path::new(&save_path).exists() {
        match std::fs::create_dir_all(&save_path) {
            Ok(_) => {}
            Err(e) => {
                return Ok(Response {
                    error: 1,
                    msg: format!("Erro ao criar o diretorio: {} {}", &save_path, e),
                    data: None,
                });
            }
        }
    }
    // create file save_path/cnpj/YYYYMM/chave.xml
    let save_path = format!("{}/{}.xml", save_path, chave);
    match std::fs::write(&save_path, xml) {
        Ok(_) => {}
        Err(e) => {
            return Ok(Response {
                error: 1,
                msg: format!("Erro ao salvar o arquivo XML: {} - {}", &save_path, e),
                data: None,
            });
        }
    }

    // save at root directory to view progress
    match std::fs::write("./last_emit.xml", xml) {
        Ok(_) => {}
        Err(e) => {
            return Ok(Response {
                error: 1,
                msg: format!("Erro ao salvar o arquivo last_emit.xml: {}", e),
                data: None,
            });
        }
    }

    return Ok(Response {
        error: 0,
        msg: format!("Diretorios gerados com sucesso"),
        data: None,
    });
}
