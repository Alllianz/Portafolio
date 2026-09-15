/// Métricas cuantitativas de evaluación financiera de series de retornos netos
#[derive(Debug, Clone)]
pub struct PortfolioMetricsCalculator;

impl PortfolioMetricsCalculator {
    /// Retorno anualizado geométrico (CAGR)
    pub fn annualized_return(equity_curve: &[f64]) -> f64 {
        if equity_curve.len() < 2 {
            return 0.0;
        }
        let first = equity_curve[0];
        let last = equity_curve[equity_curve.len() - 1];
        if first <= 0.0 || last <= 0.0 {
            return 0.0;
        }
        let total_gain = last / first;
        let years = (equity_curve.len() as f64) / 252.0;
        if years <= 0.0 {
            0.0
        } else {
            (total_gain.powf(1.0 / years) - 1.0) * 100.0
        }
    }

    /// Volatilidad anualizada de retornos netos
    pub fn annualized_volatility(returns: &[f64]) -> f64 {
        if returns.len() < 2 {
            return 0.0;
        }
        let n = returns.len() as f64;
        let mean = returns.iter().sum::<f64>() / n;
        let var = returns.iter().map(|&r| (r - mean).powi(2)).sum::<f64>() / (n - 1.0);
        var.max(0.0).sqrt() * 252.0_f64.sqrt() * 100.0
    }

    /// Ratio de Sharpe anualizado
    pub fn sharpe_ratio(returns: &[f64], rf_rate: f64) -> f64 {
        if returns.len() < 2 {
            return 0.0;
        }
        let mean_ret = (returns.iter().sum::<f64>() / (returns.len() as f64)) * 252.0;
        let vol = Self::annualized_volatility(returns) / 100.0;
        if vol <= 1e-8 {
            0.0
        } else {
            (mean_ret - rf_rate) / vol
        }
    }

    /// Ratio de Sortino anualizado
    pub fn sortino_ratio(returns: &[f64], rf_rate: f64) -> f64 {
        if returns.len() < 2 {
            return 0.0;
        }
        let rf_daily = rf_rate / 252.0;
        let mean_ret = (returns.iter().sum::<f64>() / (returns.len() as f64)) * 252.0;
        
        let sum_downside_sq: f64 = returns.iter()
            .map(|&r| {
                let diff = r - rf_daily;
                if diff < 0.0 { diff.powi(2) } else { 0.0 }
            })
            .sum();
        let dd = (sum_downside_sq / (returns.len() as f64)).sqrt() * 252.0_f64.sqrt();
        if dd <= 1e-8 {
            0.0
        } else {
            (mean_ret - rf_rate) / dd
        }
    }

    /// Máximo Drawdown porcentual
    pub fn max_drawdown(equity_curve: &[f64]) -> f64 {
        if equity_curve.is_empty() {
            return 0.0;
        }
        let mut peak = equity_curve[0];
        let mut max_dd = 0.0;

        for &val in equity_curve {
            if val > peak {
                peak = val;
            }
            let dd = if peak > 0.0 { (peak - val) / peak } else { 0.0 };
            if dd > max_dd {
                max_dd = dd;
            }
        }

        max_dd * 100.0
    }

    /// Ratio de Calmar (CAGR / MaxDD)
    pub fn calmar_ratio(annualized_ret: f64, max_dd_pct: f64) -> f64 {
        if max_dd_pct <= 0.01 {
            0.0
        } else {
            annualized_ret / max_dd_pct
        }
    }

    /// Beta de la cartera frente al Benchmark
    pub fn beta_vs_benchmark(port_returns: &[f64], bm_returns: &[f64]) -> f64 {
        let n = port_returns.len().min(bm_returns.len());
        if n < 5 {
            return 1.0;
        }
        let mean_p = port_returns[..n].iter().sum::<f64>() / (n as f64);
        let mean_b = bm_returns[..n].iter().sum::<f64>() / (n as f64);

        let mut cov = 0.0;
        let mut var_b = 0.0;
        for i in 0..n {
            let dp = port_returns[i] - mean_p;
            let db = bm_returns[i] - mean_b;
            cov += dp * db;
            var_b += db * db;
        }

        if var_b <= 1e-12 {
            1.0
        } else {
            cov / var_b
        }
    }

    /// Alpha anualizado de Jensen vs Benchmark
    pub fn alpha_annualized(port_ann_ret: f64, bm_ann_ret: f64, beta: f64, rf_rate_pct: f64) -> f64 {
        port_ann_ret - (rf_rate_pct + beta * (bm_ann_ret - rf_rate_pct))
    }

    /// Tasa de ruedas positivas (Win Rate)
    pub fn win_rate(returns: &[f64]) -> f64 {
        if returns.is_empty() {
            return 0.0;
        }
        let wins = returns.iter().filter(|&&r| r > 0.0).count();
        (wins as f64) / (returns.len() as f64) * 100.0
    }
}
