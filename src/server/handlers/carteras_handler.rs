use axum::{
    extract::{Path, State},
    Json,
};
use crate::app::AppState;
use crate::models::{ApiResponse, CarteraGuardadaItem, GuardarCarteraRequest, ListarCarterasResponse};

/// Endpoint POST /api/carteras/guardar
pub async fn handler_guardar_cartera(
    State(state): State<AppState>,
    Json(payload): Json<GuardarCarteraRequest>,
) -> Json<ApiResponse<CarteraGuardadaItem>> {
    state.registrar_heartbeat();

    if payload.nombre.trim().is_empty() {
        return Json(ApiResponse::err("El nombre de la cartera no puede estar vacío."));
    }

    if payload.tickers.is_empty() {
        return Json(ApiResponse::err("La cartera debe contener al menos un ticker."));
    }

    if payload.tickers.len() != payload.pesos.len() {
        return Json(ApiResponse::err("La cantidad de tickers y de ponderaciones debe coincidir exactamente."));
    }

    let conn = state.db_conn.lock().await;
    match crate::db::guardar_cartera(&conn, &payload) {
        Ok(id) => match crate::db::obtener_cartera_por_id(&conn, id) {
            Ok(Some(item)) => Json(ApiResponse::ok(item)),
            Ok(None) => Json(ApiResponse::err("Cartera guardada pero no se pudo recuperar de la base de datos.")),
            Err(e) => Json(ApiResponse::err(format!("Error al recuperar cartera: {}", e))),
        },
        Err(e) => Json(ApiResponse::err(format!("Error al guardar cartera en SQLite: {}", e))),
    }
}

/// Endpoint GET /api/carteras
pub async fn handler_listar_carteras(
    State(state): State<AppState>,
) -> Json<ApiResponse<ListarCarterasResponse>> {
    state.registrar_heartbeat();
    let conn = state.db_conn.lock().await;

    match crate::db::listar_carteras(&conn) {
        Ok(carteras) => {
            let total = carteras.len();
            Json(ApiResponse::ok(ListarCarterasResponse {
                total,
                carteras,
            }))
        }
        Err(e) => Json(ApiResponse::err(format!("Error al listar carteras de la base de datos: {}", e))),
    }
}

/// Endpoint DELETE /api/carteras/:id
pub async fn handler_eliminar_cartera(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<bool>> {
    state.registrar_heartbeat();
    let conn = state.db_conn.lock().await;

    match crate::db::eliminar_cartera(&conn, id) {
        Ok(eliminado) => {
            if eliminado {
                Json(ApiResponse::ok(true))
            } else {
                Json(ApiResponse::err("No se encontró la cartera especificada."))
            }
        }
        Err(e) => Json(ApiResponse::err(format!("Error al eliminar cartera: {}", e))),
    }
}
