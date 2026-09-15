use super::layers::{softmax, DenseLayer, LayerNorm};

/// Módulo de Atención Transversal Multi-Cabeza (Cross-Sectional Multi-Head Self-Attention)
/// Permite que cada activo compare sus métricas de riesgo/retorno contra el resto del universo.
#[derive(Debug, Clone)]
pub struct CrossSectionalAttention {
    pub dim: usize,
    pub num_heads: usize,
    pub head_dim: usize,
    pub w_q: DenseLayer,
    pub w_k: DenseLayer,
    pub w_v: DenseLayer,
    pub w_out: DenseLayer,
    pub layer_norm: LayerNorm,
}

impl CrossSectionalAttention {
    pub fn new(dim: usize, num_heads: usize, seed: u64) -> Self {
        let head_dim = (dim / num_heads.max(1)).max(1);
        let eff_dim = head_dim * num_heads;

        Self {
            dim: eff_dim,
            num_heads,
            head_dim,
            w_q: DenseLayer::new(eff_dim, eff_dim, seed),
            w_k: DenseLayer::new(eff_dim, eff_dim, seed + 10),
            w_v: DenseLayer::new(eff_dim, eff_dim, seed + 20),
            w_out: DenseLayer::new(eff_dim, eff_dim, seed + 30),
            layer_norm: LayerNorm::new(eff_dim),
        }
    }

    /// Forward pass de atención transversal sobre N activos
    /// asset_embeddings: [N][dim]
    /// Retorna: [N][dim] con información contextualizada de correlación y dominancia
    pub fn forward(&self, asset_embeddings: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let n_assets = asset_embeddings.len();
        if n_assets == 0 {
            return Vec::new();
        }

        // 1. Proyecciones lineales Q, K, V
        let q_mat: Vec<Vec<f64>> = asset_embeddings.iter().map(|h| self.w_q.forward(h)).collect();
        let k_mat: Vec<Vec<f64>> = asset_embeddings.iter().map(|h| self.w_k.forward(h)).collect();
        let v_mat: Vec<Vec<f64>> = asset_embeddings.iter().map(|h| self.w_v.forward(h)).collect();

        let scale = (self.head_dim as f64).sqrt();
        let mut head_outputs = vec![vec![0.0; self.dim]; n_assets];

        // 2. Procesamiento por cada cabeza de atención
        for h in 0..self.num_heads {
            let h_start = h * self.head_dim;
            let h_end = h_start + self.head_dim;

            // Matriz de atención [N][N] para la cabeza h
            let mut attn_weights = vec![vec![0.0; n_assets]; n_assets];
            for i in 0..n_assets {
                let mut logits = vec![0.0; n_assets];
                for j in 0..n_assets {
                    let mut dot = 0.0;
                    for d in h_start..h_end {
                        let q_val = if d < q_mat[i].len() { q_mat[i][d] } else { 0.0 };
                        let k_val = if d < k_mat[j].len() { k_mat[j][d] } else { 0.0 };
                        dot += q_val * k_val;
                    }
                    logits[j] = dot / scale;
                }
                attn_weights[i] = softmax(&logits, 1.0);
            }

            // Ponderación de Values
            for i in 0..n_assets {
                for d in h_start..h_end {
                    let mut sum_v = 0.0;
                    for j in 0..n_assets {
                        let v_val = if d < v_mat[j].len() { v_mat[j][d] } else { 0.0 };
                        sum_v += attn_weights[i][j] * v_val;
                    }
                    head_outputs[i][d] = sum_v;
                }
            }
        }

        // 3. Proyección de salida + Residual Connection + LayerNorm
        let mut final_out = vec![vec![0.0; self.dim]; n_assets];
        for i in 0..n_assets {
            let proj = self.w_out.forward(&head_outputs[i]);
            let mut res = vec![0.0; self.dim];
            for d in 0..self.dim {
                let orig = if d < asset_embeddings[i].len() { asset_embeddings[i][d] } else { 0.0 };
                let p = if d < proj.len() { proj[d] } else { 0.0 };
                res[d] = orig + p;
            }
            final_out[i] = self.layer_norm.forward(&res);
        }

        final_out
    }
}
