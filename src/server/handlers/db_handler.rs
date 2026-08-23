use axum::{extract::State, Json};
use crate::app::AppState;
use crate::models::{ApiResponse, DbResumenResponse, DescargarTickersRequest, DescargarTickersResponse};

pub async fn handler_get_tickers(
    State(state): State<AppState>,
) -> Json<ApiResponse<Vec<String>>> {
    state.registrar_heartbeat();
    let conn = state.db_conn.lock().await;
    match crate::db::obtener_tickers_guardados(&conn) {
        Ok(tickers) => Json(ApiResponse::ok(tickers)),
        Err(e) => Json(ApiResponse::err(format!("Error al obtener tickers: {}", e))),
    }
}

pub async fn handler_get_resumen(
    State(state): State<AppState>,
) -> Json<ApiResponse<DbResumenResponse>> {
    state.registrar_heartbeat();
    let conn = state.db_conn.lock().await;
    match crate::db::obtener_resumen_db(&conn) {
        Ok(items) => {
            let total = items.len();
            Json(ApiResponse::ok(DbResumenResponse {
                total_tickers: total,
                items,
            }))
        }
        Err(e) => Json(ApiResponse::err(format!("Error al obtener resumen de DB: {}", e))),
    }
}

pub async fn handler_descargar_tickers(
    State(state): State<AppState>,
    Json(payload): Json<DescargarTickersRequest>,
) -> Json<ApiResponse<DescargarTickersResponse>> {
    state.registrar_heartbeat();
    let mut procesados = Vec::new();

    for sym in payload.tickers {
        state.registrar_heartbeat();
        let clean = sym.trim().to_uppercase();
        if clean.is_empty() {
            continue;
        }

        match crate::ingestion::descargar_cotizaciones_yahoo(&clean).await {
            Ok(raw_data) => {
                state.registrar_heartbeat();
                let conn = state.db_conn.lock().await;
                match crate::ingestion::persistir_cotizaciones(&conn, &clean, raw_data) {
                    Ok(count) => {
                        procesados.push((clean, count, true));
                    }
                    Err(_) => {
                        procesados.push((clean, 0, false));
                    }
                }
            }
            Err(_) => {
                procesados.push((clean, 0, false));
            }
        }
        state.registrar_heartbeat();
    }

    Json(ApiResponse::ok(DescargarTickersResponse {
        success: true,
        message: format!("Descarga procesada para {} tickers.", procesados.len()),
        procesados,
    }))
}
