use crate::nfe::entity::*;
use anyhow::Error;
use dfe::nfe::types::autorizacao4::Pag as PagResponse;

pub trait PagInterface {
    fn process(&self, pag: &PagApi) -> Result<PagResponse, Error>;
}

pub struct PagBuilder;

impl PagInterface for PagBuilder {
    fn process(&self, pag: &PagApi) -> Result<PagResponse, Error> {
        Ok(PagResponse {
            ind_pag: pag.ind_pag,
            t_pag: pag.t_pag.clone(),
            x_pag: pag.x_pag.clone(),
            v_pag: pag.v_pag,
            tp_integra: pag.tp_integra,
            cnpj: pag.cnpj.clone(),
            t_band: pag.t_band.clone(),
            c_aut: pag.c_aut.clone(),
            v_troco: pag.v_troco.clone(),
        })
    }
}
