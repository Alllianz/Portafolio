use super::cross_attention::CrossSectionalAttention;
use super::layers::{DenseLayer, GatedResidualNetwork, gelu};

/// Red Neuronal de Asignación de Portafolio (DeepPortfolioNet)
/// Procesa matrices de features causales de todos los activos, aplica atención transversal
/// y emite los logits de preferencia de inversión.
#[derive(Debug, Clone)]
pub struct DeepPortfolioNet {
    pub input_dim: usize,
    pub latent_dim: usize,
    pub feature_projection: DenseLayer,
    pub grn: GatedResidualNetwork,
    pub cross_attention: CrossSectionalAttention,
    pub scoring_head: DenseLayer,
}

impl DeepPortfolioNet {
    pub fn new(input_dim: usize, latent_dim: usize, num_heads: usize, seed: u64) -> Self {
        Self {
            input_dim,
            latent_dim,
            feature_projection: DenseLayer::new(input_dim, latent_dim, seed),
            grn: GatedResidualNetwork::new(latent_dim, seed + 100),
            cross_attention: CrossSectionalAttention::new(latent_dim, num_heads, seed + 200),
            scoring_head: DenseLayer::new(latent_dim, 1, seed + 300),
        }
    }

    /// Forward pass completo de inferencia:
    /// features_matrix: [N_assets][input_dim] (Features causales de cada activo a t-1)
    /// Retorna: logits continuos [N_assets]
    pub fn forward(&self, features_matrix: &[Vec<f64>]) -> Vec<f64> {
        let n_assets = features_matrix.len();
        if n_assets == 0 {
            return Vec::new();
        }

        // 1. Proyección inicial a dimensión latente + No linealidad GELU
        let mut latent_embeddings = Vec::with_capacity(n_assets);
        for f_vec in features_matrix {
            let proj = self.feature_projection.forward(f_vec);
            let act: Vec<f64> = proj.iter().map(|&v| gelu(v)).collect();
            // 2. Gated Residual Network para selección de factores
            let refined = self.grn.forward(&act);
            latent_embeddings.push(refined);
        }

        // 3. Multi-Head Cross-Sectional Attention (Compara los N activos simultáneamente)
        let contextualized = self.cross_attention.forward(&latent_embeddings);

        // 4. Scoring Policy Head (Emisión de Logit escalar por activo)
        let mut logits = Vec::with_capacity(n_assets);
        for emb in contextualized {
            let score_vec = self.scoring_head.forward(&emb);
            let score = if !score_vec.is_empty() { score_vec[0] } else { 0.0 };
            logits.push(score);
        }

        logits
    }
}
