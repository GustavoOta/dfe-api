use crate::pdf::models::requests::*;
use crate::pdf::models::responses::*;
use actix_web::{web, Responder};
use dfe::pdf::DanfeBuilder;

pub struct PDFServices;

impl PDFServices {
    pub async fn as_file(req: web::Json<ReqAsFile>) -> impl Responder {
        let danfe = DanfeBuilder::new()
            .xml(&req.xml)
            .paper_size(&req.paper_size)
            .as_file(&req.as_file)
            .build()
            .await;

        match danfe {
            Ok(pdf_path) => web::Json(RespAsFile {
                error: 0,
                msg: "PDF as file endpoint".to_string(),
                data: Some(pdf_path),
            }),
            Err(e) => {
                return web::Json(RespAsFile {
                    error: 1,
                    msg: format!("DFE-API:{}", e),
                    data: None,
                })
            }
        }
    }

    pub async fn as_base64(req: web::Json<ReqAsBase64>) -> impl Responder {
        let danfe = DanfeBuilder::new()
            .xml(&req.xml)
            .paper_size(&req.paper_size)
            .as_base64()
            .build()
            .await;

        match danfe {
            Ok(pdf_base64) => web::Json(RespAsBase64 {
                error: 0,
                msg: "PDF as base64 endpoint".to_string(),
                data: Some(pdf_base64),
            }),
            Err(e) => {
                return web::Json(RespAsBase64 {
                    error: 1,
                    msg: format!("DFE-API:{}", e),
                    data: None,
                })
            }
        }
    }
}
