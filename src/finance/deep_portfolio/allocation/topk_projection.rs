use crate::finance::deep_portfolio::types::PortfolioConstraints;

/// Asignador de portafolio con restricciones duras
/// Garantiza:
/// 1. A lo sumo K activos seleccionados (K <= 5).
/// 2. Si un activo es seleccionado, w_i >= min_weight (5%).
/// 3. sum(w_i) = 1.0 (100% invertido, o con caja si se habilita).
#[derive(Debug, Clone)]
pub struct TopKBoundedAllocator;

impl TopKBoundedAllocator {
    /// Proyecta logits de la red neuronal sobre las restricciones de portafolio
    pub fn allocate(
        logits: &[f64],
        constraints: &PortfolioConstraints,
        temperature: f64,
    ) -> (Vec<f64>, Vec<usize>) {
        let n = logits.len();
        if n == 0 {
            return (Vec::new(), Vec::new());
        }

        let max_k = constraints.max_cardinality.min(n).max(1);
        let min_w = constraints.min_asset_weight.clamp(0.0, 0.20); // 5%

        // 1. Obtener pares (índice, logit) y ordenar de mayor a menor score con desempate determinista
        let mut indexed_logits: Vec<(usize, f64)> = logits.iter().cloned().enumerate().collect();
        indexed_logits.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.0.cmp(&b.0))
        });

        // 2. Seleccionar los Top K activos
        let top_k_items: Vec<(usize, f64)> = indexed_logits.into_iter().take(max_k).collect();
        let k_active = top_k_items.len();
        let active_indices: Vec<usize> = top_k_items.iter().map(|(idx, _)| *idx).collect();

        // 3. Asignación con piso mínimo de 5% y redistribución por Softmax
        let mut weights = vec![0.0; n];
        if k_active == 0 {
            return (weights, active_indices);
        }

        // Base mínima garantizada por activo activo
        let base_weight_total = (k_active as f64) * min_w;
        let remaining_budget = (1.0 - base_weight_total).max(0.0);

        // Softmax sobre los logits de los Top K
        let top_logits: Vec<f64> = top_k_items.iter().map(|(_, l)| *l).collect();
        let max_logit = top_logits.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let exp_vals: Vec<f64> = top_logits
            .iter()
            .map(|&l| ((l - max_logit) / temperature.max(0.05)).exp())
            .collect();
        let sum_exp: f64 = exp_vals.iter().sum::<f64>().max(1e-12);

        for (i, &(idx, _)) in top_k_items.iter().enumerate() {
            let softmax_prop = exp_vals[i] / sum_exp;
            // w_i = min_weight + remaining_budget * softmax_prop
            let w = min_w + remaining_budget * softmax_prop;
            weights[idx] = w.clamp(min_w, constraints.max_asset_weight);
        }

        // 4. Normalización estricta para garantizar sum(w) = 1.0000000000 exacto
        let sum_w: f64 = weights.iter().sum();
        if sum_w > 0.0 {
            for w in weights.iter_mut() {
                if *w > 0.0 {
                    *w /= sum_w;
                }
            }
        }

        (weights, active_indices)
    }
}
