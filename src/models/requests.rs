use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OptimizarRequest {
    pub tickers: Vec<String>,
    #[serde(default = "default_n_velas")]
    pub n_velas: usize,
    #[serde(default = "default_ccl_ref")]
    pub ccl_ref: f64,
    #[serde(default = "default_rf_rate")]
    pub rf_rate: f64,
    #[serde(default = "default_min_bound")]
    pub min_bound: f64,
    #[serde(default = "default_poblacional")]
    pub poblacional: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IsOosRequest {
    pub tickers: Vec<String>,
    #[serde(default = "default_is_velas")]
    pub is_velas: usize,
    #[serde(default = "default_oos_velas")]
    pub oos_velas: usize,
    #[serde(default = "default_rebalance_freq")]
    pub rebalance_freq: String, // "diario", "semanal", "mensual", "sin_rebalanceo"
    #[serde(default = "default_benchmark")]
    pub benchmark: String, // Ej "SPY" o "SPY: 60, QQQ: 40"
    #[serde(default = "default_ccl_ref")]
    pub ccl_ref: f64,
    #[serde(default = "default_rf_rate")]
    pub rf_rate: f64,
    #[serde(default = "default_min_bound")]
    pub min_bound: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrackingRequest {
    pub tickers: Vec<String>,
    pub pesos: Vec<f64>,
    pub fecha_inicio: String,
    #[serde(default = "default_rebalance_freq")]
    pub rebalance_freq: String,
    #[serde(default = "default_ccl_ref")]
    pub ccl_ref: f64,
    #[serde(default = "default_rf_rate")]
    pub rf_rate: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DescargarTickersRequest {
    pub tickers: Vec<String>,
}

fn default_n_velas() -> usize {
    2520
}

fn default_is_velas() -> usize {
    252
}

fn default_oos_velas() -> usize {
    252
}

fn default_rebalance_freq() -> String {
    "mensual".to_string()
}

fn default_benchmark() -> String {
    "SPY".to_string()
}

fn default_ccl_ref() -> f64 {
    1250.0
}

fn default_rf_rate() -> f64 {
    0.04
}

fn default_min_bound() -> f64 {
    0.05
}

fn default_poblacional() -> bool {
    false
}
