use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OptimizarResponse {
    pub success: bool,
    pub message: String,
    pub tickers: Vec<String>,
    pub n_velas: usize,
    pub pesos_sharpe: Vec<f64>,
    pub pesos_sortino: Vec<f64>,
    pub esperados: Vec<f64>,
    pub vols: Vec<f64>,
    pub downside_vols: Vec<f64>,
    pub betas: Vec<f64>,
    pub capm_returns: Vec<f64>,
    pub port_return_sharpe: f64,
    pub port_vol_sharpe: f64,
    pub port_downside_vol: f64,
    pub sharpe_ratio: f64,
    pub sortino_ratio: f64,
    pub var_95: f64,
    pub tc_cartera_ponderado: f64,
    pub spread_ccl: f64,
    pub ccl_ref: f64,
    pub rf_rate: f64,
    pub retornos_map: HashMap<String, Vec<f64>>,
    pub series_map: HashMap<String, Vec<f64>>,
    pub time_labels: Vec<String>,
    pub frontera_puntos: Vec<(f64, f64)>, // (volatilidad_anual, retorno_anual)
    pub corr_tickers: Vec<String>,
    pub matriz_correlacion: Vec<Vec<f64>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IsOosResponse {
    pub success: bool,
    pub message: String,
    pub tickers: Vec<String>,
    pub benchmark_nombre: String,
    pub rebalance_freq: String,
    // Métricas In-Sample (Entrenamiento)
    pub is_return: f64,
    pub is_vol: f64,
    pub is_sharpe: f64,
    pub is_max_dd: f64,
    pub is_gain_pct: f64,
    // Métricas Out-Of-Sample (Prueba con Rebalanceo)
    pub oos_return: f64,
    pub oos_vol: f64,
    pub oos_sharpe: f64,
    pub oos_max_dd: f64,
    pub oos_gain_pct: f64,
    // Métricas Benchmark en ambos periodos
    pub bm_is_return: f64,
    pub bm_is_vol: f64,
    pub bm_is_sharpe: f64,
    pub bm_oos_return: f64,
    pub bm_oos_vol: f64,
    pub bm_oos_sharpe: f64,
    pub bm_oos_gain_pct: f64,
    // Ponderaciones calibradas en IS
    pub pesos_sharpe: Vec<f64>,
    pub pesos_sortino: Vec<f64>,
    // Curvas continuas completas de evolución (Base 100)
    pub full_time_labels: Vec<String>,
    pub port_full_equity_curve: Vec<f64>, // Base 100
    pub bm_full_equity_curve: Vec<f64>,   // Base 100
    pub split_index: usize,
    pub ccl_ref: f64,
    pub rf_rate: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrackingResponse {
    pub success: bool,
    pub message: String,
    pub tickers: Vec<String>,
    pub pesos: Vec<f64>,
    pub fecha_inicio: String,
    pub actual_start_date: String,
    pub n_velas: usize,
    pub tracking_gain_pct: f64,
    pub tracking_max_dd_pct: f64,
    pub tracking_sharpe: f64,
    pub tracking_sortino: f64,
    pub tracking_avg_stagnation_days: f64,
    pub tracking_max_stagnation_days: usize,
    pub ann_ret: f64,
    pub vol_anual: f64,
    pub tc_cartera_ponderado: f64,
    pub spread_ccl: f64,
    pub port_equity_curve: Vec<f64>,
    pub spy_equity_curve: Vec<f64>,
    pub time_labels: Vec<String>,
    pub series_map: HashMap<String, Vec<f64>>,
    pub retornos_map: HashMap<String, Vec<f64>>,
    pub esperados: Vec<f64>,
    pub vols: Vec<f64>,
    pub downside_vols: Vec<f64>,
    pub betas: Vec<f64>,
    pub capm_returns: Vec<f64>,
    pub pesos_sharpe: Vec<f64>,
    pub pesos_sortino: Vec<f64>,
    pub port_return_sharpe: f64,
    pub port_vol_sharpe: f64,
    pub sharpe_ratio: f64,
    pub sortino_ratio: f64,
    pub var_95: f64,
    pub ccl_ref: f64,
    pub rf_rate: f64,
    pub frontera_puntos: Vec<(f64, f64)>,
    pub corr_tickers: Vec<String>,
    pub matriz_correlacion: Vec<Vec<f64>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TickerResumenItem {
    pub ticker: String,
    pub count: usize,
    pub desde: String,
    pub hasta: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DbResumenResponse {
    pub total_tickers: usize,
    pub items: Vec<TickerResumenItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DescargarTickersResponse {
    pub success: bool,
    pub message: String,
    pub procesados: Vec<(String, usize, bool)>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CarteraGuardadaItem {
    pub id: i64,
    pub nombre: String,
    pub descripcion: String,
    pub fecha_creacion: String,
    pub tipo_ponderacion: String,
    pub tickers: Vec<String>,
    pub pesos: Vec<f64>,
    pub retorno_esperado: Option<f64>,
    pub volatilidad: Option<f64>,
    pub sharpe_ratio: Option<f64>,
    pub ccl_ref: Option<f64>,
    pub rf_rate: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListarCarterasResponse {
    pub total: usize,
    pub carteras: Vec<CarteraGuardadaItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(error: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error.into()),
        }
    }
}
