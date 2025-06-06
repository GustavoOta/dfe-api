use actix_web::http;
use actix_web::{post, web, Responder, Result};
use dfe::nfe::xml_extractor::structs::*;
use dfe::nfe::xml_extractor::*;

pub mod structs;
use structs::*;

#[post("/nfe/tools/xml-to-json")]
pub async fn xml_to_json(
    post: web::Json<ReqXMLToJSON>,
    req: http::Method,
) -> Result<impl Responder> {
    if req == http::Method::OPTIONS {
        return Ok(web::Json(RespXMLToJSON {
            error: 0,
            msg: "Preflight request".to_string(),
            data: None,
        }));
    }

    if req != http::Method::POST {
        return Ok(web::Json(RespXMLToJSON {
            error: 1,
            msg: "Método de requisição não permitido.".to_string(),
            data: None,
        }));
    }

    let parser_type = post.parser_type.clone();
    let xml_file_path = post.xml_file_path.clone();
    let xml_string = post.xml_string.clone();
    let xml_extractor = XmlExtractor::new();
    match parser_type.as_str() {
        "file" => {
            let result: Result<NFeProc, XMLExtractorError> =
                xml_extractor.nfe_proc_from_file(xml_file_path.as_deref().unwrap_or(""));
            if result.is_err() {
                return Ok(web::Json(RespXMLToJSON {
                    error: 1,
                    msg: format!("Error processing XML file: {:?}", result.unwrap_err()),
                    data: None,
                }));
            }
            let result: NFeProc = result.unwrap();
            // return NFeProc as JSON response
            Ok(web::Json(RespXMLToJSON {
                error: 0,
                msg: "XML file processed successfully.".to_string(),
                data: Some(
                    serde_json::to_string(&result)
                        .unwrap_or_else(|_| "Error converting to JSON".to_string()),
                ),
            }))
        }
        "string" => {
            let result: Result<NFeProc, XMLExtractorError> = xml_extractor.nfe_proc_from_string(
                &xml_string
                    .as_deref()
                    .unwrap_or("xml_string: Option<String> is None"),
            );
            if result.is_err() {
                return Ok(web::Json(RespXMLToJSON {
                    error: 1,
                    msg: format!("Error processing XML string: {:?}", result.unwrap_err()),
                    data: None,
                }));
            }
            let result: NFeProc = result.unwrap();

            Ok(web::Json(RespXMLToJSON {
                error: 0,
                msg: "XML string processed successfully.".to_string(),
                data: Some(
                    serde_json::to_string(&result)
                        .unwrap_or_else(|_| "Error converting to JSON".to_string()),
                ),
            }))
        }
        _ => Ok(web::Json(RespXMLToJSON {
            error: 1,
            msg: "Invalid parser_type. Use 'file' or 'string'.".to_string(),
            data: None,
        })),
    }
}
