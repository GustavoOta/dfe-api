#[cfg(test)]
mod gravis {
    use crate::system::*;

    #[test]
    fn calc_percent() {
        let gravis = Gravis;
        let value = 200.0;
        let percent = 15.0;
        let decimals = 2;
        let result = gravis.calc_percent(value, percent, decimals);
        assert!(
            (result - 30.0).abs() < f64::EPSILON,
            "Esperado 30.0, obteve {}",
            result
        );
    }
}
