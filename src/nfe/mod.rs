pub mod dest;
pub mod entity;

use crate::common::doc_type;
use crate::Response;
use actix_web::http;
use actix_web::{post, web, Responder, Result};
use anyhow::Error as AnyError;
use anyhow::Result as AnyResult;
use dfe::nfe::autorizacao::emit;
use dfe::nfe::types::autorizacao4::*;
use entity::*;
use serde_json::json;

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
        dest: dest_builder(&post.dest).unwrap(),
        det: post
            .det
            .iter()
            .map(|det| det_builder(det))
            .collect::<Vec<Det>>(),
        total: total_builder(&post.det),
        transp: Transp {
            mod_frete: post.transp.mod_frete.clone(),
            ..Default::default()
        },
        pag: Pag {
            ind_pag: post.pag.ind_pag,
            t_pag: post.pag.t_pag.clone(),
            v_pag: post.pag.v_pag,
        },
        inf_adic: inf_adic_process,
    })
    .await;
    //println!("{:?}", teste);
    if let Err(e) = teste {
        return Ok(web::Json(Response {
            error: 1,
            msg: format!("{:?}", e),
            data: None,
        }));
    } else {
        let teste: dfe::nfe::autorizacao::Response = teste.unwrap();
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

        if !save_xml_file(&post.emit.cnpj, &post.xml_save_path, &inf_prot.ch_nfe, &xml) {
            return Ok(web::Json(Response {
                error: 1,
                msg: format!("Erro ao salvar o arquivo XML em: {:?}", &post.xml_save_path),
                data: None,
            }));
        }
        return Ok(web::Json(Response {
            error: 0,
            msg: "Resposta do WebService:".to_string(),
            data: Some(response_data.to_string()),
        }));
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
                    ie: dest.ie.clone(),
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

fn total_builder(post_det: &Vec<DetApi>) -> Total {
    let mut v_bc = 0.0;
    // let mut p_icms = 0.0;
    let mut v_icms = 0.0;
    let mut v_prod = 0.0;
    for det in post_det {
        let mut v_bc_sub = 0.0;
        let mut p_icms_sub = 0.0;
        if det.v_bc.is_some() {
            v_bc += det.v_bc.unwrap_or(0.0);
            v_bc_sub = det.v_bc.unwrap_or(0.0);
        }
        if det.p_icms.is_some() {
            //p_icms += det.p_icms.unwrap_or(0.0);
            p_icms_sub = det.p_icms.unwrap_or(0.0);
        }
        // calc v_icms = v_bc * p_icms
        v_icms += v_bc_sub * p_icms_sub / 100.0;
        v_prod += det.v_prod;
    }

    Total {
        v_bc,
        v_icms,
        v_icms_deson: 0.0,
        v_fcpuf_dest: 0.0,
        v_icms_uf_dest: 0.0,
        v_icms_uf_remet: 0.0,
        v_fcp: 0.0,
        v_bc_st: 0.0,
        v_st: 0.0,
        v_fcpst: 0.0,
        v_fcpst_ret: 0.0,
        v_prod,
        v_frete: 0.0,
        v_seg: 0.0,
        v_desc: 0.0,
        v_ii: 0.0,
        v_ipi: 0.0,
        v_ipi_devol: 0.0,
        v_pis: 0.0,
        v_cofins: 0.0,
        v_outro: 0.0,
        v_nf: v_prod,
        v_tot_trib: 0.0,
    }
}

fn det_builder(det: &DetApi) -> Det {
    //println!("{:?}", det);
    let mut det_temp = Det {
        c_prod: det.c_prod.clone(),
        x_prod: det.x_prod.clone(),
        ncm: det.ncm.clone(),
        cfop: det.cfop.clone(),
        u_com: det.u_com.clone(),
        q_com: det.q_com,
        v_un_com: det.v_un_com,
        v_prod: det.v_prod,
        u_trib: det.u_trib.clone(),
        q_trib: det.q_trib,
        v_un_trib: det.v_un_trib,
        ind_tot: det.ind_tot,
        x_ped: det.x_ped.clone(),
        n_item_ped: det.n_item_ped.clone(),
        icms: det.icms.clone(),
        orig: Some(det.orig),
        cst: Some(det.cst.clone()),
        pis: det.pis.clone(),
        cofins: det.cofins.clone(),
        inf_ad_prod: det.inf_ad_prod.clone(),
        ..Default::default()
    };

    // Se caso o ICMS00, ou outros deve adicionar BC e ICMS
    match det.cst.as_str() {
        "00" => {
            // Se det.p_icms for none, então não é possível calcular o v_bc e p_icms
            if det.p_icms.is_none() {
                return det_temp;
            }
            if det.v_bc.is_none() {
                return det_temp;
            }
            let p_icms = det
                .p_icms
                .expect("p_icms não pode ser None pois o CST é 00");
            let v_bc = det.v_bc.expect("v_bc não pode ser None pois o CST é 00");
            det_temp.mod_bc = det.mod_bc.clone();
            det_temp.p_icms = det.p_icms;
            det_temp.v_bc = det.v_bc;
            // Assumindo que v_bc e p_icms são f64
            let v_icms_multi = v_bc * p_icms / 100.0;
            // Limitar para 2 casas decimais
            let v_icms_arredondado = (v_icms_multi * 100.0).round() / 100.0;
            det_temp.v_icms = Some(v_icms_arredondado);
        }
        _ => {}
    }
    return det_temp;
}

fn save_xml_file(cnpj: &str, dir: &str, chave: &str, xml: &str) -> bool {
    // create dir save_path/cnpj if not exists
    let save_path = format!("{}/{}", dir, cnpj);
    if !std::path::Path::new(&save_path).exists() {
        std::fs::create_dir_all(&save_path).unwrap();
    }
    // create inside dir save_path/cnpj/ another dir with the date YYYYMM if not exists
    let date = chrono::Local::now().format("%Y%m").to_string();
    let save_path = format!("{}/{}", save_path, date);
    if !std::path::Path::new(&save_path).exists() {
        std::fs::create_dir_all(&save_path).unwrap();
    }
    // create file save_path/cnpj/YYYYMM/chave.xml
    let save_path = format!("{}/{}.xml", save_path, chave);
    std::fs::write(save_path, xml).unwrap();

    true
}
