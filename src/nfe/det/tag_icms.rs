use super::types::*;
use crate::system::{Gravis, GravisInterface};
use dfe::nfe::types::autorizacao4::Det;

// Recebe a variavel det_temp que é mutavel por referencia então
// ao ser preenchida aqui, ela será preenchida no objeto Det
pub fn icms_by_cst(det_temp: &mut Det, det: &DetApi) {
    match det_temp.icms.as_ref() {
        "ICMS00" => match det.cst.as_deref() {
            Some("00") => {
                if let (Some(mod_bc), Some(p_icms), Some(v_bc)) = (det.mod_bc, det.p_icms, det.v_bc)
                {
                    det_temp.mod_bc = Some(mod_bc);
                    det_temp.p_icms = Some(p_icms);
                    det_temp.v_bc = Some(v_bc);
                    det_temp.v_icms = Some(Gravis.calc_percent(v_bc, p_icms, 2));
                }
            }
            Some(cst) => {
                det_temp.cst = Some(format!("CST[{}] para ICMS00 inválido", cst));
            }
            None => {
                det_temp.cst = Some("ICMS CST não enviado".to_string());
            }
        },
        "ICMSSN101" => match det.csosn.as_deref() {
            Some("101") => {
                det_temp.p_cred_sn = det.p_cred_sn;
                det_temp.v_cred_icmssn = det.v_cred_icmssn;
            }
            Some(csosn) => {
                det_temp.csosn = Some(format!("CSOSN[{}] para ICMSSN101 inválido", csosn));
            }
            None => {
                det_temp.csosn = Some("ICMSSN CSOSN não enviado".to_string());
            }
        },
        "ICMSSN102" => match det.csosn.as_deref() {
            Some("102") => {
                // ICMSSN102 não possui campos adicionais a serem preenchidos
            }
            Some("103") => {
                // ICMSSN103 não possui campos adicionais a serem preenchidos
            }
            Some("300") => {
                // ICMSSN300 não possui campos adicionais a serem preenchidos
            }
            Some("400") => {
                // ICMSSN400 não possui campos adicionais a serem preenchidos
            }
            Some(csosn) => {
                det_temp.csosn = Some(format!("CSOSN[{}] para ICMSSN102 inválido", csosn));
            }
            None => {
                det_temp.csosn = Some("ICMSSN CSOSN não enviado".to_string());
            }
        },
        // Adicione outros ICMS tipos aqui conforme necessário
        _ => {}
    }
}
