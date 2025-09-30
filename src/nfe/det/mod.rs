pub mod tag_cofins;
pub mod tag_icms;
pub mod tag_pis;
pub mod types;

use super::tag_cofins::cofins_by_cst;
use super::tag_icms::icms_by_cst;
use super::tag_pis::pis_by_cst;
use dfe::nfe::types::autorizacao4::Det;
use types::*;
pub mod tag_ibs_cbs;
use tag_ibs_cbs::IbsCbs;

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
                IbsCbs::build(&mut det_temp, det);
                self.v_tot_trib(&mut det_temp);
                det_temp
            })
            .collect()
    }

    fn det_produtos(&self, det: &DetApi) -> Det {
        let ncm = det.ncm.clone().unwrap_or("ncm não enviado".to_string());
        let cfop = det.cfop.clone().unwrap_or(0);

        let det_temp = Det {
            c_prod: det.c_prod.clone(),
            x_prod: det.x_prod.clone(),
            ncm,
            cfop,
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
            cst: det.cst.clone(),
            csosn: det.csosn.clone(),
            pis: det.pis.clone(),
            pis_cst: det.pis_cst.clone(),
            pis_v_bc: det.pis_v_bc,
            pis_p_pis: det.pis_p_pis,
            pis_v_pis: det.pis_v_pis,
            pis_q_bc_prod: det.pis_q_bc_prod,
            pis_v_aliq_prod: det.pis_v_aliq_prod,
            cofins: det.cofins.clone(),
            cofins_cst: det.cofins_cst.clone(),
            cofins_v_bc: det.cofins_v_bc,
            cofins_p_cofins: det.cofins_p_cofins,
            cofins_v_cofins: det.cofins_v_cofins,
            cofins_q_bc_prod: det.cofins_q_bc_prod,
            cofins_v_aliq_prod: det.cofins_v_aliq_prod,
            inf_ad_prod: det.inf_ad_prod.clone(),
            ..Default::default()
        };
        det_temp
    }
    fn v_tot_trib(&self, det_temp: &mut Det) {
        let tot_icms_valor = det_temp.v_icms.unwrap_or(0.0);
        let tot_pis_valor = det_temp.pis_v_pis.unwrap_or(0.0);
        let tot_cofins_valor = det_temp.cofins_v_cofins.unwrap_or(0.0);

        // Simples Nacional
        let tot_v_cred_icmssn = det_temp.v_cred_icmssn.unwrap_or(0.0);

        det_temp.v_tot_trib = tot_icms_valor + tot_pis_valor + tot_cofins_valor + tot_v_cred_icmssn;
    }
}
