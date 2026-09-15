use axum::{extract::State, Json};
use std::collections::HashMap;

use crate::app::AppState;
use crate::finance::deep_portfolio::{
    DeepPortfolioConfig, DeepPortfolioExecutionResult, DeepPortfolioService, PortfolioConstraints,
};
use crate::models::{ApiResponse, NeuralAllocationRequest};

pub async fn handler_neural_allocation(
    State(state): State<AppState>,
    Json(payload): Json<NeuralAllocationRequest>,
) -> Json<ApiResponse<DeepPortfolioExecutionResult>> {
    state.registrar_heartbeat();

    // 1. Determinar el universo de activos
    let mut tickers: Vec<String> = if payload.tickers.is_empty() {
        let conn = state.db_conn.lock().await;
        crate::db::obtener_tickers_guardados(&conn).unwrap_or_default()
    } else {
        payload
            .tickers
            .into_iter()
            .map(|s| s.trim().to_uppercase())
            .filter(|s| !s.is_empty())
            .collect()
    };

    // Asegurar que SPY siempre esté para el Benchmark
    if !tickers.contains(&"SPY".to_string()) {
        tickers.insert(0, "SPY".to_string());
    }

    // Deduplicar manteniendo orden de inserción determinista
    let mut seen = std::collections::HashSet::new();
    tickers.retain(|t| seen.insert(t.clone()));

    if tickers.len() < 3 {
        return Json(ApiResponse::err(
            "Se requieren al menos 3 activos para la simulación neuronal multiactivo.",
        ));
    }

    // 2. Descargar o actualizar cotizaciones si es necesario
    let min_velas_req = (payload.lookback_window + 50).max(150);
    for ticker in &tickers {
        state.registrar_heartbeat();
        let count_disponibles = {
            let conn = state.db_conn.lock().await;
            crate::db::obtener_precios_ticker(&conn, ticker, 2520)
                .map(|v| v.len())
                .unwrap_or(0)
        };

        if count_disponibles < min_velas_req {
            match crate::ingestion::descargar_cotizaciones_yahoo(ticker).await {
                Ok(raw_data) => {
                    let conn = state.db_conn.lock().await;
                    let _ = crate::ingestion::persistir_cotizaciones(&conn, ticker, raw_data);
                }
                Err(e) => {
                    eprintln!("Advertencia: no se pudieron descargar datos para {}: {}", ticker, e);
                }
            }
        }
    }

    state.registrar_heartbeat();

    // 3. Alinear series temporales por fecha común
    let conn = state.db_conn.lock().await;
    let mut ticker_data_map: HashMap<String, Vec<(String, f64)>> = HashMap::new();

    for ticker in &tickers {
        if let Ok(data) = crate::db::obtener_precios_ticker(&conn, ticker, 2520) {
            if !data.is_empty() {
                ticker_data_map.insert(ticker.clone(), data);
            }
        }
    }

    if !ticker_data_map.contains_key("SPY") {
        return Json(ApiResponse::err("No se encontraron cotizaciones para el benchmark SPY."));
    }

    let spy_series = ticker_data_map.get("SPY").unwrap();
    let mut valid_dates: Vec<String> = spy_series.iter().map(|(d, _)| d.clone()).collect();

    // Filtrar activos disponibles (excluyendo SPY del universo de inversión directa, SPY actúa como Benchmark)
    let investable_tickers: Vec<String> = tickers
        .into_iter()
        .filter(|t| t != "SPY" && ticker_data_map.contains_key(t))
        .collect();

    if investable_tickers.len() < 2 {
        return Json(ApiResponse::err(
            "No hay suficientes activos invertibles con cotizaciones históricas cargadas.",
        ));
    }

    // Intersección de fechas
    for t in &investable_tickers {
        let series = ticker_data_map.get(t).unwrap();
        let date_set: std::collections::HashSet<String> = series.iter().map(|(d, _)| d.clone()).collect();
        valid_dates.retain(|d| date_set.contains(d));
    }

    if valid_dates.len() <= payload.lookback_window + 10 {
        return Json(ApiResponse::err(format!(
            "Solo hay {} fechas comunes para el universo seleccionado. Se requieren al menos {}.",
            valid_dates.len(),
            payload.lookback_window + 15
        )));
    }

    // 4. Construir matrices de precios alineadas
    let mut prices_matrix: Vec<Vec<f64>> = Vec::new();
    for t in &investable_tickers {
        let series = ticker_data_map.get(t).unwrap();
        let price_by_date: HashMap<String, f64> = series.iter().cloned().collect();
        let aligned_prices: Vec<f64> = valid_dates.iter().map(|d| *price_by_date.get(d).unwrap_or(&100.0)).collect();
        prices_matrix.push(aligned_prices);
    }

    let spy_by_date: HashMap<String, f64> = spy_series.iter().cloned().collect();
    let aligned_bm_prices: Vec<f64> = valid_dates.iter().map(|d| *spy_by_date.get(d).unwrap_or(&100.0)).collect();

    // 5. Configurar hiperparámetros
    let config = DeepPortfolioConfig {
        lookback_window: payload.lookback_window,
        rebalance_freq: payload.rebalance_freq,
        latent_dim: 32,
        num_attention_heads: 4,
        temperature: 1.0,
        seed: payload.seed,
        constraints: PortfolioConstraints {
            max_cardinality: payload.max_cardinality,
            min_asset_weight: payload.min_asset_weight,
            max_asset_weight: 1.00,
            transaction_fee_bps: payload.transaction_fee_bps,
            quadratic_impact_bps: 2.0,
            allow_cash: false,
        },
    };

    // 6. Ejecutar simulación Walk-Forward
    match DeepPortfolioService::run_experiment(
        &investable_tickers,
        &prices_matrix,
        &aligned_bm_prices,
        &valid_dates,
        &config,
    ) {
        Ok(result) => Json(ApiResponse::ok(result)),
        Err(e) => Json(ApiResponse::err(format!("Error en simulación neuronal: {}", e))),
    }
}
