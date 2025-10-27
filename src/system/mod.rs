use rust_decimal::prelude::FromPrimitive;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

pub struct Gravis;

impl Gravis {
    /// Arredondamento para o número de casas decimais especificado usando a regra "bankers rounding":
    /// - Se o dígito seguinte for menor que 5, arredonda para baixo.
    /// - Se for maior que 5, arredonda para cima.
    /// - Se for exatamente 5, arredonda para o número par mais próximo (arredondamento bancário).
    pub fn calc_percent(&self, value: f64, percent: f64, decimals: u32) -> f64 {
        let value_dec = Decimal::from_f64(value).unwrap_or(Decimal::ZERO);
        let percent_dec = Decimal::from_f64(percent).unwrap_or(Decimal::ZERO);
        let result = value_dec * (percent_dec / Decimal::new(100, 0));
        match result.round_dp(decimals).to_f64() {
            Some(val) => val,
            None => {
                eprintln!(
                    "Erro ao executar Gravis.calc_percent({}, {}, {}).",
                    value, percent, decimals
                );
                0.0
            }
        }
    }
}
