use std::fs;
use std::path::Path;

/// Retorna el HTML del informe institucional de 6 páginas maquetado.
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
                return Ok(content);
            }
        }
    }

    Ok(generar_plantilla_reporte_fallback())
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
