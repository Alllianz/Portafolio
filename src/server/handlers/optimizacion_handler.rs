use axum::{extract::State, Json};
use crate::app::AppState;
use crate::models::{ApiResponse, OptimizarRequest, OptimizarResponse};

pub async fn handler_optimizar(
    State(state): State<AppState>,
    Json(mut payload): Json<OptimizarRequest>,
) -> Json<ApiResponse<OptimizarResponse>> {
    state.registrar_heartbeat();

    payload.tickers = payload
        .tickers
        .into_iter()
        .map(|s| s.trim().to_uppercase())
        .filter(|s| !s.is_empty())
        .collect();

    if payload.tickers.is_empty() {
        return Json(ApiResponse::err("Debe ingresar al menos un ticker para optimizar."));
    }

    let tickers_to_check = {
        let mut t_list = payload.tickers.clone();
        if !t_list.contains(&"SPY".to_string()) {
            t_list.insert(0, "SPY".to_string());
        }
        t_list
    };

    for ticker in &tickers_to_check {
        state.registrar_heartbeat();
        let count_disponibles = {
            let conn = state.db_conn.lock().await;
            crate::db::obtener_precios_ticker(&conn, ticker, payload.n_velas + 1)
                .map(|v| v.len())
                .unwrap_or(0)
        };

        if count_disponibles < (payload.n_velas / 3).max(5) {
            match crate::ingestion::descargar_cotizaciones_yahoo(ticker).await {
                Ok(raw_data) => {
                    let conn = state.db_conn.lock().await;
                    let _ = crate::ingestion::persistir_cotizaciones(&conn, ticker, raw_data);
                }
                Err(e) => {
                    return Json(ApiResponse::err(format!(
                        "No se pudieron obtener cotizaciones de mercado para '{}': {}",
                        ticker, e
                    )));
                }
            }
            state.registrar_heartbeat();
        }
    }

    state.registrar_heartbeat();
    let conn = state.db_conn.lock().await;
    match crate::finance::ejecutar_optimizacion_cartera(&payload, &conn, state.dias_anualizacion) {
        Ok(resp) => {
            state.registrar_heartbeat();
            Json(ApiResponse::ok(resp))
        }
        Err(e) => Json(ApiResponse::err(format!("Error en el cálculo cuantitativo: {}", e))),
    }
}
