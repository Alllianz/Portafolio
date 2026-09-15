/// Extractor de métricas predictivas y de riesgo rodantes estrictamente causales (Lag t-1)
#[derive(Debug, Clone)]
pub struct RollingMetricsExtractor;

impl RollingMetricsExtractor {
    /// Calcula el momentum logarítmico a horizonte k sobre precios históricos
    pub fn momentum_k(prices: &[f64], end_idx: usize, k: usize) -> f64 {
        if end_idx == 0 || end_idx < k || prices.len() <= end_idx {
            return 0.0;
        }
        let p_curr = prices[end_idx];
        let p_prev = prices[end_idx - k];
        if p_prev <= 0.0 || p_curr <= 0.0 {
            0.0
        } else {
            (p_curr / p_prev).ln()
        }
    }

    /// Calcula la media móvil de retornos simples sobre ventana W
    pub fn rolling_mean_return(returns: &[f64], end_idx: usize, window: usize) -> f64 {
        if end_idx == 0 || returns.is_empty() {
            return 0.0;
        }
        let start = end_idx.saturating_sub(window);
        let slice = &returns[start..end_idx]; // Excluye end_idx si se evalúa a t-1
        if slice.is_empty() {
            return 0.0;
        }
        slice.iter().sum::<f64>() / (slice.len() as f64)
    }

    /// Calcula la volatilidad anualizada móvil
    pub fn rolling_volatility(returns: &[f64], end_idx: usize, window: usize) -> f64 {
        if end_idx < 2 || returns.is_empty() {
            return 0.01;
        }
        let start = end_idx.saturating_sub(window);
        let slice = &returns[start..end_idx];
        if slice.len() < 2 {
            return 0.01;
        }
        let mean = slice.iter().sum::<f64>() / (slice.len() as f64);
        let var = slice.iter().map(|&r| (r - mean).powi(2)).sum::<f64>() / ((slice.len() - 1) as f64);
        (var.max(1e-8).sqrt() * 252.0_f64.sqrt()).max(0.005)
    }

    /// Calcula la semi-desviación móvil (Downside Deviation / Riesgo a la baja)
    pub fn rolling_downside_deviation(returns: &[f64], end_idx: usize, window: usize, rf_daily: f64) -> f64 {
        if end_idx < 2 || returns.is_empty() {
            return 0.01;
        }
        let start = end_idx.saturating_sub(window);
        let slice = &returns[start..end_idx];
        if slice.is_empty() {
            return 0.01;
        }
        let sum_sq_down = slice.iter()
            .map(|&r| {
                let diff = r - rf_daily;
                if diff < 0.0 { diff.powi(2) } else { 0.0 }
            })
            .sum::<f64>();
        let dd = (sum_sq_down / (slice.len() as f64)).sqrt() * 252.0_f64.sqrt();
        dd.max(0.005)
    }

    /// Calcula el Ratio de Sharpe rodante anualizado
    pub fn rolling_sharpe(returns: &[f64], end_idx: usize, window: usize, rf_annual: f64) -> f64 {
        let mean_daily = Self::rolling_mean_return(returns, end_idx, window);
        let vol_annual = Self::rolling_volatility(returns, end_idx, window);
        let ann_ret = mean_daily * 252.0;
        (ann_ret - rf_annual) / (vol_annual + 1e-8)
    }

    /// Calcula el Ratio de Sortino rodante anualizado
    pub fn rolling_sortino(returns: &[f64], end_idx: usize, window: usize, rf_annual: f64) -> f64 {
        let rf_daily = rf_annual / 252.0;
        let mean_daily = Self::rolling_mean_return(returns, end_idx, window);
        let dd_annual = Self::rolling_downside_deviation(returns, end_idx, window, rf_daily);
        let ann_ret = mean_daily * 252.0;
        (ann_ret - rf_annual) / (dd_annual + 1e-8)
    }

    /// Genera el vector completo de D features para un activo en el paso t (estrictamente causal hasta t-1)
    pub fn extract_features_vector(
        prices: &[f64],
        returns: &[f64],
        current_step: usize,
        window: usize,
        rf_rate: f64,
    ) -> Vec<f64> {
        // En el paso current_step, solo tenemos cerrados los precios hasta current_step - 1
        let end_idx = current_step;
        if end_idx == 0 {
            return vec![0.0; 8];
        }

        let mom_5 = Self::momentum_k(prices, end_idx, 5);
        let mom_21 = Self::momentum_k(prices, end_idx, 21);
        let mom_63 = Self::momentum_k(prices, end_idx, 63);
        let mom_126 = Self::momentum_k(prices, end_idx, 126);
        let mean_ret = Self::rolling_mean_return(returns, end_idx, window);
        let vol = Self::rolling_volatility(returns, end_idx, window);
        let sharpe = Self::rolling_sharpe(returns, end_idx, window, rf_rate).clamp(-4.0, 4.0);
        let sortino = Self::rolling_sortino(returns, end_idx, window, rf_rate).clamp(-4.0, 4.0);

        vec![
            mom_5,
            mom_21,
            mom_63,
            mom_126,
            mean_ret * 252.0, // Retorno anualizado
            vol,
            sharpe,
            sortino,
        ]
    }
}
