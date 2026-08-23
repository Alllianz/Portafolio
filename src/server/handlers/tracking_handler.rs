use axum::{extract::State, Json};
use crate::app::AppState;
use crate::models::{ApiResponse, TrackingRequest, TrackingResponse};

pub async fn handler_tracking(
    State(state): State<AppState>,
    Json(mut payload): Json<TrackingRequest>,
) -> Json<ApiResponse<TrackingResponse>> {
    state.registrar_heartbeat();

    payload.tickers = payload
        .tickers
        .into_iter()
        .map(|s| s.trim().to_uppercase())
        .filter(|s| !s.is_empty())
        .collect();

    if payload.tickers.is_empty() {
        return Json(ApiResponse::err("Debe ingresar al menos un ticker para el seguimiento."));
    }

    if payload.fecha_inicio.trim().is_empty() {
        payload.fecha_inicio = "2024-01-01".to_string();
    }

    let mut tickers_to_check = payload.tickers.clone();
    if !tickers_to_check.contains(&"SPY".to_string()) {
        tickers_to_check.insert(0, "SPY".to_string());
    }

    for ticker in &tickers_to_check {
        state.registrar_heartbeat();
        let count_disponibles = {
            let conn = state.db_conn.lock().await;
            crate::db::obtener_precios_ticker_desde_fecha(&conn, ticker, &payload.fecha_inicio)
                .map(|v| v.len())
                .unwrap_or(0)
        };

        if count_disponibles < 5 {
            match crate::ingestion::descargar_cotizaciones_yahoo(ticker).await {
                Ok(raw_data) => {
                    let conn = state.db_conn.lock().await;
                    let _ = crate::ingestion::persistir_cotizaciones(&conn, ticker, raw_data);
                }
                Err(e) => {
                    return Json(ApiResponse::err(format!(
                        "No se pudieron descargar cotizaciones para '{}': {}",
                        ticker, e
                    )));
                }
            }
            state.registrar_heartbeat();
        }
    }

    state.registrar_heartbeat();
    let conn = state.db_conn.lock().await;
    match crate::finance::tracking::ejecutar_seguimiento_cartera(&payload, &conn, state.dias_anualizacion) {
        Ok(resp) => {
            state.registrar_heartbeat();
            Json(ApiResponse::ok(resp))
        }
        Err(e) => Json(ApiResponse::err(format!("Error en seguimiento de portafolio: {}", e))),
    }
}
