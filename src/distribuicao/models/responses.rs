use dfe::distribuicao::{DistribuicaoResposta, ManifestacaoResposta};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RespConsulta {
    pub error: u8,
    pub msg: String,
    pub data: Option<DistribuicaoResposta>,
}

#[derive(Serialize, Deserialize)]
pub struct RespManifestacao {
    pub error: u8,
    pub msg: String,
    pub data: Option<ManifestacaoResposta>,
}
