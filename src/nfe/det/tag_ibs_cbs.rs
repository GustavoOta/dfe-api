use super::types::*;
use dfe::nfe::types::autorizacao4::Det;
use rust_decimal::Decimal;

pub struct IbsCbs;
impl IbsCbs {
    pub fn build(det_temp: &mut Det, det: &DetApi) {
        // Build IBSCBS specific fields here
        /*
        pub ibs_cbs_cst: String, // N3 EX: "000"
        pub ibs_cbs_class_trib: String, //N6 EX: "000001"
        pub ibs_cbs_v_bc: Decimal,      // EX: 100.00
        pub p_ibs_uf: Decimal,          // EX: 0.90
        pub p_ibs_mun: Decimal,         // EX: 0.00
        pub p_cbs: Decimal,             // EX: 0.10
        */
        det_temp.ibs_cbs_cst = det.ibs_cbs_cst.clone();
        det_temp.ibs_cbs_class_trib = det.ibs_cbs_class_trib.clone();
        det_temp.ibs_cbs_v_bc = det.ibs_cbs_v_bc;
        det_temp.p_ibs_uf = det.p_ibs_uf;
        det_temp.p_ibs_mun = det.p_ibs_mun;
        det_temp.p_cbs = det.p_cbs;

        let hundred = Decimal::new(100, 0);
        det_temp.v_ibs_uf = (det.ibs_cbs_v_bc * det.p_ibs_uf / hundred).round_dp(2);
        det_temp.v_ibs_mun = (det.ibs_cbs_v_bc * det.p_ibs_mun / hundred).round_dp(2);
        det_temp.v_cbs = (det.ibs_cbs_v_bc * det.p_cbs / hundred).round_dp(2);
    }
}
