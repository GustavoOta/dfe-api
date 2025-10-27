use super::types::*;
use crate::system::Gravis;
use dfe::nfe::types::autorizacao4::Det;

pub fn pis_by_cst(det_temp: &mut Det, det: &DetApi) {
    match det.pis.as_str() {
        "PISAliq" => {
            match det.pis_cst.as_deref() {
                Some("01") => {
                    det_temp.pis_cst = Some("01".to_string());
                    if let (Some(pis_v_bc), Some(pis_p_pis)) = (det.pis_v_bc, det.pis_p_pis) {
                        let pis_total = Gravis.calc_percent(pis_v_bc, pis_p_pis, 2);
                        det_temp.pis_v_bc = Some(pis_v_bc);
                        det_temp.pis_p_pis = Some(pis_p_pis);
                        det_temp.pis_v_pis = Some(pis_total);
                    }
                }
                Some("02") => {
                    det_temp.pis_cst = Some("02".to_string());
                    // Lógica para CST 02 pode ser adicionada aqui
                }
                Some(cst) => {
                    det_temp.pis_cst = Some(format!("{} inválido", cst));
                }
                None => {
                    det_temp.pis_cst = Some("PIS CST não enviado".to_string());
                }
            }
        }
        // Adicione outros PIS CSTs aqui conforme necessário
        _ => {}
    }
}
