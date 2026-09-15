use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// Capa lineal densa y operaciones de activación
#[derive(Debug, Clone)]
pub struct DenseLayer {
    pub in_features: usize,
    pub out_features: usize,
    pub weights: Vec<Vec<f64>>, // [out_features][in_features]
    pub bias: Vec<f64>,        // [out_features]
}

impl DenseLayer {
    pub fn new(in_features: usize, out_features: usize, seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        // Inicialización de Xavier / Glorot
        let std_dev = (2.0 / ((in_features + out_features) as f64)).sqrt();
        
        let mut weights = vec![vec![0.0; in_features]; out_features];
        for o in 0..out_features {
            for i in 0..in_features {
                let u1: f64 = rng.gen_range(1e-7..1.0);
                let u2: f64 = rng.gen_range(1e-7..1.0);
                let normal = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
                weights[o][i] = normal * std_dev;
            }
        }
        let bias = vec![0.0; out_features];

        Self {
            in_features,
            out_features,
            weights,
            bias,
        }
    }

    /// Forward pass lineal: y = W x + b
    pub fn forward(&self, input: &[f64]) -> Vec<f64> {
        let mut out = vec![0.0; self.out_features];
        for o in 0..self.out_features {
            let mut sum = self.bias[o];
            for i in 0..self.in_features.min(input.len()) {
                sum += self.weights[o][i] * input[i];
            }
            out[o] = sum;
        }
        out
    }
}

/// Capa de Normalización de Capa (LayerNorm)
#[derive(Debug, Clone)]
pub struct LayerNorm {
    pub dim: usize,
    pub gamma: Vec<f64>,
    pub beta: Vec<f64>,
    pub eps: f64,
}

impl LayerNorm {
    pub fn new(dim: usize) -> Self {
        Self {
            dim,
            gamma: vec![1.0; dim],
            beta: vec![0.0; dim],
            eps: 1e-5,
        }
    }

    pub fn forward(&self, x: &[f64]) -> Vec<f64> {
        let n = x.len() as f64;
        if n == 0.0 {
            return Vec::new();
        }
        let mean = x.iter().sum::<f64>() / n;
        let var = x.iter().map(|&v| (v - mean).powi(2)).sum::<f64>() / n;
        let std = (var + self.eps).sqrt();

        x.iter()
            .enumerate()
            .map(|(i, &v)| {
                let g = if i < self.gamma.len() { self.gamma[i] } else { 1.0 };
                let b = if i < self.beta.len() { self.beta[i] } else { 0.0 };
                g * ((v - mean) / std) + b
            })
            .collect()
    }
}

/// Gated Residual Network (GRN) para selección dinámica de factores
#[derive(Debug, Clone)]
pub struct GatedResidualNetwork {
    pub linear1: DenseLayer,
    pub linear2: DenseLayer,
    pub gate: DenseLayer,
    pub layer_norm: LayerNorm,
    pub dim: usize,
}

impl GatedResidualNetwork {
    pub fn new(dim: usize, seed: u64) -> Self {
        Self {
            linear1: DenseLayer::new(dim, dim, seed),
            linear2: DenseLayer::new(dim, dim, seed + 1),
            gate: DenseLayer::new(dim, dim, seed + 2),
            layer_norm: LayerNorm::new(dim),
            dim,
        }
    }

    pub fn forward(&self, x: &[f64]) -> Vec<f64> {
        // Capa intermedia con activación GELU
        let h1 = self.linear1.forward(x);
        let h1_act: Vec<f64> = h1.iter().map(|&v| gelu(v)).collect();

        // Segunda transformación
        let h2 = self.linear2.forward(&h1_act);

        // Compuerta GLU: sigmoid(gate(x))
        let g = self.gate.forward(x);
        let g_sig: Vec<f64> = g.iter().map(|&v| sigmoid(v)).collect();

        // Gated: h2 * gate
        let mut gated = vec![0.0; self.dim];
        for i in 0..self.dim {
            let v_h2 = if i < h2.len() { h2[i] } else { 0.0 };
            let v_g = if i < g_sig.len() { g_sig[i] } else { 1.0 };
            let v_x = if i < x.len() { x[i] } else { 0.0 };
            // Skip-connection residual: x + Gated(h2)
            gated[i] = v_x + v_h2 * v_g;
        }

        self.layer_norm.forward(&gated)
    }
}

// Funciones de activación matemáticas
#[inline]
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

#[inline]
pub fn gelu(x: f64) -> f64 {
    0.5 * x * (1.0 + ((2.0 / std::f64::consts::PI).sqrt() * (x + 0.044715 * x.powi(3))).tanh())
}

#[inline]
pub fn relu(x: f64) -> f64 {
    if x > 0.0 { x } else { 0.0 }
}

#[inline]
pub fn softmax(logits: &[f64], temperature: f64) -> Vec<f64> {
    if logits.is_empty() {
        return Vec::new();
    }
    let temp = temperature.max(0.01);
    let max_val = logits.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let exp_vals: Vec<f64> = logits.iter().map(|&x| ((x - max_val) / temp).exp()).collect();
    let sum: f64 = exp_vals.iter().sum::<f64>().max(1e-12);
    exp_vals.iter().map(|&v| v / sum).collect()
}
