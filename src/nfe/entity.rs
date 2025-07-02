use crate::nfe::det::types::DetApi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFeApi {
    pub cert_path: String,
    pub cert_pass: String,
    pub xml_save_path: String,
    pub ide: IdeApi,
    pub emit: EmitApi,
    pub dest: DestApi,
    pub det: Vec<DetApi>,
    pub transp: TranspApi,
    pub pag: PagApi,
    pub inf_adic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeApi {
    pub c_uf: u16,
    pub serie: u32,
    pub n_nf: u64,
    pub id_dest: u8,
    pub c_mun_fg: String,
    pub tp_emis: u8,
    pub tp_amb: u8,
    pub ind_final: u8,
    pub ind_pres: u8,
    pub mod_: u32,
    pub tp_imp: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmitApi {
    pub cnpj: String,
    pub ie: String,
    pub crt: u8,
    pub x_nome: String,
    pub x_fant: String,
    pub x_lgr: String,
    pub nro: String,
    pub x_bairro: String,
    pub c_mun: String,
    pub x_mun: String,
    pub uf: String,
    pub cep: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestApi {
    pub doc: String,
    pub x_nome: String,
    pub x_lgr: String,
    pub nro: String,
    pub x_bairro: String,
    pub c_mun: String,
    pub x_mun: String,
    pub uf: String,
    pub cep: String,
    pub ind_ie_dest: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ie: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranspApi {
    pub mod_frete: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagApi {
    pub ind_pag: u8,
    pub t_pag: String,
    pub v_pag: f64,
    pub tp_integra: Option<u8>,
    pub cnpj: Option<String>,
    pub t_band: Option<String>,
    pub c_aut: Option<String>,
    pub v_troco: Option<String>,
}
/*
Implementar PISST
Nota Fiscal eletrônica
Manual de Orientação do Contribuinte
Pág. 215 / 299
287.1 R01.1 -x- Sequência XML CG R01 1-1 Informar os campos R02 e R03 para cálculo do PIS em
percentual.
288 R02 vBC Valor da Base de Cálculo do PIS E R01.1 N 1-1 13v2
289 R03 pPIS Alíquota do PIS (em percentual) E R01.1 N 1-1 3v2-4
289.1 R03.1 -x- Sequência XML CG R01 1-1 Informar os campos R04 e R05 para cálculo do PIS em valor.
# ID Campo Descrição Ele Pai Tipo Ocor. Tam. Observação
290 R04 qBCProd Quantidade Vendida E R031. N 1-1 12v0-4
291 R05 vAliqProd Alíquota do PIS (em reais) E R03.1 N 1-1 11v0-4
292 R06 vPIS Valor do PIS E R01 N 1-1 13v2

Implementar COFINSST
313 T01 COFINSST Grupo COFINS Substituição Tributária G M01 0-1
313.1 T01.1 -x- Sequência XML CG T01 1-1 Informar os campos T02 e T03 para cálculo da COFINS
Substituição Tributária em percentual.
314 T02 vBC Valor da Base de Cálculo da COFINS E T01.1 N 1-1 13v2
315 T03 pCOFINS Alíquota da COFINS (em percentual) E T01.1 N 1-1 3v2-4
315.1 T03.1 -x- Sequência XML CG T01 1-1 Informar os campos T04 e T05 para cálculo da COFINS
Substituição Tributária em valor.
316 T04 qBCProd Quantidade Vendida E T03.1 N 1-1 12v0-4
317 T05 vAliqProd Alíquota da COFINS (em reais) E T03.1 N 1-1 11v0-4
318 T06 vCOFINS Valor da COFINS E T01 N 1-1 13v2

 */
