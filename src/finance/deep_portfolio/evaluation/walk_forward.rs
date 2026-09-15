use std::collections::HashMap;
use crate::finance::deep_portfolio::allocation::{FrictionModel, TopKBoundedAllocator};
use crate::finance::deep_portfolio::audit::PortfolioAuditVerifier;
use crate::finance::deep_portfolio::evaluation::metrics::PortfolioMetricsCalculator;
use crate::finance::deep_portfolio::features::{RollingMetricsExtractor, WelfordOnlineScaler};
use crate::finance::deep_portfolio::nn::DeepPortfolioNet;
use crate::finance::deep_portfolio::types::{DeepPortfolioConfig, DeepPortfolioExecutionResult, StepAllocation};

/// Motor de simulación Walk-Forward Causal (Cero Look-Ahead Bias)
pub struct WalkForwardEngine;

impl WalkForwardEngine {
    /// Ejecuta la simulación cronológica completa con inferencia neuronal y rebalanceo periódico
    pub fn execute_simulation(
        tickers: &[String],
        prices_matrix: &[Vec<f64>],
        benchmark_prices: &[f64],
        time_labels: &[String],
        config: &DeepPortfolioConfig,
    ) -> Result<DeepPortfolioExecutionResult, String> {
        let n_assets = tickers.len();
        if n_assets == 0 {
            return Err("El universo de activos no puede estar vacío.".to_string());
        }
        let total_timesteps = prices_matrix[0].len();
        if total_timesteps <= config.lookback_window + 10 {
            return Err(format!(
                "La serie histórica ({} ruedas) es insuficiente para la ventana causal de {} ruedas.",
                total_timesteps, config.lookback_window
            ));
        }

        // 1. Calcular retornos diarios simples de cada activo y del benchmark
        let mut returns_matrix = vec![vec![0.0; total_timesteps]; n_assets];
        for i in 0..n_assets {
            for t in 1..total_timesteps {
                let p_prev = prices_matrix[i][t - 1];
                let p_curr = prices_matrix[i][t];
                returns_matrix[i][t] = if p_prev > 0.0 { (p_curr - p_prev) / p_prev } else { 0.0 };
            }
        }

        let mut bm_returns = vec![0.0; total_timesteps];
        for t in 1..total_timesteps.min(benchmark_prices.len()) {
            let p_prev = benchmark_prices[t - 1];
            let p_curr = benchmark_prices[t];
            bm_returns[t] = if p_prev > 0.0 { (p_curr - p_prev) / p_prev } else { 0.0 };
        }

        // 2. Inicializar Red Neuronal e inferencia
        let feature_dim = 8; // mom5, mom21, mom63, mom126, ann_mean, vol, sharpe, sortino
        let net = DeepPortfolioNet::new(
            feature_dim,
            config.latent_dim,
            config.num_attention_heads,
            config.seed,
        );

        // Inicializar normalizadores causales Welford (uno por feature)
        let mut feature_scalers = vec![WelfordOnlineScaler::new(); feature_dim];

        // 3. Variables de estado de simulación
        let mut port_equity = 100.0;
        let mut bm_equity = 100.0;
        let mut eq_equity = 100.0;

        let mut port_equity_curve = Vec::new();
        let mut bm_equity_curve = Vec::new();
        let mut eq_equity_curve = Vec::new();
        let mut sim_time_labels = Vec::new();
        let mut net_returns = Vec::new();
        let mut sim_bm_returns = Vec::new();
        let mut step_allocations = Vec::new();
        let mut weights_matrix_history = Vec::new();
        let mut active_selection_counts: HashMap<String, usize> = HashMap::new();

        for t in tickers {
            active_selection_counts.insert(t.clone(), 0);
        }

        // Pesos iniciales equiponderados 1/N
        let mut current_weights = vec![1.0 / (n_assets as f64); n_assets];

        let start_step = config.lookback_window;

        // 4. Bucle temporal cronológico Walk-Forward (Paso a paso)
        for t in start_step..total_timesteps {
            let is_rebalance_step = (t - start_step) % config.rebalance_freq == 0;
            let mut turnover = 0.0;
            let mut tx_cost_pct = 0.0;

            if is_rebalance_step {
                // a. Extracción de features estrictamente causales hasta t-1
                let mut raw_features_matrix = Vec::with_capacity(n_assets);
                for i in 0..n_assets {
                    let feat_vec = RollingMetricsExtractor::extract_features_vector(
                        &prices_matrix[i],
                        &returns_matrix[i],
                        t,
                        config.lookback_window,
                        0.04,
                    );
                    raw_features_matrix.push(feat_vec);
                }

                // b. Normalización causal online
                let mut normalized_features = vec![vec![0.0; feature_dim]; n_assets];
                for i in 0..n_assets {
                    for f in 0..feature_dim {
                        let val = raw_features_matrix[i][f];
                        let norm_val = feature_scalers[f].update_and_transform(val);
                        normalized_features[i][f] = norm_val.clamp(-4.0, 4.0);
                    }
                }

                // c. Inferencia Neuronal: Forward pass por CSAN
                let logits = net.forward(&normalized_features);

                // d. Proyección de asignación con restricciones duras (Top 5, min 5%, sum 100%)
                let (target_weights, active_indices) = TopKBoundedAllocator::allocate(
                    &logits,
                    &config.constraints,
                    config.temperature,
                );

                // e. Drift previo de mercado y cálculo de costos de rebalanceo
                let step_rets_prev: Vec<f64> = (0..n_assets).map(|i| returns_matrix[i][t.saturating_sub(1)]).collect();
                let drifted_weights = FrictionModel::calculate_drifted_weights(&current_weights, &step_rets_prev);
                
                turnover = FrictionModel::calculate_turnover(&target_weights, &drifted_weights);
                tx_cost_pct = FrictionModel::calculate_transaction_costs(&target_weights, &drifted_weights, &config.constraints);

                current_weights = target_weights;

                // Contabilizar selecciones
                for &idx in &active_indices {
                    if let Some(t_name) = tickers.get(idx) {
                        *active_selection_counts.entry(t_name.clone()).or_insert(0) += 1;
                    }
                }
            } else {
                // Día intra-rebalanceo: los pesos evolucionan por drift de mercado
                let step_rets: Vec<f64> = (0..n_assets).map(|i| returns_matrix[i][t]).collect();
                current_weights = FrictionModel::calculate_drifted_weights(&current_weights, &step_rets);
            }

            // f. Retorno del período t
            let mut gross_step_ret = 0.0;
            let mut eq_step_ret = 0.0;
            for i in 0..n_assets {
                let r = returns_matrix[i][t];
                gross_step_ret += current_weights[i] * r;
                eq_step_ret += (1.0 / (n_assets as f64)) * r;
            }

            let net_step_ret = gross_step_ret - tx_cost_pct;
            let bm_step_ret = bm_returns[t];

            // Actualizar curves de equity
            port_equity *= (1.0 + net_step_ret).max(0.001);
            bm_equity *= (1.0 + bm_step_ret).max(0.001);
            eq_equity *= (1.0 + eq_step_ret).max(0.001);

            port_equity_curve.push(port_equity);
            bm_equity_curve.push(bm_equity);
            eq_equity_curve.push(eq_equity);
            sim_time_labels.push(if t < time_labels.len() { time_labels[t].clone() } else { format!("T_{}", t) });
            net_returns.push(net_step_ret);
            sim_bm_returns.push(bm_step_ret);
            weights_matrix_history.push(current_weights.clone());

            // Registro de asignación para auditoría
            if is_rebalance_step {
                let mut active_tickers = Vec::new();
                let mut active_weights = Vec::new();
                for (idx, &w) in current_weights.iter().enumerate() {
                    if w > 0.001 {
                        if let Some(name) = tickers.get(idx) {
                            active_tickers.push(name.clone());
                            active_weights.push(w * 100.0);
                        }
                    }
                }

                step_allocations.push(StepAllocation {
                    date: if t < time_labels.len() { time_labels[t].clone() } else { format!("Rueda_{}", t) },
                    step_index: t,
                    weights: current_weights.iter().map(|&w| w * 100.0).collect(),
                    active_tickers,
                    active_weights,
                    turnover: turnover * 100.0,
                    transaction_cost_pct: tx_cost_pct * 100.0,
                    gross_return: gross_step_ret * 100.0,
                    net_return: net_step_ret * 100.0,
                    portfolio_equity: port_equity,
                    benchmark_equity: bm_equity,
                    equal_weight_equity: eq_equity,
                });
            }
        }

        // 5. Cálculo de métricas cuantitativas Out-Of-Sample
        let total_net_gain = if !port_equity_curve.is_empty() { (port_equity_curve.last().unwrap() / 100.0 - 1.0) * 100.0 } else { 0.0 };
        let bm_gain = if !bm_equity_curve.is_empty() { (bm_equity_curve.last().unwrap() / 100.0 - 1.0) * 100.0 } else { 0.0 };
        let eq_gain = if !eq_equity_curve.is_empty() { (eq_equity_curve.last().unwrap() / 100.0 - 1.0) * 100.0 } else { 0.0 };

        let ann_ret = PortfolioMetricsCalculator::annualized_return(&port_equity_curve);
        let bm_ann_ret = PortfolioMetricsCalculator::annualized_return(&bm_equity_curve);
        let ann_vol = PortfolioMetricsCalculator::annualized_volatility(&net_returns);
        let sharpe = PortfolioMetricsCalculator::sharpe_ratio(&net_returns, 0.04);
        let sortino = PortfolioMetricsCalculator::sortino_ratio(&net_returns, 0.04);
        let mdd = PortfolioMetricsCalculator::max_drawdown(&port_equity_curve);
        let bm_mdd = PortfolioMetricsCalculator::max_drawdown(&bm_equity_curve);
        let calmar = PortfolioMetricsCalculator::calmar_ratio(ann_ret, mdd);
        let beta = PortfolioMetricsCalculator::beta_vs_benchmark(&net_returns, &sim_bm_returns);
        let alpha = PortfolioMetricsCalculator::alpha_annualized(ann_ret, bm_ann_ret, beta, 4.0);
        let win_rate = PortfolioMetricsCalculator::win_rate(&net_returns);

        // 6. Auditoría formal de invariantes
        let audit_report = PortfolioAuditVerifier::audit_execution(&step_allocations, &config.constraints);

        Ok(DeepPortfolioExecutionResult {
            success: true,
            message: "Simulación de red neuronal causal completada con éxito.".to_string(),
            tickers: tickers.to_vec(),
            time_labels: sim_time_labels,
            portfolio_equity_curve: port_equity_curve,
            benchmark_equity_curve: bm_equity_curve,
            equal_weight_equity_curve: eq_equity_curve,
            net_returns,
            benchmark_returns: sim_bm_returns,
            allocations_history: step_allocations,
            weights_matrix_history,
            active_selection_counts,
            total_net_gain_pct: total_net_gain,
            benchmark_gain_pct: bm_gain,
            equal_weight_gain_pct: eq_gain,
            alpha_annualized_pct: alpha,
            beta_vs_benchmark: beta,
            annualized_net_return_pct: ann_ret,
            annualized_volatility_pct: ann_vol,
            sharpe_ratio: sharpe,
            sortino_ratio: sortino,
            calmar_ratio: calmar,
            max_drawdown_pct: mdd,
            benchmark_max_drawdown_pct: bm_mdd,
            win_rate_pct: win_rate,
            audit_report,
        })
    }
}
