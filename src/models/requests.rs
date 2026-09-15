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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GuardarCarteraRequest {
    pub nombre: String,
    #[serde(default)]
    pub descripcion: Option<String>,
    #[serde(default = "default_tipo_ponderacion")]
    pub tipo_ponderacion: String, // "sharpe", "sortino", "manual"
    pub tickers: Vec<String>,
    pub pesos: Vec<f64>,
    pub retorno_esperado: Option<f64>,
    pub volatilidad: Option<f64>,
    pub sharpe_ratio: Option<f64>,
    pub ccl_ref: Option<f64>,
    pub rf_rate: Option<f64>,
}

fn default_tipo_ponderacion() -> String {
    "sharpe".to_string()
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NeuralAllocationRequest {
    #[serde(default)]
    pub tickers: Vec<String>,
    #[serde(default = "default_neural_lookback")]
    pub lookback_window: usize,
    #[serde(default = "default_neural_rebalance")]
    pub rebalance_freq: usize,
    #[serde(default = "default_max_cardinality")]
    pub max_cardinality: usize,
    #[serde(default = "default_min_asset_weight")]
    pub min_asset_weight: f64,
    #[serde(default = "default_fee_bps")]
    pub transaction_fee_bps: f64,
    #[serde(default = "default_ccl_ref")]
    pub ccl_ref: f64,
    #[serde(default = "default_rf_rate")]
    pub rf_rate: f64,
    #[serde(default = "default_seed")]
    pub seed: u64,
}

fn default_neural_lookback() -> usize {
    63
}

fn default_neural_rebalance() -> usize {
    21
}

fn default_max_cardinality() -> usize {
    5
}

fn default_min_asset_weight() -> f64 {
    0.05
}

fn default_fee_bps() -> f64 {
    10.0
}

fn default_seed() -> u64 {
    42
}

