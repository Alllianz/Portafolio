/// Algoritmo online de Welford para cálculo de media y varianza causal en O(1)
/// Permite estandarización de features (Z-Score) sin fuga de datos futuros.
#[derive(Debug, Clone)]
pub struct WelfordOnlineScaler {
    count: usize,
    mean: f64,
    m2: f64,
    epsilon: f64,
}

impl WelfordOnlineScaler {
    pub fn new() -> Self {
        Self {
            count: 0,
            mean: 0.0,
            m2: 0.0,
            epsilon: 1e-8,
        }
    }

    /// Actualiza el estado con una nueva observación causal x_t
    pub fn update(&mut self, x: f64) {
        self.count += 1;
        let delta = x - self.mean;
        self.mean += delta / (self.count as f64);
        let delta2 = x - self.mean;
        self.m2 += delta * delta2;
    }

    /// Retorna la media muestral acumulada
    pub fn mean(&self) -> f64 {
        self.mean
    }

    /// Retorna la varianza muestral (con corrección de Bessel n-1)
    pub fn variance(&self) -> f64 {
        if self.count < 2 {
            0.0
        } else {
            self.m2 / ((self.count - 1) as f64)
        }
    }

    /// Retorna el desvío estándar muestral
    pub fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }

    /// Normaliza un valor utilizando EXCLUSIVAMENTE los parámetros calculados hasta el momento
    pub fn transform(&self, x: f64) -> f64 {
        let std = self.std_dev();
        if std < self.epsilon {
            0.0
        } else {
            (x - self.mean) / (std + self.epsilon)
        }
    }

    /// Actualiza el estado y retorna el valor normalizado causalmente
    pub fn update_and_transform(&mut self, x: f64) -> f64 {
        let z = self.transform(x);
        self.update(x);
        z
    }
}

/// Normalizador rodante con ventana finita (Rolling Causal Z-Score)
#[derive(Debug, Clone)]
pub struct RollingCausalScaler {
    window_size: usize,
    epsilon: f64,
}

impl RollingCausalScaler {
    pub fn new(window_size: usize) -> Self {
        Self {
            window_size: window_size.max(5),
            epsilon: 1e-8,
        }
    }

    /// Calcula el Z-score de un valor en base a los últimos W valores estrictamente anteriores
    pub fn normalize_causal(&self, series: &[f64], current_idx: usize) -> f64 {
        if current_idx == 0 || series.is_empty() {
            return 0.0;
        }

        let start_idx = current_idx.saturating_sub(self.window_size);
        let window = &series[start_idx..current_idx]; // Excluye current_idx (estricto t-1)
        if window.is_empty() {
            return 0.0;
        }

        let n = window.len() as f64;
        let mean = window.iter().sum::<f64>() / n;
        let var = if window.len() > 1 {
            window.iter().map(|&v| (v - mean).powi(2)).sum::<f64>() / (n - 1.0)
        } else {
            0.0
        };
        let std = var.sqrt();

        if std < self.epsilon {
            0.0
        } else {
            let val = series[current_idx];
            ((val - mean) / (std + self.epsilon)).clamp(-5.0, 5.0)
        }
    }
}
