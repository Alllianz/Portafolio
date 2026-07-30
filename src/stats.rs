use nalgebra::{DMatrix, DVector};

/// Calcula los retornos diarios de una serie de precios.
/// Similar a pct_change().dropna() en Pandas.
pub fn calcular_retornos_diarios(precios: &DMatrix<f64>) -> DMatrix<f64> {
    let rows = precios.nrows();
    let cols = precios.ncols();
    
    // Si no hay suficientes filas, retornamos matriz vacía
    if rows < 2 {
        return DMatrix::zeros(0, cols);
    }
    
    let mut retornos = DMatrix::zeros(rows - 1, cols);
    for c in 0..cols {
        for r in 1..rows {
            let actual = precios[(r, c)];
            let anterior = precios[(r - 1, c)];
            retornos[(r - 1, c)] = (actual / anterior) - 1.0;
        }
    }
    retornos
}

/// Calcula el retorno esperado (media de los retornos) por cada activo.
pub fn calcular_retorno_esperado(retornos: &DMatrix<f64>) -> DVector<f64> {
    let mut medias = DVector::zeros(retornos.ncols());
    for c in 0..retornos.ncols() {
        let col = retornos.column(c);
        medias[c] = col.mean();
    }
    medias
}

/// Calcula el riesgo (volatilidad) de cada activo.
/// `poblacional`: Si es false, usa ddof=1 (muestral), si es true usa ddof=0 (poblacional).
pub fn calcular_riesgo_volatilidad(retornos: &DMatrix<f64>, poblacional: bool) -> DVector<f64> {
    let n = retornos.nrows() as f64;
    let divisor = if poblacional { n } else { n - 1.0 };
    
    let mut volatilidades = DVector::zeros(retornos.ncols());
    for c in 0..retornos.ncols() {
        let col = retornos.column(c);
        let mean = col.mean();
        let mut suma_cuadrados = 0.0;
        for r in 0..retornos.nrows() {
            let diff = col[r] - mean;
            suma_cuadrados += diff * diff;
        }
        volatilidades[c] = (suma_cuadrados / divisor).sqrt();
    }
    volatilidades
}

/// Calcula la matriz de covarianza de los retornos.
pub fn calcular_matriz_covarianza(retornos: &DMatrix<f64>, poblacional: bool) -> DMatrix<f64> {
    let n = retornos.nrows() as f64;
    let divisor = if poblacional { n } else { n - 1.0 };
    let cols = retornos.ncols();
    
    let medias = calcular_retorno_esperado(retornos);
    
    // Centrar los retornos
    let mut centrados = retornos.clone();
    for c in 0..cols {
        let m = medias[c];
        for r in 0..retornos.nrows() {
            centrados[(r, c)] -= m;
        }
    }
    
    // Covarianza = (X^T * X) / divisor
    let cov_matrix = (centrados.transpose() * centrados) / divisor;
    cov_matrix
}

/// Calcula el Beta de los activos respecto a un mercado (SPY, por ejemplo).
pub fn calcular_betas(retornos: &DMatrix<f64>, serie_mercado: &DVector<f64>, poblacional: bool) -> DVector<f64> {
    let n = retornos.nrows() as f64;
    let divisor = if poblacional { n } else { n - 1.0 };
    
    let media_mercado = serie_mercado.mean();
    let mut var_mercado = 0.0;
    let mut mercado_centrado = DVector::zeros(serie_mercado.len());
    
    for i in 0..serie_mercado.len() {
        let diff = serie_mercado[i] - media_mercado;
        mercado_centrado[i] = diff;
        var_mercado += diff * diff;
    }
    var_mercado /= divisor;
    
    let cols = retornos.ncols();
    let mut betas = DVector::zeros(cols);
    let medias = calcular_retorno_esperado(retornos);
    
    for c in 0..cols {
        let mut cov_activo_mercado = 0.0;
        for r in 0..retornos.nrows() {
            cov_activo_mercado += (retornos[(r, c)] - medias[c]) * mercado_centrado[r];
        }
        cov_activo_mercado /= divisor;
        betas[c] = cov_activo_mercado / var_mercado;
    }
    
    betas
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::{dmatrix, dvector};

    #[test]
    fn test_retornos_diarios() {
        let precios = dmatrix![
            100.0, 50.0;
            110.0, 45.0;
            121.0, 40.5
        ];
        let retornos = calcular_retornos_diarios(&precios);
        assert!((retornos[(0, 0)] - 0.1).abs() < 1e-6);
        assert!((retornos[(0, 1)] + 0.1).abs() < 1e-6);
    }
}
