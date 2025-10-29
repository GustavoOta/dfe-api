use actix_web::http;
use actix_web::{post, web, Responder, Result};
use dfe::nfe::common::cert::CertificateInfo;
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
                    msg: format!(
                        "Erro fatal: Ao tentar converter o arquivo XML para o formato JSON, o formato da estrutura do arquivo XML não é compativel com NFeProc: {:?}",
                        result.unwrap_err()
                    ),
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

#[post("/nfe/tools/certificate-info")]
pub async fn certificate_info(
    post: web::Json<ReqCertificateInfo>,
    req: http::Method,
) -> Result<impl Responder> {
    if req == http::Method::OPTIONS {
        return Ok(web::Json(RespCertificateInfo {
            error: 0,
            msg: "Preflight request".to_string(),
            data: None,
        }));
    }

    if req != http::Method::POST {
        return Ok(web::Json(RespCertificateInfo {
            error: 1,
            msg: "Método de requisição não permitido.".to_string(),
            data: None,
        }));
    }

    let cert_path = post.cert_path.clone();
    //println!("Cert Path: {}", &cert_path);
    let cert_password = post.cert_password.clone();
    if cert_password.is_empty() {
        return Ok(web::Json(RespCertificateInfo {
            error: 1,
            msg: "A senha do certificado é requirida, certifique-se de fornecê-la.".to_string(),
            data: None,
        }));
    }
    if cert_password == "null" || cert_password == "undefined" {
        return Ok(web::Json(RespCertificateInfo {
            error: 1,
            msg: format!("A senha do certificado não pode ser [{}].", cert_password),
            data: None,
        }));
    }
    //println!("Cert Password: {}", &cert_password);

    match CertificateInfo::from_pfx(&cert_path, &cert_password) {
        Ok(info) => {
            // Função auxiliar para converter o formato
            fn format_cert_date(date_str: &str) -> String {
                //Aug  2 14:03:46 2024 GMT
                let parts: Vec<&str> = date_str.split_whitespace().collect();
                if parts.len() == 5 {
                    let month = parts[0];
                    let day = format!("{:0>2}", parts[1]);
                    let time = parts[2];
                    let year = parts[3];
                    // Aug para portugues do brasil e todos outros meses
                    let month = match month {
                        "Jan" => "01",
                        "Feb" => "02",
                        "Mar" => "03",
                        "Apr" => "04",
                        "May" => "05",
                        "Jun" => "06",
                        "Jul" => "07",
                        "Aug" => "08",
                        "Sep" => "09",
                        "Oct" => "10",
                        "Nov" => "11",
                        "Dec" => "12",
                        _ => month,
                    };
                    // Formata a data no padrão DD/MM/YYYY HH:MM:SS
                    return format!("{}/{}/{} {}", day, month, year, time);
                }
                date_str.to_string()
            }

            let valid_from = format_cert_date(&info.valid_from);
            let valid_to = format_cert_date(&info.valid_to);

            let cert_info = CertificateInfoAPI {
                subject: info.subject,
                issuer: info.issuer,
                valid_from,
                valid_to,
            };
            Ok(web::Json(RespCertificateInfo {
                error: 0,
                msg: "Certificate information extracted successfully.".to_string(),
                data: Some(cert_info),
            }))
        }
        Err(e) => Ok(web::Json(RespCertificateInfo {
            error: 1,
            msg: format!("Error extracting certificate info: {}", e),
            data: None,
        })),
    }
}
