use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};

use crate::app::AppState;
use crate::server::handlers;

pub fn crear_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/", get(handler_index))
        .route("/api/optimizar", post(handlers::handler_optimizar))
        .route("/api/is_oos", post(handlers::handler_is_oos))
        .route("/api/tracking", post(handlers::handler_tracking))
        .route("/api/db/tickers", get(handlers::handler_get_tickers))
        .route("/api/db/resumen", get(handlers::handler_get_resumen))
        .route("/api/db/descargar", post(handlers::handler_descargar_tickers))
        .route("/api/reporte/html", get(handlers::handler_get_reporte_html))
        .route("/4.png", get(handlers::handler_get_logo))
        .route("/logo.png", get(handlers::handler_get_logo))
        .route("/api/logo", get(handlers::handler_get_logo))
        .route("/api/heartbeat", post(handler_heartbeat))
        .route("/api/shutdown", post(handler_shutdown))
        .layer(cors)
        .with_state(state)
}

async fn handler_index() -> Html<&'static str> {
    Html(crate::ui::obtener_index_html())
}

async fn handler_heartbeat(State(state): State<AppState>) -> impl IntoResponse {
    state.registrar_heartbeat();
    Json(json!({ "status": "alive" }))
}

async fn handler_shutdown() -> impl IntoResponse {
    tokio::spawn(async {
        tokio::time::sleep(Duration::from_millis(300)).await;
        println!("\n✓ Cierre de ventana detectado. Finalizando consola...");
        std::process::exit(0);
    });
    Json(json!({ "status": "shutting_down" }))
}
