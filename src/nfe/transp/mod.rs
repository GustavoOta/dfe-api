use dfe::nfe::types::autorizacao4::Transp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranspApi {
    pub mod_frete: u8,
}
pub struct TranspBuilder {
    pub mod_frete: u8,
    // outros campos...
}

impl TranspBuilder {
    pub fn process(transp_api: Option<TranspApi>) -> Transp {
        let mod_frete = match transp_api {
            Some(transp) => transp.mod_frete,
            None => 9, // Valor padrão se não for fornecido
        };

        return Transp {
            mod_frete,
            ..Default::default()
        };
    }
}
/*
transp: Transp {
    mod_frete: post.transp.mod_frete.clone(),
    ..Default::default()
}
 */
