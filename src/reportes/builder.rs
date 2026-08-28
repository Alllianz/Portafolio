use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::fs;
use std::path::Path;

/// Retorna el data URI base64 del logo 4.png para incrustación directa sin dependencias de red.
pub fn obtener_logo_base64_data_uri() -> Option<String> {
    let candidatos = [
        "4.png",
        "../4.png",
        "src/ui/4.png",
        "c:/Users/Allianz/Downloads/Club de Finanzas UBA/4.png",
    ];

    for path_str in candidatos {
        let p = Path::new(path_str);
        if p.exists() {
            if let Ok(bytes) = fs::read(p) {
                let encoded = STANDARD.encode(&bytes);
                return Some(format!("data:image/png;base64,{}", encoded));
            }
        }
    }
    None
}

/// Inyecta el logo en formato Base64 para que se renderice siempre al 100%, incluso sin conexión o en PDF.
fn inyectar_logo_en_html(mut html: String) -> String {
    if let Some(data_uri) = obtener_logo_base64_data_uri() {
        html = html.replace("src=\"4.png\"", &format!("src=\"{}\"", data_uri));
        html = html.replace("src=\"/4.png\"", &format!("src=\"{}\"", data_uri));
        html = html.replace("src=\"logo.png\"", &format!("src=\"{}\"", data_uri));
        html = html.replace("src=\"/logo.png\"", &format!("src=\"{}\"", data_uri));
        html = html.replace("src='4.png'", &format!("src='{}'", data_uri));
        html = html.replace("src='/4.png'", &format!("src='{}'", data_uri));
    }
    html
}

/// Retorna el HTML del informe institucional de 6 páginas maquetado con logos incrustados.
pub fn obtener_html_reporte_institucional() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let candidatos = [
        "Portafolio_Agosto_2026_Vertical_6_paginas.html",
        "Portafolio_Agosto_16_9.html",
        "../Portafolio_Agosto_2026_Vertical_6_paginas.html",
        "../Portafolio_Agosto_16_9.html",
    ];

    for path_str in candidatos {
        let p = Path::new(path_str);
        if p.exists() {
            if let Ok(content) = fs::read_to_string(p) {
                return Ok(inyectar_logo_en_html(content));
            }
        }
    }

    Ok(inyectar_logo_en_html(generar_plantilla_reporte_fallback()))
}

/// Retorna el HTML del informe de seguimiento de portafolio vs SPY con logos incrustados.
pub fn obtener_html_reporte_seguimiento() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let candidatos = [
        "Reporte_Seguimiento.html",
        "Dashboard_Seguimiento.html",
        "../Reporte_Seguimiento.html",
        "../Dashboard_Seguimiento.html",
    ];

    for path_str in candidatos {
        let p = Path::new(path_str);
        if p.exists() {
            if let Ok(content) = fs::read_to_string(p) {
                return Ok(inyectar_logo_en_html(content));
            }
        }
    }

    Ok(inyectar_logo_en_html(generar_plantilla_reporte_fallback()))
}



fn generar_plantilla_reporte_fallback() -> String {
    r#"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <title>Reporte Institucional - Club de Finanzas UBA</title>
    <style>
        body { font-family: sans-serif; padding: 40px; color: #0A192F; }
        h1 { color: #0062FF; }
    </style>
</head>
<body>
    <h1>Reporte Institucional de Portafolio</h1>
    <p>Plantilla base del Club de Finanzas UBA.</p>
</body>
</html>"#.to_string()
}
