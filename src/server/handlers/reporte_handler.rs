use axum::{
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
};
use std::fs;
use std::path::Path;

pub async fn handler_get_reporte_html() -> Html<String> {
    let html = crate::reportes::obtener_html_reporte_institucional()
        .unwrap_or_else(|_| "<h1>Error al cargar el reporte institucional</h1>".to_string());
    Html(html)
}

pub async fn handler_get_reporte_seguimiento_html() -> Html<String> {
    let html = crate::reportes::obtener_html_reporte_seguimiento()
        .unwrap_or_else(|_| "<h1>Error al cargar el informe de seguimiento</h1>".to_string());
    Html(html)
}


pub async fn handler_get_logo() -> Response {
    let candidatos = ["4.png", "../4.png", "src/ui/4.png"];
    for c in candidatos {
        let p = Path::new(c);
        if p.exists() {
            if let Ok(bytes) = fs::read(p) {
                let mut headers = HeaderMap::new();
                headers.insert(header::CONTENT_TYPE, "image/png".parse().unwrap());
                headers.insert(header::CACHE_CONTROL, "public, max-age=86400".parse().unwrap());
                return (StatusCode::OK, headers, bytes).into_response();
            }
        }
    }
    StatusCode::NOT_FOUND.into_response()
}
