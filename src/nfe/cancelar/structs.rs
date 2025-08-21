use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReqCancelarNFe {
    pub cert_path: String,
    pub cert_pass: String,
    pub tp_amb: u8,
    pub mod_: Option<u32>,
    pub chave: String,
    pub protocolo: String,
    pub justificativa: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespCancelarNFe {
    pub error: u8,
    pub msg: String,
    pub data: Option<String>,
}
