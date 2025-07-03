use super::types::*;
use crate::system::{Gravis, GravisInterface};
use dfe::nfe::types::autorizacao4::Det;

pub fn cofins_by_cst(det_temp: &mut Det, det: &DetApi) {
    match det.cofins.as_str() {
        "COFINSAliq" => {
            match det.cofins_cst.as_deref() {
                Some("01") => {
                    det_temp.cofins_cst = Some("01".to_string());
                    if let (Some(cofins_v_bc), Some(cofins_p_cofins)) =
                        (det.cofins_v_bc, det.cofins_p_cofins)
                    {
                        det_temp.cofins_v_bc = Some(cofins_v_bc);
                        det_temp.cofins_p_cofins = Some(cofins_p_cofins);
                        det_temp.cofins_v_cofins =
                            Some(Gravis.calc_percent(cofins_v_bc, cofins_p_cofins, 2));
                    }
                }
                Some("02") => {
                    det_temp.cofins_cst = Some("02".to_string());
                    // Lógica para CST 02 pode ser adicionada aqui
                }
                Some(cst) => {
                    det_temp.cofins_cst = Some(format!("{} inválido", cst));
                }
                None => {
                    det_temp.cofins_cst = Some("COFINS CST não enviado".to_string());
                }
            }
        }
        // Adicione outros COFINS CSTs aqui conforme necessário
        _ => {}
    }
}
