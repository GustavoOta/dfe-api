use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RespAsFile {
    pub error: u8,
    pub msg: String,
    pub data: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RespAsBase64 {
    pub error: u8,
    pub msg: String,
    pub data: Option<String>,
}
