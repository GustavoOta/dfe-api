pub mod tag_cofins;
pub mod tag_icms;
pub mod tag_pis;
pub mod types;

use super::tag_cofins::cofins_by_cst;
use super::tag_icms::icms_by_cst;
use super::tag_pis::pis_by_cst;
use dfe::nfe::types::autorizacao4::Det;
use types::*;

pub trait DETInterface {
    fn process(&self, dets: &[DetApi]) -> Vec<Det>;
    fn det_produtos(&self, det: &DetApi) -> Det;
    fn icms_by_cst(&self, det_temp: &mut Det, det: &DetApi) {
        icms_by_cst(det_temp, det);
    }
    fn pis_by_cst(&self, det_temp: &mut Det, det: &DetApi) {
        pis_by_cst(det_temp, det);
    }
    fn cofins_by_cst(&self, det_temp: &mut Det, det: &DetApi) {
        cofins_by_cst(det_temp, det);
    }
    fn v_tot_trib(&self, det_temp: &mut Det);
}

pub struct DETBuilder;

impl DETInterface for DETBuilder {
    fn process(&self, dets: &[DetApi]) -> Vec<Det> {
        dets.iter()
            .map(|det| {
                let mut det_temp = self.det_produtos(det);
                self.icms_by_cst(&mut det_temp, det);
                self.pis_by_cst(&mut det_temp, det);
                self.cofins_by_cst(&mut det_temp, det);
                self.v_tot_trib(&mut det_temp);
                det_temp
            })
            .collect()
    }

    fn det_produtos(&self, det: &DetApi) -> Det {
        let det_temp = Det {
            c_prod: det.c_prod.clone(),
            x_prod: det.x_prod.clone(),
            ncm: det.ncm.clone(),
            cfop: det.cfop.clone(),
            u_com: det.u_com.clone(),
            q_com: det.q_com,
            v_un_com: det.v_un_com,
            v_prod: det.v_prod,
            u_trib: det.u_trib.clone(),
            q_trib: det.q_trib,
            v_un_trib: det.v_un_trib,
            ind_tot: det.ind_tot,
            x_ped: det.x_ped.clone(),
            n_item_ped: det.n_item_ped.clone(),
            icms: det.icms.clone(),
            orig: Some(det.orig),
            cst: Some(det.cst.clone()),
            pis: det.pis.clone(),
            cofins: det.cofins.clone(),
            inf_ad_prod: det.inf_ad_prod.clone(),
            ..Default::default()
        };
        det_temp
    }
    fn v_tot_trib(&self, det_temp: &mut Det) {
        let tot_icms_valor = det_temp.v_icms.unwrap_or(0.0);
        let tot_pis_valor = det_temp.pis_v_pis.unwrap_or(0.0);
        let tot_cofins_valor = det_temp.cofins_v_cofins.unwrap_or(0.0);

        det_temp.v_tot_trib = tot_icms_valor + tot_pis_valor + tot_cofins_valor;
    }
}
