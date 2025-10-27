use crate::system::Gravis;
use dfe::nfe::types::autorizacao4::Det;
use dfe::nfe::types::autorizacao4::Total;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

pub struct TOTALBuilder;

impl TOTALBuilder {
    pub fn process(&self, det_processed: &Vec<Det>, desconto_rateio: &Option<Decimal>) -> Total {
        let tot_icms_base_calculo = self.calc_icms_v_bc(&det_processed);
        let tot_icms_valor = self.calc_icms_v_icms(&det_processed);
        let tot_prod_valor = self.calc_icms_v_prod(&det_processed);

        // Simples Nacional
        let tot_cred_icmssn = self.calc_icms_v_cred_icmssn(&det_processed);
        // PIS e COFINS
        let tot_pis_valor = self.pis_v_pis(&det_processed);
        let tot_cofins_valor = self.cofins_v_cofins(&det_processed);
        let tot_nota = tot_prod_valor;
        let tot_tributado = tot_icms_valor + tot_cred_icmssn + tot_pis_valor + tot_cofins_valor;
        // Descontos
        let v_desc = if let Some(desc) = desconto_rateio {
            desc.to_f64().unwrap_or(0.0)
        } else {
            0.0
        };
        let tot_nota = tot_nota - v_desc;
        Total {
            v_bc: tot_icms_base_calculo,
            v_icms: tot_icms_valor,
            v_icms_deson: 0.0,
            v_fcpuf_dest: 0.0,
            v_icms_uf_dest: 0.0,
            v_icms_uf_remet: 0.0,
            v_fcp: 0.0,
            v_bc_st: 0.0,
            v_st: 0.0,
            v_fcpst: 0.0,
            v_fcpst_ret: 0.0,
            v_prod: tot_prod_valor,
            v_frete: 0.0,
            v_seg: 0.0,
            v_desc: v_desc,
            v_ii: 0.0,
            v_ipi: 0.0,
            v_ipi_devol: 0.0,
            v_pis: tot_pis_valor,
            v_cofins: tot_cofins_valor,
            v_outro: 0.0,
            v_nf: tot_nota,
            v_tot_trib: tot_tributado,
        }
    }

    fn calc_icms_v_bc(&self, det_processed: &Vec<Det>) -> f64 {
        let mut v_bc = 0.0;
        for det in det_processed {
            if det.v_bc.is_some() {
                v_bc += det.v_bc.unwrap_or(0.0);
            }
        }
        return v_bc;
    }
    fn calc_icms_v_icms(&self, det_processed: &Vec<Det>) -> f64 {
        let mut v_icms = 0.0;
        for det in det_processed {
            let mut v_bc_sub = 0.0;
            let mut p_icms_sub = 0.0;

            if det.v_bc.is_some() {
                v_bc_sub = det.v_bc.unwrap_or(0.0);
            }
            if det.p_icms.is_some() {
                p_icms_sub = det.p_icms.unwrap_or(0.0);
            }
            v_icms += Gravis.calc_percent(v_bc_sub, p_icms_sub, 2);
        }
        return v_icms;
    }
    fn calc_icms_v_prod(&self, det_processed: &Vec<Det>) -> f64 {
        let mut v_prod = 0.0;
        for det in det_processed {
            v_prod += det.v_prod;
        }
        return v_prod;
    }
    // Simples Nacional
    fn calc_icms_v_cred_icmssn(&self, det_processed: &Vec<Det>) -> f64 {
        let mut v_cred_icmssn = 0.0;
        for det in det_processed {
            if det.v_cred_icmssn.is_some() {
                v_cred_icmssn += det.v_cred_icmssn.unwrap();
            }
        }
        return v_cred_icmssn;
    }
    fn pis_v_pis(&self, det_processed: &Vec<Det>) -> f64 {
        let mut pis_v_pis = 0.0;
        for det in det_processed {
            if det.pis_v_pis.is_some() {
                pis_v_pis += det.pis_v_pis.unwrap();
            }
        }
        return pis_v_pis;
    }
    fn cofins_v_cofins(&self, det_processed: &Vec<Det>) -> f64 {
        let mut cofins_v_cofins = 0.0;
        for det in det_processed {
            if det.cofins_v_cofins.is_some() {
                cofins_v_cofins += det.cofins_v_cofins.unwrap();
            }
        }
        return cofins_v_cofins;
    }
}
