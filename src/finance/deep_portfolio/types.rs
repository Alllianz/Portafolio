use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Restricciones operativas duras del portafolio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioConstraints {
    pub max_cardinality: usize,     // Máximo de activos (por defecto 5)
    pub min_asset_weight: f64,      // Piso mínimo por posición abierta (por defecto 0.05 = 5%)
    pub max_asset_weight: f64,      // Techo máximo por posición (ej: 0.60 o 1.00)
    pub transaction_fee_bps: f64,   // Comisión lineal en basis points (ej: 10 bps = 0.0010)
    pub quadratic_impact_bps: f64,  // Slippage cuadrático de impacto (ej: 5 bps)
    pub allow_cash: bool,           // Si permite reservar liquidez/caja defensiva
}

impl Default for PortfolioConstraints {
    fn default() -> Self {
        Self {
            max_cardinality: 5,
            min_asset_weight: 0.05,
            max_asset_weight: 1.00,
            transaction_fee_bps: 10.0,
            quadratic_impact_bps: 2.0,
            allow_cash: false,
        }
    }
}

/// Configuración de hiperparámetros del modelo neuronal y cálculo causal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepPortfolioConfig {
    pub lookback_window: usize,      // Ventana causal histórica (ej: 63 o 126 días)
    pub rebalance_freq: usize,       // Frecuencia de rebalanceo en ruedas (ej: 21 ruedas = mensual, 5 = semanal)
    pub latent_dim: usize,           // Dimensión latente de embeddings (ej: 32 o 64)
    pub num_attention_heads: usize,  // Número de cabezas de Multi-Head Attention (ej: 4)
    pub temperature: f64,            // Temperatura del Softmax para dispersión (ej: 1.0)
    pub seed: u64,                   // Semilla determinista de inicialización de pesos neuronales
    pub constraints: PortfolioConstraints,
}

impl Default for DeepPortfolioConfig {
    fn default() -> Self {
        Self {
            lookback_window: 63,
            rebalance_freq: 21,
            latent_dim: 32,
            num_attention_heads: 4,
            temperature: 1.0,
            seed: 42,
            constraints: PortfolioConstraints::default(),
        }
    }
}

/// Registro de una asignación puntual en un instante temporal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepAllocation {
    pub date: String,
    pub step_index: usize,
    pub weights: Vec<f64>,                  // Ponderación de cada ticker
    pub active_tickers: Vec<String>,        // Tickers seleccionados (máx 5)
    pub active_weights: Vec<f64>,          // Ponderaciones de los seleccionados (>= 5%)
    pub turnover: f64,                      // Rotación de cartera
    pub transaction_cost_pct: f64,          // Costo de fricción deducido
    pub gross_return: f64,                  // Retorno bruto del período
    pub net_return: f64,                    // Retorno neto tras comisiones
    pub portfolio_equity: f64,              // Valor acumulado de la cartera (Base 100)
    pub benchmark_equity: f64,              // Valor acumulado del benchmark (Base 100)
    pub equal_weight_equity: f64,          // Valor acumulado de cartera 1/N (Base 100)
}

/// Resumen de auditoría de integridad estadística
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditVerificationReport {
    pub lookahead_bias_detected: bool,
    pub max_cardinality_violations: usize,
    pub min_weight_violations: usize,
    pub sum_weights_tolerance_violations: usize,
    pub total_rebalance_events: usize,
    pub avg_turnover_per_rebalance: f64,
    pub total_transaction_costs_absorbed: f64,
    pub audit_passed: bool,
    pub audit_notes: Vec<String>,
}

/// Resultado final de la simulación neuronal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepPortfolioExecutionResult {
    pub success: bool,
    pub message: String,
    pub tickers: Vec<String>,
    pub time_labels: Vec<String>,
    pub portfolio_equity_curve: Vec<f64>,
    pub benchmark_equity_curve: Vec<f64>,
    pub equal_weight_equity_curve: Vec<f64>,
    pub net_returns: Vec<f64>,
    pub benchmark_returns: Vec<f64>,
    pub allocations_history: Vec<StepAllocation>,
    pub weights_matrix_history: Vec<Vec<f64>>, // [Paso][Ticker]
    pub active_selection_counts: HashMap<String, usize>,
    
    // Métricas cuantitativas OOS
    pub total_net_gain_pct: f64,
    pub benchmark_gain_pct: f64,
    pub equal_weight_gain_pct: f64,
    pub alpha_annualized_pct: f64,
    pub beta_vs_benchmark: f64,
    pub annualized_net_return_pct: f64,
    pub annualized_volatility_pct: f64,
    pub sharpe_ratio: f64,
    pub sortino_ratio: f64,
    pub calmar_ratio: f64,
    pub max_drawdown_pct: f64,
    pub benchmark_max_drawdown_pct: f64,
    pub win_rate_pct: f64,
    
    // Informe de auditoría
    pub audit_report: AuditVerificationReport,
}
