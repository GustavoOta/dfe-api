#![cfg_attr(
    all(target_os = "windows", not(debug_assertions),),
    windows_subsystem = "windows"
)]

use actix_cors::Cors;
use actix_web::dev::Service;
use actix_web::error::JsonPayloadError;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

pub mod common;
pub mod nfe;
pub mod system;
pub mod tests;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub error: u8,
    pub msg: String,
    pub data: Option<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("DFE_API_PORT").unwrap_or("3020".to_string());
    println!("Starting DFE-API Server at http://0.0.0.0:{}/", &port);

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .send_wildcard(),
            )
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                let msg = match &err {
                    JsonPayloadError::Overflow { limit, .. } => {
                        format!(
                            "Payload size is bigger than allowed. (default: {} bytes)",
                            limit
                        )
                    }
                    JsonPayloadError::ContentType => "Content-type error".to_string(),
                    JsonPayloadError::Deserialize(e) => {
                        format!("Error deserializing JSON: {}", e)
                    }
                    _ => "Unknown error".to_string(),
                };

                actix_web::error::InternalError::from_response(
                    err,
                    // Return a 400 Bad Request response with the error message
                    HttpResponse::BadRequest().json(crate::Response {
                        error: 1,
                        msg,
                        data: None,
                    }),
                )
                .into()
            }))
            .default_service(
                web::route()
                    .guard(actix_web::guard::Options())
                    .to(|| async {
                        println!("OPTIONS request");
                        HttpResponse::Ok().finish()
                    })
                    .to(|| async {
                        HttpResponse::NotFound().json(crate::Response {
                            error: 1,
                            msg: "Rota não encontrada".to_string(),
                            data: None,
                        })
                    }),
            )
            .wrap_fn(|req, srv| {
                //println!("Request received: {:?}", req);
                let fut = srv.call(req);
                async {
                    let res = fut.await?;
                    //  println!("Response sent: {:?}", res);
                    Ok(res)
                }
            })
            .service(credits)
            .service(nfe::emitir)
            .service(nfe::cancelar::process)
            .service(nfe::tools::xml_to_json)
            .service(version)
    })
    .bind(("0.0.0.0", port.parse::<u16>().expect("Invalid port")))?
    .run()
    .await
}

#[get["/"]]
async fn credits() -> impl Responder {
    HttpResponse::Ok().body(
        "DFe API - Autor: Gustavo Ota https://github.com/GustavoOta/dfe-api - LICENSE: MIT "
            .to_string()
            + " - *** Versao: "
            + VERSION,
    )
}

#[get["/version"]]
async fn version() -> impl Responder {
    //Response with an json format
    HttpResponse::Ok().json(Response {
        error: 0,
        msg: "Success".to_string(),
        data: Some(VERSION.to_string()),
    })
}
