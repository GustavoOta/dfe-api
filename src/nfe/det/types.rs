use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetApi {
    pub c_prod: String,
    pub c_ean: Option<String>,
    pub x_prod: String,
    pub ncm: Option<String>,
    pub cest: Option<u16>,
    pub cfop: Option<u16>,
    pub u_com: String,
    pub q_com: f64,
    pub v_un_com: f64,
    pub v_prod: f64,
    pub u_trib: String,
    pub q_trib: f64,
    pub v_un_trib: f64,
    pub ind_tot: u8,
    // PEDIDO
    pub x_ped: Option<String>,
    pub n_item_ped: Option<String>,

    // ICMS
    pub icms: String,
    // ICMS10 **************************
    pub orig: u8,
    pub cst: Option<String>,
    pub mod_bc: Option<u8>,
    pub v_bc: Option<f64>,
    pub p_icms: Option<f64>,
    pub mod_bcst: Option<u8>,
    pub p_mvast: Option<f64>,
    pub p_red_bcst: Option<f64>,
    pub v_bcst: Option<f64>,
    pub p_icmsst: Option<f64>,
    pub v_icmsst: Option<f64>,
    // ICMS20 **************************
    // pub orig: u8,
    // pub cst: String,
    // pub mod_bc: Option<u8>,
    pub p_red_bc: Option<f64>,
    // pub v_bc: Option<f64>,
    // pub p_icms: Option<f64>,
    // ** OPCIONAIS **
    pub v_icms_deson: Option<f64>,
    pub mot_des_icms: Option<String>,
    // ICMS30 **************************
    // pub orig: u8,
    // pub cst: String,
    // pub mod_bcst: Option<u8>,
    // pub p_mvast: Option<f64>,
    // pub p_red_bcst: Option<f64>,
    // pub v_bcst: Option<f64>,
    // pub p_icmsst: Option<f64>,
    // pub v_icmsst: Option<f64>,
    // ** OPCIONAIS **
    // pub v_icms_deson: Option<f64>,
    // pub mot_des_icms: Option<String>,
    // ICMS40 **************************
    // pub orig: u8,
    // pub cst: String, // cst 40=isenta, 41=não tributada, 50=suspensão
    // ** OPCIONAIS **
    // pub v_icms_deson: Option<f64>,
    // pub mot_des_icms: Option<String>,
    // ICMS51 **************************
    // pub orig: u8,
    // pub cst: String,
    // pub mod_bc: Option<u8>,
    // pub p_red_bc: Option<f64>,
    // pub v_bc: Option<f64>,
    // pub p_icms: Option<f64>,
    pub p_dif: Option<f64>,
    pub v_icms_dif: Option<f64>,
    // ICMS60 **************************
    // pub orig: u8,
    // pub cst: String,
    // ** OPCIONAIS **
    pub v_bcst_ret: Option<f64>,
    pub v_icmsst_ret: Option<f64>,
    // ICMS70 **************************
    // pub orig: u8,
    // pub cst: String,
    // pub mod_bc: Option<u8>,
    // pub p_red_bc: Option<f64>,
    // pub v_bc: Option<f64>,
    // pub p_icms: Option<f64>,
    // pub mod_bcst: Option<u8>,
    // pub p_mvast: Option<f64>,
    // pub p_red_bcst: Option<f64>,
    // pub v_bcst: Option<f64>,
    // pub p_icmsst: Option<f64>,
    // pub v_icmsst: Option<f64>,
    // ** OPCIONAIS **
    // pub v_icms_deson: Option<f64>,
    // pub mot_des_icms: Option<String>,
    // ICMS90 **************************
    // pub orig: u8,
    // pub cst: String,
    // ** OPCIONAIS **
    // pub mod_bc: Option<u8>,
    // pub v_bc: Option<f64>,
    // pub p_red_bc: Option<f64>,
    // pub p_icms: Option<f64>,
    // ** OPCIONAIS **
    // pub mod_bcst: Option<u8>,
    // pub p_mvast: Option<f64>,
    // pub p_red_bcst: Option<f64>,
    // pub v_bcst: Option<f64>,
    // pub p_icmsst: Option<f64>,
    // pub v_icmsst: Option<f64>,
    // ** OPCIONAIS **
    // pub v_icms_deson: Option<f64>,
    // pub mot_des_icms: Option<String>,
    // ICMSPart **************************
    // pub orig: u8,
    // pub cst: String, // 10=tributada e com cobrança do ICMS por substituição tributária
    // pub mod_bc: Option<u8>,
    // pub v_bc: Option<f64>,
    // pub p_red_bc: Option<f64>,
    // pub p_icms: Option<f64>,
    // pub mod_bcst: Option<u8>,
    // pub p_mvast: Option<f64>,
    // pub p_red_bcst: Option<f64>,
    // pub v_bcst: Option<f64>,
    // pub p_icmsst: Option<f64>,
    // pub v_icmsst: Option<f64>,
    pub p_bcop: Option<f64>,
    pub ufst: Option<String>,
    // ICMSST **************************
    // pub orig: u8,
    // pub cst: String, // 41=não tributada
    // pub v_bcst_ret: Option<f64>,
    // pub v_icmsst_ret: Option<f64>,
    pub v_bcst_dest: Option<f64>,
    pub v_icmsst_dest: Option<f64>,
    // ICMSSN101 **************************
    // pub orig: u8,
    pub csosn: Option<String>,
    pub p_cred_sn: Option<f64>,
    pub v_cred_icmssn: Option<f64>,
    // ICMSSN102 **************************
    // pub orig: u8,
    // pub csosn: String, // 102, 103, 300, 400
    // ICMSSN201 **************************
    // pub orig: u8,
    // pub csosn: String, // 201, 202, 203, 900
    // pub mod_bcst: Option<u8>,
    // pub p_mvast: Option<f64>,
    // pub p_red_bcst: Option<f64>,
    // pub v_bcst: Option<f64>,
    // pub p_icmsst: Option<f64>,
    // pub v_icmsst: Option<f64>,
    //pub p_cred_sn: Option<f64>,
    //pub v_cred_icmssn: Option<f64>,
    // ICMSSN202 **************************
    // pub orig: u8,
    // pub csosn: String, // 202, 203
    //pub mod_bcst: Option<u8>,
    //pub p_mvast: Option<f64>,
    //pub p_red_bcst: Option<f64>,
    //pub v_bcst: Option<f64>,
    //pub p_icmsst: Option<f64>,
    //pub v_icmsst: Option<f64>,
    // ICMSSN500 **************************
    // pub orig: u8,
    // pub csosn: String, // 500
    // ** OPCIONAIS **
    pub bc_st_ret: Option<f64>,
    pub icms_st_ret: Option<f64>,
    // ICMSSN900 **************************
    // pub orig: u8,
    // pub csosn: String, // 900
    // ** OPCIONAIS **
    // pub mod_bc: Option<u8>,
    // pub v_bc: Option<f64>,
    // pub p_red_bc: Option<f64>,
    // pub p_icms: Option<f64>,
    // ** OPCIONAIS **
    // pub mod_bcst: Option<u8>,
    // pub p_mvast: Option<f64>,
    // pub p_red_bcst: Option<f64>,
    // pub v_bcst: Option<f64>,
    // pub p_icmsst: Option<f64>,
    // pub v_icmsst: Option<f64>,
    // ** OPCIONAIS **
    // pub p_cred_sn: Option<f64>,
    // pub v_cred_icmssn: Option<f64>,

    // PIS **************************
    pub pis: String,
    // PISAliq **************************
    pub pis_cst: Option<String>, // 01, 02
    pub pis_v_bc: Option<f64>,
    pub pis_p_pis: Option<f64>,
    pub pis_v_pis: Option<f64>,
    // PISQtde **************************
    // pub pis_cst: Option<String>, // 03
    pub pis_q_bc_prod: Option<f64>,
    pub pis_v_aliq_prod: Option<f64>,
    // PISNT **************************
    // Grupo PIS não tributado
    // pub pis_cst: Option<String>, // 04, 05, 06, 07, 08, 09
    // PISOutr **************************
    // pub pis_cst: Option<String>, // 49, 50, 51, 52, 53, 54, 55,56, 60, 61, 62, 63, 64, 65, 66, 67, 70, 71, 72, 73, 74, 75, 98, 99
    // ** OPCIONAIS ** PORCENTAGENS
    //pub pis_v_bc: Option<f64>,
    //pub pis_p_pis: Option<f64>,
    // ** OPCIONAIS ** QUANTIDADE
    //pub pis_q_bc_prod: Option<f64>,
    //pub pis_v_aliq_prod: Option<f64>,
    // PISST **************************
    // TODO: Implementar

    // COFINS **************************
    pub cofins: String,
    // COFINSAliq **************************
    pub cofins_cst: Option<String>, // 01, 02
    pub cofins_v_bc: Option<f64>,
    pub cofins_p_cofins: Option<f64>,
    pub cofins_v_cofins: Option<f64>,
    // COFINSQtde **************************
    // pub cofins_cst: Option<String>, // 03
    pub cofins_q_bc_prod: Option<f64>,
    pub cofins_v_aliq_prod: Option<f64>,
    // COFINSNT **************************
    // Grupo COFINS não tributado
    // pub cofins_cst: Option<String>, // 04, 05, 06, 07, 08, 09
    // COFINSOutr **************************
    // pub cofins_cst: Option<String>, // 49, 50, 51, 52, 53, 54, 55,56, 60, 61, 62, 63, 64, 65, 66, 67, 70, 71, 72, 73, 74, 75, 98, 99
    // ** OPCIONAIS ** PORCENTAGENS
    //pub cofins_v_bc: Option<f64>,
    //pub cofins_p_cofins: Option<f64>,
    // ** OPCIONAIS ** QUANTIDADE
    //pub cofins_q_bc_prod: Option<f64>,
    //pub cofins_v_aliq_prod: Option<f64>,
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
    pub ipi_v_bc: Option<f64>,
    pub ipi_pipi: Option<f64>,
    // ** OPCIONAIS ** QUANTIDADE
    pub ipi_q_unid: Option<f64>,
    pub ipi_v_unid: Option<f64>,
    pub ipi_v_ipi: Option<f64>,
    // IPINT **************************
    // pub ipi_cst: Option<String>, // 01, 02, 03, 04, 05, 51, 52, 53, 54, 55
    // II **************************
    pub ii_v_bc: Option<f64>,       // TODO: impl
    pub ii_v_desp_adu: Option<f64>, // TODO: impl
    pub ii_v_ii: Option<f64>,       // TODO: impl
    pub ii_v_iof: Option<f64>,      // TODO: impl
    // PRODUTO OBS
    pub inf_ad_prod: Option<String>,
}
