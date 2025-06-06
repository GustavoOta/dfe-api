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
