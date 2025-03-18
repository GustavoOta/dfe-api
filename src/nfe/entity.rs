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
pub struct DetApi {
    pub c_prod: String,
    pub c_ean: Option<String>,
    pub x_prod: String,
    pub ncm: String,
    pub cest: Option<u16>,
    pub cfop: u16,
    pub u_com: String,
    pub q_com: f32,
    pub v_un_com: f32,
    pub v_prod: f32,
    pub u_trib: String,
    pub q_trib: f32,
    pub v_un_trib: f32,
    pub ind_tot: u8,

    // ICMS
    pub icms: String,
    // ICMS10 **************************
    pub orig: u8,
    pub cst: String,
    pub mod_bc: Option<u8>,
    pub v_bc: Option<f32>,
    pub p_icms: Option<f32>,
    pub mod_bcst: Option<u8>,
    pub p_mvast: Option<f32>,
    pub p_red_bcst: Option<f32>,
    pub v_bcst: Option<f32>,
    pub p_icmsst: Option<f32>,
    pub v_icmsst: Option<f32>,
    // ICMS20 **************************
    // pub orig: u8,
    // pub cst: String,
    // pub mod_bc: Option<u8>,
    pub p_red_bc: Option<f32>,
    // pub v_bc: Option<f32>,
    // pub p_icms: Option<f32>,
    // ** OPCIONAIS **
    pub v_icms_deson: Option<f32>,
    pub mot_des_icms: Option<String>,
    // ICMS30 **************************
    // pub orig: u8,
    // pub cst: String,
    // pub mod_bcst: Option<u8>,
    // pub p_mvast: Option<f32>,
    // pub p_red_bcst: Option<f32>,
    // pub v_bcst: Option<f32>,
    // pub p_icmsst: Option<f32>,
    // pub v_icmsst: Option<f32>,
    // ** OPCIONAIS **
    // pub v_icms_deson: Option<f32>,
    // pub mot_des_icms: Option<String>,
    // ICMS40 **************************
    // pub orig: u8,
    // pub cst: String, // cst 40=isenta, 41=não tributada, 50=suspensão
    // ** OPCIONAIS **
    // pub v_icms_deson: Option<f32>,
    // pub mot_des_icms: Option<String>,
    // ICMS51 **************************
    // pub orig: u8,
    // pub cst: String,
    // pub mod_bc: Option<u8>,
    // pub p_red_bc: Option<f32>,
    // pub v_bc: Option<f32>,
    // pub p_icms: Option<f32>,
    pub p_dif: Option<f32>,
    pub v_icms_dif: Option<f32>,
    // ICMS60 **************************
    // pub orig: u8,
    // pub cst: String,
    // ** OPCIONAIS **
    pub v_bcst_ret: Option<f32>,
    pub v_icmsst_ret: Option<f32>,
    // ICMS70 **************************
    // pub orig: u8,
    // pub cst: String,
    // pub mod_bc: Option<u8>,
    // pub p_red_bc: Option<f32>,
    // pub v_bc: Option<f32>,
    // pub p_icms: Option<f32>,
    // pub mod_bcst: Option<u8>,
    // pub p_mvast: Option<f32>,
    // pub p_red_bcst: Option<f32>,
    // pub v_bcst: Option<f32>,
    // pub p_icmsst: Option<f32>,
    // pub v_icmsst: Option<f32>,
    // ** OPCIONAIS **
    // pub v_icms_deson: Option<f32>,
    // pub mot_des_icms: Option<String>,
    // ICMS90 **************************
    // pub orig: u8,
    // pub cst: String,
    // ** OPCIONAIS **
    // pub mod_bc: Option<u8>,
    // pub v_bc: Option<f32>,
    // pub p_red_bc: Option<f32>,
    // pub p_icms: Option<f32>,
    // ** OPCIONAIS **
    // pub mod_bcst: Option<u8>,
    // pub p_mvast: Option<f32>,
    // pub p_red_bcst: Option<f32>,
    // pub v_bcst: Option<f32>,
    // pub p_icmsst: Option<f32>,
    // pub v_icmsst: Option<f32>,
    // ** OPCIONAIS **
    // pub v_icms_deson: Option<f32>,
    // pub mot_des_icms: Option<String>,
    // ICMSPart **************************
    // pub orig: u8,
    // pub cst: String, // 10=tributada e com cobrança do ICMS por substituição tributária
    // pub mod_bc: Option<u8>,
    // pub v_bc: Option<f32>,
    // pub p_red_bc: Option<f32>,
    // pub p_icms: Option<f32>,
    // pub mod_bcst: Option<u8>,
    // pub p_mvast: Option<f32>,
    // pub p_red_bcst: Option<f32>,
    // pub v_bcst: Option<f32>,
    // pub p_icmsst: Option<f32>,
    // pub v_icmsst: Option<f32>,
    pub p_bcop: Option<f32>,
    pub ufst: Option<String>,
    // ICMSST **************************
    // pub orig: u8,
    // pub cst: String, // 41=não tributada
    // pub v_bcst_ret: Option<f32>,
    // pub v_icmsst_ret: Option<f32>,
    pub v_bcst_dest: Option<f32>,
    pub v_icmsst_dest: Option<f32>,
    // ICMSSN101 **************************
    // pub orig: u8,
    pub csosn: Option<String>,
    pub p_cred_sn: Option<f32>,
    pub v_cred_icmssn: Option<f32>,
    // ICMSSN102 **************************
    // pub orig: u8,
    // pub csosn: String, // 102, 103, 300, 400
    // ICMSSN201 **************************
    // pub orig: u8,
    // pub csosn: String, // 201, 202, 203, 900
    // pub mod_bcst: Option<u8>,
    // pub p_mvast: Option<f32>,
    // pub p_red_bcst: Option<f32>,
    // pub v_bcst: Option<f32>,
    // pub p_icmsst: Option<f32>,
    // pub v_icmsst: Option<f32>,
    //pub p_cred_sn: Option<f32>,
    //pub v_cred_icmssn: Option<f32>,
    // ICMSSN202 **************************
    // pub orig: u8,
    // pub csosn: String, // 202, 203
    //pub mod_bcst: Option<u8>,
    //pub p_mvast: Option<f32>,
    //pub p_red_bcst: Option<f32>,
    //pub v_bcst: Option<f32>,
    //pub p_icmsst: Option<f32>,
    //pub v_icmsst: Option<f32>,
    // ICMSSN500 **************************
    // pub orig: u8,
    // pub csosn: String, // 500
    // ** OPCIONAIS **
    pub bc_st_ret: Option<f32>,
    pub icms_st_ret: Option<f32>,
    // ICMSSN900 **************************
    // pub orig: u8,
    // pub csosn: String, // 900
    // ** OPCIONAIS **
    // pub mod_bc: Option<u8>,
    // pub v_bc: Option<f32>,
    // pub p_red_bc: Option<f32>,
    // pub p_icms: Option<f32>,
    // ** OPCIONAIS **
    // pub mod_bcst: Option<u8>,
    // pub p_mvast: Option<f32>,
    // pub p_red_bcst: Option<f32>,
    // pub v_bcst: Option<f32>,
    // pub p_icmsst: Option<f32>,
    // pub v_icmsst: Option<f32>,
    // ** OPCIONAIS **
    // pub p_cred_sn: Option<f32>,
    // pub v_cred_icmssn: Option<f32>,

    // PIS **************************
    pub pis: String,
    // PISAliq **************************
    pub pis_cst: Option<String>, // 01, 02
    pub pis_v_bc: Option<f32>,
    pub pis_p_pis: Option<f32>,
    // PISQtde **************************
    // pub pis_cst: Option<String>, // 03
    pub pis_q_bc_prod: Option<f32>,
    pub pis_v_aliq_prod: Option<f32>,
    // PISNT **************************
    // Grupo PIS não tributado
    // pub pis_cst: Option<String>, // 04, 05, 06, 07, 08, 09
    // PISOutr **************************
    // pub pis_cst: Option<String>, // 49, 50, 51, 52, 53, 54, 55,56, 60, 61, 62, 63, 64, 65, 66, 67, 70, 71, 72, 73, 74, 75, 98, 99
    // ** OPCIONAIS ** PORCENTAGENS
    //pub pis_v_bc: Option<f32>,
    //pub pis_p_pis: Option<f32>,
    // ** OPCIONAIS ** QUANTIDADE
    //pub pis_q_bc_prod: Option<f32>,
    //pub pis_v_aliq_prod: Option<f32>,
    // PISST **************************
    // TODO: Implementar

    // COFINS **************************
    pub cofins: String,
    // COFINSAliq **************************
    pub cofins_cst: Option<String>, // 01, 02
    pub cofins_v_bc: Option<f32>,
    pub cofins_p_cofins: Option<f32>,
    // COFINSQtde **************************
    // pub cofins_cst: Option<String>, // 03
    pub cofins_q_bc_prod: Option<f32>,
    pub cofins_v_aliq_prod: Option<f32>,
    // COFINSNT **************************
    // Grupo COFINS não tributado
    // pub cofins_cst: Option<String>, // 04, 05, 06, 07, 08, 09
    // COFINSOutr **************************
    // pub cofins_cst: Option<String>, // 49, 50, 51, 52, 53, 54, 55,56, 60, 61, 62, 63, 64, 65, 66, 67, 70, 71, 72, 73, 74, 75, 98, 99
    // ** OPCIONAIS ** PORCENTAGENS
    //pub cofins_v_bc: Option<f32>,
    //pub cofins_p_cofins: Option<f32>,
    // ** OPCIONAIS ** QUANTIDADE
    //pub cofins_q_bc_prod: Option<f32>,
    //pub cofins_v_aliq_prod: Option<f32>,
    // COFINSST **************************
    // TODO: Implementar

    // IPI **************************
    pub cl_enq: Option<String>,
    pub cnpj_prod: Option<String>,
    pub c_selo: Option<String>,
    pub q_selo: Option<String>,
    pub c_enq: Option<String>,
    pub ipi: Option<String>,
    // IPITrib **************************
    pub ipi_cst: Option<String>, // 00, 49, 50, 99
    // ** OPCIONAIS ** PORCENTAGENS
    pub ipi_v_bc: Option<f32>,
    pub ipi_pipi: Option<f32>,
    // ** OPCIONAIS ** QUANTIDADE
    pub ipi_q_unid: Option<f32>,
    pub ipi_v_unid: Option<f32>,
    pub ipi_v_ipi: Option<f32>,
    // IPINT **************************
    // pub ipi_cst: Option<String>, // 01, 02, 03, 04, 05, 51, 52, 53, 54, 55
    // II **************************
    pub ii_v_bc: Option<f32>,       // TODO: impl
    pub ii_v_desp_adu: Option<f32>, // TODO: impl
    pub ii_v_ii: Option<f32>,       // TODO: impl
    pub ii_v_iof: Option<f32>,      // TODO: impl
    // PEDIDO
    pub x_ped: Option<String>,
    pub n_item_ped: Option<String>,
    // PRODUTO OBS
    pub inf_ad_prod: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranspApi {
    pub mod_frete: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagApi {
    pub t_pag: String,
    pub v_pag: f32,
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
