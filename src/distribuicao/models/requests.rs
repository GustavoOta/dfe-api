use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ReqConsulta {
    pub cert_path: String,
    pub cert_pass: String,
    pub cnpj: String,
    pub uf: u8,
    pub ambiente: u8,
}

#[derive(Serialize, Deserialize)]
pub struct ReqConsultaUltNSU {
    pub cert_path: String,
    pub cert_pass: String,
    pub cnpj: String,
    pub uf: u8,
    pub ambiente: u8,
    pub nsu: String,
    pub check_flag: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct ReqConsultaChaveAcesso {
    pub cert_path: String,
    pub cert_pass: String,
    pub cnpj: String,
    pub uf: u8,
    pub ambiente: u8,
    pub chave_acesso: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReqManifestacao {
    pub cert_path: String,
    pub cert_pass: String,
    pub cnpj: String,
    pub ambiente: u8,
    pub chave_acesso: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReqOperacaoNaoRealizada {
    pub cert_path: String,
    pub cert_pass: String,
    pub cnpj: String,
    pub ambiente: u8,
    pub chave_acesso: String,
    pub justificativa: String,
}
