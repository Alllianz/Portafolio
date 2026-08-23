use axum::response::Html;

pub async fn handler_get_reporte_html() -> Html<String> {
    let html = crate::reportes::obtener_html_reporte_institucional()
        .unwrap_or_else(|_| "<h1>Error al cargar el reporte institucional</h1>".to_string());
    Html(html)
}
