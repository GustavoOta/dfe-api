use dfe::nfe::cancelar::nfe_cancelar;
use dfe::nfe::types::cancelar::*;

use actix_web::http;
use actix_web::{post, web, Responder, Result};

pub mod structs;
use structs::*;
const LOG_FILE_LIMIT: usize = 100; // Limit of log files to keep
const LOG_FILE_PREFIX: &str = "cancelamento";

#[post("/nfe/cancelar")]
pub async fn process(post: web::Json<ReqCancelarNFe>, req: http::Method) -> Result<impl Responder> {
    if req == http::Method::OPTIONS {
        println!("OPTIONS request received for /nfe/cancelar");
        return Ok(web::Json(RespCancelarNFe {
            error: 0,
            msg: "Preflight request".to_string(),
            data: None,
        }));
    }

    if req != http::Method::POST {
        return Ok(web::Json(RespCancelarNFe {
            error: 1,
            msg: "Método de requisição não permitido.".to_string(),
            data: None,
        }));
    }

    let teste = nfe_cancelar(NFeCancelar {
        cert_path: post.cert_path.clone(),
        cert_pass: post.cert_pass.clone(),
        tp_amb: post.tp_amb,
        chave: post.chave.clone(),
        protocolo: post.protocolo.clone(),
        justificativa: post.justificativa.clone(),
    })
    .await;

    if teste.is_err() {
        println!("Error: {:?}", teste);
        return Ok(web::Json(RespCancelarNFe {
            error: 1,
            msg: format!("Erro ao processar cancelamento: {:?}", teste),
            data: None,
        }));
    } else {
        let inf_evento: InfEvento = teste.as_ref().unwrap().response.clone();
        let send_xml = teste.as_ref().unwrap().send_xml.clone();
        let receive_xml = teste.as_ref().unwrap().receive_xml.clone();

        if let Ok(success) = &teste {
            if let Err(e) = save_cancelamento_log(success) {
                println!("Error saving cancelamento log: {:?}", e);
            }
        }

        if let Some(resp) = response_error(&inf_evento) {
            return Ok(web::Json(resp));
        }

        Ok(web::Json(RespCancelarNFe {
            error: 0,
            msg: format!(
                "Cancelamento realizado com sucesso: {} - {}",
                inf_evento.c_stat, inf_evento.x_motivo
            ),
            data: Some(format!(
                "{{\"inf_evento\": {:?}, \"send_xml\": {:?}, \"receive_xml\": {:?}}}",
                inf_evento, send_xml, receive_xml
            )),
        }))
    }
}

fn response_error(inf_evento: &InfEvento) -> Option<RespCancelarNFe> {
    let c_stat = inf_evento.c_stat.as_str();
    let x_motivo = inf_evento.x_motivo.as_str();

    if c_stat == "100" || c_stat == "101" || c_stat == "110" || c_stat == "135" {
        // Success cases 135 is for "Evento registrado e vinculado a NF-e"
        return None;
    }

    match c_stat {
        // Failure cases based on c_stat codes
        "573" => {
            return error_resp("Cancelamento já realizado.", c_stat, x_motivo);
        }
        "155" => {
            return error_resp("Prazo de cancelamento excedido.", c_stat, x_motivo);
        }
        _ => {
            return error_resp("Erro ao cancelar NF-e.", c_stat, x_motivo);
        }
    }
}

fn error_resp(text: &str, c_stat: &str, x_motivo: &str) -> Option<RespCancelarNFe> {
    Some(RespCancelarNFe {
        error: 1,
        msg: format!("{}: Cód[{}] - {}", text, c_stat, x_motivo),
        data: None,
    })
}

fn save_cancelamento_log<T: serde::Serialize>(response: &T) -> std::io::Result<()> {
    use chrono::Local;
    use std::fs;
    use std::fs::File;
    use std::io::Write;

    let log_dir = "./dfe-logs";
    if !fs::metadata(log_dir).is_ok() {
        fs::create_dir(log_dir)?;
    }
    let mut entries: Vec<_> = fs::read_dir(log_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().is_file() && e.file_name().to_string_lossy().starts_with("cancelamento-")
        })
        .collect();

    entries.sort_by_key(|e| e.file_name());

    if entries.len() >= LOG_FILE_LIMIT {
        for entry in &entries[..entries.len() - LOG_FILE_LIMIT + 1] {
            let _ = fs::remove_file(entry.path());
        }
    }

    let now = Local::now();
    let filename = format!(
        "{}/{}-{}.json",
        log_dir,
        LOG_FILE_PREFIX,
        now.format("%Y-%m-%d-%H-%M-%S")
    );
    let mut file = File::create(filename)?;
    let response_str = serde_json::to_string_pretty(response)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    file.write_all(response_str.as_bytes())?;
    Ok(())
}
