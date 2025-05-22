use super::types::*;
use crate::system::{Gravis, GravisInterface};
use dfe::nfe::types::autorizacao4::Det;

pub fn icms_by_cst(det_temp: &mut Det, det: &DetApi) {
    match det_temp.cst.as_deref() {
        Some("00") => {
            if let (Some(mod_bc), Some(p_icms), Some(v_bc)) = (det.mod_bc, det.p_icms, det.v_bc) {
                det_temp.mod_bc = Some(mod_bc);
                det_temp.p_icms = Some(p_icms);
                det_temp.v_bc = Some(v_bc);
                det_temp.v_icms = Some(Gravis.calc_percent(v_bc, p_icms, 2));
            }
        }
        // Adicione outros ICMS CSTs aqui conforme necessário
        _ => {}
    }
}
