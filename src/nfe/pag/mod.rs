use crate::nfe::entity::*;
use anyhow::Error;
use dfe::nfe::types::autorizacao4::Pag as PagResponse;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

pub struct PagBuilder;

impl PagBuilder {
    pub fn process(
        &self,
        pag: &PagApi,
        desconto_rateio: &Option<Decimal>,
    ) -> Result<PagResponse, Error> {
        let v_pag_adjusted = if let Some(desc) = desconto_rateio {
            let desc_f64 = desc.to_f64().unwrap_or(0.0);
            let original_v_pag = pag.v_pag;
            let desconto_applied = original_v_pag - desc_f64;
            if desconto_applied < 0.0 {
                0.0
            } else {
                desconto_applied
            }
        } else {
            pag.v_pag
        };
        Ok(PagResponse {
            ind_pag: pag.ind_pag,
            t_pag: pag.t_pag.clone(),
            x_pag: pag.x_pag.clone(),
            v_pag: v_pag_adjusted,
            tp_integra: pag.tp_integra,
            cnpj: pag.cnpj.clone(),
            t_band: pag.t_band.clone(),
            c_aut: pag.c_aut.clone(),
            v_troco: pag.v_troco.clone(),
        })
    }
}
