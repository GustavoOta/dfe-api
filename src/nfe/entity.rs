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
    pub c_mun_fg: u64,
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
    pub ie: u64,
    pub crt: u8,
    pub x_nome: String,
    pub x_fant: String,
    pub x_lgr: String,
    pub nro: String,
    pub x_bairro: String,
    pub c_mun: u32,
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
    pub c_mun: u32,
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
    pub icms: String,
    pub pis: String,
    pub cofins: String,
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
