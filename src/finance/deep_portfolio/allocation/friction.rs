use crate::finance::deep_portfolio::types::PortfolioConstraints;

/// Modelador de fricciones de mercado, drift de precios y costos de rotación (Turnover)
#[derive(Debug, Clone)]
pub struct FrictionModel;

impl FrictionModel {
    /// Calcula los pesos derivados post-movimiento de mercado (Market Drift)
    /// w_i^- = (w_{i, t-1} * (1 + r_{i, t})) / sum_j (w_{j, t-1} * (1 + r_{j, t}))
    pub fn calculate_drifted_weights(prev_weights: &[f64], step_returns: &[f64]) -> Vec<f64> {
        let n = prev_weights.len();
        if n == 0 || step_returns.len() < n {
            return prev_weights.to_vec();
        }

        let mut evolved_values = vec![0.0; n];
        let mut total_value = 0.0;

        for i in 0..n {
            let r = step_returns[i];
            let val = prev_weights[i] * (1.0 + r).max(0.0001);
            evolved_values[i] = val;
            total_value += val;
        }

        if total_value <= 1e-12 {
            return prev_weights.to_vec();
        }

        evolved_values.iter().map(|&v| v / total_value).collect()
    }

    /// Calcula la tasa de rotación de una vía (One-way Turnover)
    /// Turnover = 0.5 * sum |w_target - w_drifted|
    pub fn calculate_turnover(target_weights: &[f64], drifted_weights: &[f64]) -> f64 {
        let n = target_weights.len().min(drifted_weights.len());
        if n == 0 {
            return 0.0;
        }

        let mut l1_dist = 0.0;
        for i in 0..n {
            l1_dist += (target_weights[i] - drifted_weights[i]).abs();
        }

        0.5 * l1_dist
    }

    /// Deduce los costos totales de transacción por rebalanceo
    /// Costo = c_broker * Turnover + gamma_slippage * sum (w_target - w_drifted)^2
    pub fn calculate_transaction_costs(
        target_weights: &[f64],
        drifted_weights: &[f64],
        constraints: &PortfolioConstraints,
    ) -> f64 {
        let n = target_weights.len().min(drifted_weights.len());
        if n == 0 {
            return 0.0;
        }

        let fee_rate = constraints.transaction_fee_bps / 10000.0; // bps a decimal
        let slippage_rate = constraints.quadratic_impact_bps / 10000.0;

        let mut linear_cost = 0.0;
        let mut quad_cost = 0.0;

        for i in 0..n {
            let diff = (target_weights[i] - drifted_weights[i]).abs();
            linear_cost += fee_rate * diff;
            quad_cost += slippage_rate * diff.powi(2);
        }

        linear_cost + quad_cost
    }
}
