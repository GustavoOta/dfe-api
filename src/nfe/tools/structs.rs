use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReqXMLToJSON {
    pub parser_type: String,
    pub xml_file_path: Option<String>,
    pub xml_string: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RespXMLToJSON {
    pub error: u8,
    pub msg: String,
    pub data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CertificateInfoAPI {
    pub subject: String,
    pub issuer: String,
    pub valid_from: String,
    pub valid_to: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReqCertificateInfo {
    pub cert_path: String,
    pub cert_password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RespCertificateInfo {
    pub error: u8,
    pub msg: String,
    pub data: Option<CertificateInfoAPI>,
}
