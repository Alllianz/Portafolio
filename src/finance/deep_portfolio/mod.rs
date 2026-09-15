pub mod types;
pub mod features;
pub mod nn;
pub mod allocation;
pub mod evaluation;
pub mod audit;

pub use types::{
    AuditVerificationReport, DeepPortfolioConfig, DeepPortfolioExecutionResult, PortfolioConstraints,
    StepAllocation,
};
pub use evaluation::WalkForwardEngine;

/// Orquestador principal del módulo experimental de Red Neuronal para Asignación de Portafolios
pub struct DeepPortfolioService;

impl DeepPortfolioService {
    /// Ejecuta el proceso de asignación dinámica de red neuronal sobre datos de mercado
    pub fn run_experiment(
        tickers: &[String],
        prices_matrix: &[Vec<f64>],
        benchmark_prices: &[f64],
        time_labels: &[String],
        config: &DeepPortfolioConfig,
    ) -> Result<DeepPortfolioExecutionResult, String> {
        WalkForwardEngine::execute_simulation(
            tickers,
            prices_matrix,
            benchmark_prices,
            time_labels,
            config,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finance::deep_portfolio::allocation::TopKBoundedAllocator;
    use crate::finance::deep_portfolio::features::WelfordOnlineScaler;

    #[test]
    fn test_topk_allocation_constraints() {
        let constraints = PortfolioConstraints {
            max_cardinality: 5,
            min_asset_weight: 0.05,
            max_asset_weight: 1.00,
            transaction_fee_bps: 10.0,
            quadratic_impact_bps: 2.0,
            allow_cash: false,
        };

        // Generar 10 activos con logits arbitrarios
        let logits = vec![2.5, -1.0, 0.4, 3.2, 1.8, -0.5, 4.1, 0.0, 1.1, -2.0];
        let (weights, active_indices) = TopKBoundedAllocator::allocate(&logits, &constraints, 1.0);

        // 1. Cardinalidad <= 5
        let active_count = weights.iter().filter(|&&w| w > 1e-6).count();
        assert!(active_count <= 5, "Debe tener a lo sumo 5 activos seleccionados");
        assert_eq!(active_indices.len(), active_count);

        // 2. Piso mínimo >= 5%
        for &w in &weights {
            if w > 1e-6 {
                assert!(w >= 0.049999, "Cada activo seleccionado debe tener al menos 5% de peso");
            }
        }

        // 3. Suma = 1.0 exacto
        let sum_w: f64 = weights.iter().sum();
        assert!((sum_w - 1.0).abs() < 1e-7, "La suma de ponderaciones debe ser 100%");
    }

    #[test]
    fn test_welford_online_scaler_no_lookahead() {
        let mut scaler = WelfordOnlineScaler::new();
        let stream = vec![10.0, 12.0, 14.0, 11.0, 13.0];

        let mut transformed = Vec::new();
        for &val in &stream {
            let z = scaler.update_and_transform(val);
            transformed.push(z);
        }

        assert_eq!(transformed.len(), 5);
        // La media calculada al final debe ser 12.0
        assert!((scaler.mean() - 12.0).abs() < 1e-6);
    }

    #[test]
    fn test_ledoit_wolf_shrinkage_symmetry_and_posdef() {
        use crate::finance::deep_portfolio::features::LedoitWolfEstimator;

        let rets = vec![
            vec![0.01, -0.02, 0.015, 0.03, -0.01, 0.02, -0.005],
            vec![0.005, -0.015, 0.02, 0.025, -0.008, 0.015, -0.002],
            vec![-0.01, 0.02, -0.01, -0.015, 0.025, -0.01, 0.01],
        ];

        let cov = LedoitWolfEstimator::estimate_covariance(&rets);
        assert_eq!(cov.len(), 3);
        assert_eq!(cov[0].len(), 3);

        // Simetría
        for i in 0..3 {
            for j in 0..3 {
                assert!((cov[i][j] - cov[j][i]).abs() < 1e-10);
            }
            // Varianza positiva
            assert!(cov[i][i] > 0.0);
        }
    }

    #[test]
    fn test_friction_and_drift_calculation() {
        use crate::finance::deep_portfolio::allocation::FrictionModel;

        let prev_weights = vec![0.5, 0.5];
        let step_rets = vec![0.10, -0.10]; // El primer activo sube 10%, el segundo cae 10%

        let drifted = FrictionModel::calculate_drifted_weights(&prev_weights, &step_rets);
        assert!(drifted[0] > drifted[1], "El activo con retorno positivo debe aumentar su ponderación por drift");
        assert!(((drifted[0] + drifted[1]) - 1.0).abs() < 1e-10);

        let target_weights = vec![0.5, 0.5];
        let turnover = FrictionModel::calculate_turnover(&target_weights, &drifted);
        assert!(turnover > 0.0);
    }

    #[test]
    fn test_exact_100pct_determinism() {
        let tickers = vec!["AAPL".to_string(), "YPF".to_string(), "AMZN".to_string(), "NVDA".to_string()];
        let t_len = 100;
        let mut p1 = vec![100.0; t_len];
        let mut p2 = vec![100.0; t_len];
        let mut p3 = vec![100.0; t_len];
        let mut p4 = vec![100.0; t_len];
        let mut bm = vec![100.0; t_len];
        let mut dates = Vec::new();

        for i in 1..t_len {
            p1[i] = p1[i - 1] * (1.0 + (i as f64 * 0.001).sin() * 0.02);
            p2[i] = p2[i - 1] * (1.0 + (i as f64 * 0.0015).cos() * 0.025);
            p3[i] = p3[i - 1] * (1.0 + (i as f64 * 0.002).sin() * 0.018);
            p4[i] = p4[i - 1] * (1.0 + (i as f64 * 0.0008).cos() * 0.03);
            bm[i] = bm[i - 1] * (1.0 + (i as f64 * 0.0012).sin() * 0.012);
            dates.push(format!("2024-01-{:02}", (i % 28) + 1));
        }
        dates.insert(0, "2024-01-01".to_string());

        let prices_matrix = vec![p1, p2, p3, p4];
        let config = DeepPortfolioConfig {
            lookback_window: 30,
            rebalance_freq: 10,
            seed: 42,
            ..Default::default()
        };

        let run1 = DeepPortfolioService::run_experiment(&tickers, &prices_matrix, &bm, &dates, &config).unwrap();
        let run2 = DeepPortfolioService::run_experiment(&tickers, &prices_matrix, &bm, &dates, &config).unwrap();

        assert_eq!(run1.portfolio_equity_curve.len(), run2.portfolio_equity_curve.len());
        for i in 0..run1.portfolio_equity_curve.len() {
            assert_eq!(run1.portfolio_equity_curve[i], run2.portfolio_equity_curve[i], "Los valores de equity deben ser idénticos");
        }
        assert_eq!(run1.total_net_gain_pct, run2.total_net_gain_pct);
        assert_eq!(run1.sharpe_ratio, run2.sharpe_ratio);
        assert_eq!(run1.allocations_history.len(), run2.allocations_history.len());
        for k in 0..run1.allocations_history.len() {
            assert_eq!(run1.allocations_history[k].active_tickers, run2.allocations_history[k].active_tickers);
            assert_eq!(run1.allocations_history[k].active_weights, run2.allocations_history[k].active_weights);
        }
    }
}
