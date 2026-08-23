use nalgebra::DVector;

/// Modelo CAPM (Capital Asset Pricing Model).
/// `rf_anual`: Tasa libre de riesgo anual.
/// `betas`: Vector de betas calculados contra el mercado.
/// `rm_diario`: Retorno medio diario del mercado.
/// `dias_anualizacion`: 252.0.
pub fn calcular_retorno_capm(
    rf_anual: f64,
    betas: &DVector<f64>,
    rm_diario: f64,
    dias_anualizacion: f64,
) -> DVector<f64> {
    let rf_diaria = rf_anual / dias_anualizacion;
    let mut retornos_capm = DVector::zeros(betas.len());
    
    for i in 0..betas.len() {
        let ret_diario = rf_diaria + betas[i] * (rm_diario - rf_diaria);
        retornos_capm[i] = ret_diario * dias_anualizacion;
    }
    
    retornos_capm
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::dvector;

    #[test]
    fn test_capm() {
        let betas = dvector![1.0, 1.5, 0.5];
        let rf_anual = 0.04;
        let rm_diario = 0.0005;
        let dias = 252.0;
        let rf_diaria = rf_anual / dias;
        
        let capm = calcular_retorno_capm(rf_anual, &betas, rm_diario, dias);
        
        // Beta 1.0 debe dar exactamente el retorno de mercado anualizado
        assert!((capm[0] - (rm_diario * dias)).abs() < 1e-8);
        
        // Beta 1.5
        let esperado_1_5 = (rf_diaria + 1.5 * (rm_diario - rf_diaria)) * dias;
        assert!((capm[1] - esperado_1_5).abs() < 1e-8);
    }
}
