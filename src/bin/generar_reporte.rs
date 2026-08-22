use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===========================================================");
    println!("  GENERADOR INFORME DE PORTAFOLIO - CLUB DE FINANZAS UBA");
    println!("  Reporte Institucional Completo (6 Páginas) & 1920x1080 PNG");
    println!("===========================================================\n");

    let source_file = find_file_path("Portafolio_Agosto_2026_Vertical_6_paginas.html")?;
    let content = fs::read_to_string(&source_file)?;

    let output_paths = vec![
        Path::new("Portafolio_Agosto_16_9.html").to_path_buf(),
        Path::new("../Portafolio_Agosto_16_9.html").to_path_buf(),
    ];

    for path in output_paths {
        if let Ok(_) = fs::write(&path, &content) {
            println!("✅ Archivo HTML generado en: {}", path.display());
        }
    }

    println!("\n¡Informe y Dashboard generados exitosamente desde la plantilla oficial de 6 páginas!");
    Ok(())
}

fn find_file_path(filename: &str) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let candidatos = vec![
        format!("../Portafolio/{}", filename),
        format!("Portafolio/{}", filename),
        format!("../{}", filename),
        filename.to_string(),
    ];

    for c in candidatos {
        let p = Path::new(&c);
        if p.exists() {
            return Ok(p.to_path_buf());
        }
    }

    Err(format!("No se encontró el archivo '{}'", filename).into())
}

fn procesar_docx_a_html<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;
    
    let mut xml_file = match archive.by_name("word/document.xml") {
        Ok(f) => f,
        Err(_) => return Err("Formato de docx no válido: no contiene word/document.xml".into()),
    };

    let mut xml_content = String::new();
    xml_file.read_to_string(&mut xml_content)?;

    let body_start = xml_content.find("<w:body>").unwrap_or(0);
    let body_end = xml_content.find("</w:body>").unwrap_or(xml_content.len());
    let body_xml = &xml_content[body_start..body_end];

    let mut out_html = String::new();
    let mut pos = 0;

    while pos < body_xml.len() {
        let p_next = body_xml[pos..].find("<w:p>");
        let p_space_next = body_xml[pos..].find("<w:p ");
        
        let p_idx = match (p_next, p_space_next) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };

        let tbl_next = body_xml[pos..].find("<w:tbl>");
        let tbl_space_next = body_xml[pos..].find("<w:tbl ");

        let tbl_idx = match (tbl_next, tbl_space_next) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };

        match (p_idx, tbl_idx) {
            (None, None) => break,
            (Some(p_offset), None) => {
                let abs_p = pos + p_offset;
                if let Some(p_end) = body_xml[abs_p..].find("</w:p>") {
                    let p_xml = &body_xml[abs_p..abs_p + p_end + 6];
                    if let Some(rendered) = render_paragraph_xml(p_xml) {
                        out_html.push_str(&rendered);
                    }
                    pos = abs_p + p_end + 6;
                } else {
                    pos += p_offset + 4;
                }
            }
            (None, Some(tbl_offset)) => {
                let abs_tbl = pos + tbl_offset;
                if let Some(tbl_end) = body_xml[abs_tbl..].find("</w:tbl>") {
                    let tbl_xml = &body_xml[abs_tbl..abs_tbl + tbl_end + 8];
                    let rendered = render_table_xml(tbl_xml);
                    out_html.push_str(&rendered);
                    pos = abs_tbl + tbl_end + 8;
                } else {
                    pos += tbl_offset + 6;
                }
            }
            (Some(p_offset), Some(tbl_offset)) => {
                if p_offset < tbl_offset {
                    let abs_p = pos + p_offset;
                    if let Some(p_end) = body_xml[abs_p..].find("</w:p>") {
                        let p_xml = &body_xml[abs_p..abs_p + p_end + 6];
                        if let Some(rendered) = render_paragraph_xml(p_xml) {
                            out_html.push_str(&rendered);
                        }
                        pos = abs_p + p_end + 6;
                    } else {
                        pos += p_offset + 4;
                    }
                } else {
                    let abs_tbl = pos + tbl_offset;
                    if let Some(tbl_end) = body_xml[abs_tbl..].find("</w:tbl>") {
                        let tbl_xml = &body_xml[abs_tbl..abs_tbl + tbl_end + 8];
                        let rendered = render_table_xml(tbl_xml);
                        out_html.push_str(&rendered);
                        pos = abs_tbl + tbl_end + 8;
                    } else {
                        pos += tbl_offset + 6;
                    }
                }
            }
        }
    }

    Ok(out_html)
}

fn render_table_xml(tbl_xml: &str) -> String {
    let mut table_html = String::new();
    table_html.push_str(r#"<div class="table-container"><table class="doc-table">"#);

    let mut is_first_row = true;
    for row_chunk in tbl_xml.split("<w:tr") {
        if row_chunk.is_empty() || !row_chunk.contains("</w:tr>") {
            continue;
        }
        if row_chunk.starts_with("Pr>") || row_chunk.starts_with("Pr ") {
            continue;
        }

        let row_end = row_chunk.find("</w:tr>").unwrap_or(row_chunk.len());
        let row_content = &row_chunk[..row_end];

        let mut row_cells = Vec::new();
        for cell_chunk in row_content.split("<w:tc") {
            if cell_chunk.is_empty() || !cell_chunk.contains("</w:tc>") {
                continue;
            }
            if cell_chunk.starts_with("Pr>") || cell_chunk.starts_with("Pr ") || cell_chunk.starts_with("Mar>") {
                continue;
            }

            let cell_end = cell_chunk.find("</w:tc>").unwrap_or(cell_chunk.len());
            let cell_content = &cell_chunk[..cell_end];

            let text = extract_text_from_xml(cell_content);
            let trimmed = text.trim();
            row_cells.push(trimmed.to_string());
        }

        if row_cells.iter().all(|c| c.is_empty()) {
            continue;
        }

        table_html.push_str("<tr>");
        for cell_text in row_cells {
            let escaped = escape_html(&cell_text);
            if is_first_row {
                table_html.push_str(&format!("<th>{}</th>", escaped));
            } else {
                table_html.push_str(&format!("<td>{}</td>", escaped));
            }
        }
        table_html.push_str("</tr>");
        is_first_row = false;
    }

    table_html.push_str("</table></div>");
    table_html
}

fn render_paragraph_xml(p_xml: &str) -> Option<String> {
    let text = extract_text_from_xml(p_xml);
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }

    if trimmed == "Agosto 2026" || trimmed == "Agosto" {
        return None;
    }

    if trimmed == "Disclaimer" || trimmed.starts_with("El presente informe tiene finalidad exclusivamente informativa") {
        return Some(format!(r#"<div class="disclaimer-text">{}</div>"#, escape_html(trimmed)));
    }

    let escaped = escape_html(trimmed);

    if trimmed == "PORTAFOLIO TEMÁTICO" {
        Some(format!(r#"<div class="doc-title-badge">PORTAFOLIO TEMÁTICO</div>"#))
    } else if trimmed == "Energía e Infraestructura de IA" {
        Some(format!(r#"<h1 class="main-title">{}</h1>"#, escaped))
    } else if trimmed.starts_with("1. ") || trimmed.starts_with("2. ") || trimmed.starts_with("3. ") 
           || trimmed.starts_with("4. ") || trimmed.starts_with("5. ") || trimmed.starts_with("6. ") 
           || trimmed.starts_with("7. ") || trimmed.starts_with("8. ") || trimmed.starts_with("9. ") 
           || trimmed.starts_with("10. ") || trimmed.starts_with("Fuentes") {
        Some(format!(r#"<h2 class="section-heading">{}</h2>"#, escaped))
    } else if trimmed.starts_with("Sector:") || trimmed.starts_with("Ponderación") 
           || trimmed.starts_with("Rol en") || trimmed.starts_with("Números") 
           || trimmed.starts_with("El Riesgo:") || trimmed.starts_with("Valor Agregado:") {
        let (key, val) = if let Some(idx) = escaped.find(':') {
            (&escaped[..=idx], &escaped[idx+1..])
        } else {
            ("", escaped.as_str())
        };
        if !key.is_empty() {
            Some(format!(r#"<div class="insight-field"><strong class="field-key">{}</strong><span class="field-val">{}</span></div>"#, key, val))
        } else {
            Some(format!(r#"<div class="insight-field"><span class="field-val">{}</span></div>"#, escaped))
        }
    } else if trimmed.starts_with("Perfil sugerido:") {
        Some(format!(r#"<div class="profile-callout">{}</div>"#, escaped))
    } else if trimmed.len() < 70 && (trimmed.contains("XOM") || trimmed.contains("SO") || trimmed.contains("WMB") || trimmed.contains("ETN") || trimmed.contains("VST") || trimmed.contains("YPF") || trimmed.contains("Exxon") || trimmed.contains("Southern") || trimmed.contains("Williams") || trimmed.contains("Eaton") || trimmed.contains("Vistra")) {
        Some(format!(r#"<h3 class="stock-heading">{}</h3>"#, escaped))
    } else {
        Some(format!(r#"<p class="paragraph">{}</p>"#, escaped))
    }
}

fn extract_text_from_xml(xml: &str) -> String {
    let mut text = String::new();
    let mut pos = 0;

    while let Some(start) = xml[pos..].find("<w:t") {
        let actual_start = pos + start;

        if actual_start + 4 < xml.len() {
            let next_byte = xml.as_bytes()[actual_start + 4];
            if next_byte != b'>' && next_byte != b' ' && next_byte != b'\t' && next_byte != b'\n' && next_byte != b'\r' {
                pos = actual_start + 4;
                continue;
            }
        }

        if let Some(tag_close) = xml[actual_start..].find('>') {
            let text_start = actual_start + tag_close + 1;
            if let Some(end) = xml[text_start..].find("</w:t>") {
                let val = &xml[text_start..text_start + end];
                let clean = strip_xml_tags(val);
                text.push_str(&clean);
                pos = text_start + end + 6;
            } else {
                pos = actual_start + 4;
            }
        } else {
            pos = actual_start + 4;
        }
    }

    text
}

fn strip_xml_tags(input: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    for c in input.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            result.push(c);
        }
    }
    result
}

fn construir_documento_final(_portfolio_html: &str, _insights_html: &str, logo_b64: &str) -> String {
    let logo_img_html = if !logo_b64.is_empty() {
        format!(r#"<img src="{}" alt="Club de Finanzas UBA" class="brand-logo-img" style="height: 38px; object-fit: contain;" />"#, logo_b64)
    } else {
        r#"<div class="brand-logo-text"><span class="logo-top">CLUB DE FINANZAS</span><span class="logo-sub">UBA</span></div>"#.to_string()
    };

    format!(r##"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Portfolio Renta Variable: Estrategia y Análisis de Activos — Club de Finanzas UBA</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;500;600;700;800&family=Playfair+Display:ital,wght@0,600;0,700;0,800;1,600&family=JetBrains+Mono:wght@500;700&display=swap" rel="stylesheet">
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/html2canvas/1.4.1/html2canvas.min.js"></script>
    <style>
        :root {{
            --doc-navy-dark: #0A192F;
            --doc-navy: #0F2D59;
            --doc-blue: #0062FF;
            --doc-blue-light: #2563EB;
            --doc-cyan: #00D2D3;
            --doc-cyan-soft: #E0F2FE;
            --doc-bg: #F1F5F9;
            --doc-card-bg: #FFFFFF;
            --doc-text-main: #0F172A;
            --doc-text-muted: #64748B;
            --doc-border: #E2E8F0;
            --doc-border-dark: #CBD5E1;
            --doc-pill-bg: #EBF3FA;
            --doc-shadow: 0 4px 20px -2px rgba(15, 23, 42, 0.08), 0 2px 6px -1px rgba(15, 23, 42, 0.04);
            --doc-shadow-lg: 0 10px 30px -4px rgba(15, 23, 42, 0.12);
        }}

        * {{
            box-sizing: border-box;
            margin: 0;
            padding: 0;
            -webkit-font-smoothing: antialiased;
        }}

        body {{
            font-family: 'Plus Jakarta Sans', -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            background-color: var(--doc-bg);
            color: var(--doc-text-main);
            display: flex;
            flex-direction: column;
            align-items: center;
            padding: 24px 16px 80px 16px;
            gap: 28px;
            -webkit-print-color-adjust: exact;
            print-color-adjust: exact;
        }}

        /* Barra de Control Flotante */
        .app-header-controls {{
            position: sticky;
            top: 16px;
            z-index: 1000;
            background: rgba(10, 25, 47, 0.95);
            backdrop-filter: blur(12px);
            color: #FFFFFF;
            border-radius: 12px;
            padding: 12px 24px;
            display: flex;
            align-items: center;
            justify-content: space-between;
            width: 100%;
            max-width: 1160px;
            box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
            border: 1px solid rgba(255, 255, 255, 0.1);
        }}

        .app-brand-summary {{
            display: flex;
            align-items: center;
            gap: 14px;
        }}

        .app-badge-tag {{
            background: var(--doc-blue);
            color: #FFFFFF;
            font-weight: 700;
            font-size: 0.75rem;
            padding: 4px 10px;
            border-radius: 6px;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }}

        .app-brand-title {{
            font-weight: 700;
            font-size: 1.05rem;
            letter-spacing: -0.01em;
        }}

        .app-header-actions {{
            display: flex;
            align-items: center;
            gap: 10px;
        }}

        .btn-header {{
            background: rgba(255, 255, 255, 0.1);
            color: #FFFFFF;
            border: 1px solid rgba(255, 255, 255, 0.2);
            padding: 8px 16px;
            border-radius: 8px;
            font-weight: 600;
            font-size: 0.85rem;
            cursor: pointer;
            transition: all 0.2s ease;
            display: inline-flex;
            align-items: center;
            gap: 6px;
            text-decoration: none;
        }}

        .btn-header:hover {{
            background: var(--doc-blue);
            border-color: var(--doc-blue);
            transform: translateY(-1px);
        }}

        .btn-header-primary {{
            background: var(--doc-blue);
            border-color: var(--doc-blue);
        }}

        .btn-header-primary:hover {{
            background: #0050d8;
        }}

        /* Document Canvas / Pages */
        .doc-page-container {{
            width: 100%;
            max-width: 1160px;
            background: #FFFFFF;
            border-radius: 12px;
            box-shadow: var(--doc-shadow-lg);
            border: 1px solid var(--doc-border);
            overflow: hidden;
            display: flex;
            flex-direction: column;
            position: relative;
        }}

        /* Encabezado Superior Institucional */
        .doc-header-strip {{
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 24px 36px 16px 36px;
            border-bottom: 1px solid var(--doc-border);
        }}

        .doc-header-left {{
            display: flex;
            align-items: center;
            gap: 14px;
        }}

        .brand-logo-text {{
            display: flex;
            flex-direction: column;
        }}

        .logo-top {{
            font-size: 0.72rem;
            font-weight: 800;
            letter-spacing: 1.2px;
            color: var(--doc-navy-dark);
            text-transform: uppercase;
        }}

        .logo-sub {{
            font-size: 1.1rem;
            font-weight: 800;
            color: var(--doc-blue);
            letter-spacing: -0.02em;
        }}

        .doc-category-divider {{
            height: 28px;
            width: 1.5px;
            background: var(--doc-border-dark);
        }}

        .doc-category-text {{
            font-size: 0.85rem;
            font-weight: 700;
            letter-spacing: 1px;
            color: var(--doc-navy-dark);
            text-transform: uppercase;
        }}

        .doc-page-badge {{
            background: var(--doc-navy-dark);
            color: #FFFFFF;
            font-size: 0.75rem;
            font-weight: 700;
            padding: 6px 14px;
            border-radius: 4px;
            letter-spacing: 1px;
            text-transform: uppercase;
        }}

        /* Hero / Portada Header */
        .doc-main-hero {{
            padding: 28px 36px 20px 36px;
            display: flex;
            justify-content: space-between;
            align-items: flex-end;
            border-bottom: 1px solid var(--doc-border);
            position: relative;
        }}

        .doc-hero-left {{
            max-width: 780px;
        }}

        .doc-main-title {{
            font-family: 'Playfair Display', Georgia, serif;
            font-size: 2.35rem;
            font-weight: 700;
            color: var(--doc-navy-dark);
            line-height: 1.15;
            letter-spacing: -0.02em;
            margin-bottom: 4px;
        }}

        .doc-main-subtitle {{
            font-family: 'Playfair Display', Georgia, serif;
            font-size: 1.85rem;
            font-weight: 700;
            color: var(--doc-navy);
            letter-spacing: -0.01em;
            margin-bottom: 10px;
        }}

        .doc-theme-badge-row {{
            display: flex;
            align-items: center;
            gap: 12px;
            font-size: 0.88rem;
            font-weight: 700;
            color: var(--doc-blue);
            margin-bottom: 6px;
        }}

        .doc-authors-line {{
            font-size: 0.82rem;
            color: var(--doc-text-muted);
            font-weight: 500;
        }}

        .doc-authors-line strong {{
            color: var(--doc-navy-dark);
        }}

        .doc-hero-decor {{
            width: 140px;
            height: 48px;
        }}

        /* Sección de Introducción */
        .doc-intro-section {{
            padding: 20px 36px;
            background: #FAFCFE;
            border-bottom: 1px solid var(--doc-border);
        }}

        .doc-section-title-inline {{
            display: flex;
            align-items: center;
            gap: 8px;
            font-size: 0.88rem;
            font-weight: 800;
            color: var(--doc-navy-dark);
            text-transform: uppercase;
            letter-spacing: 0.5px;
            margin-bottom: 10px;
        }}

        .doc-intro-p {{
            font-size: 0.88rem;
            line-height: 1.6;
            color: #334155;
            margin-bottom: 8px;
            text-align: justify;
        }}

        .doc-profile-callout {{
            font-size: 0.85rem;
            font-weight: 600;
            color: var(--doc-navy-dark);
            background: var(--doc-pill-bg);
            padding: 8px 14px;
            border-radius: 6px;
            border-left: 3px solid var(--doc-blue);
            margin-top: 8px;
        }}

        /* Rejilla de Métricas Principales (KPI Cards con Píldoras Circulares) */
        .doc-kpis-grid {{
            display: grid;
            grid-template-columns: repeat(7, 1fr);
            gap: 12px;
            padding: 24px 36px;
            border-bottom: 1px solid var(--doc-border);
            background: #FFFFFF;
        }}

        .doc-kpi-card {{
            background: #FFFFFF;
            border: 1px solid var(--doc-border);
            border-radius: 10px;
            padding: 16px 10px;
            display: flex;
            flex-direction: column;
            align-items: center;
            text-align: center;
            box-shadow: 0 2px 6px rgba(0,0,0,0.02);
            position: relative;
            transition: transform 0.2s ease, box-shadow 0.2s ease;
        }}

        .doc-kpi-card:hover {{
            transform: translateY(-2px);
            box-shadow: var(--doc-shadow);
            border-color: var(--doc-blue);
        }}

        .kpi-icon-pill {{
            width: 32px;
            height: 32px;
            border-radius: 50%;
            background: var(--doc-navy-dark);
            color: #FFFFFF;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 0.85rem;
            font-weight: 700;
            margin-bottom: 8px;
        }}

        .kpi-icon-pill.blue {{
            background: var(--doc-blue);
        }}

        .kpi-icon-pill.cyan {{
            background: #0284C7;
        }}

        .kpi-label {{
            font-size: 0.72rem;
            font-weight: 600;
            color: var(--doc-text-muted);
            line-height: 1.2;
            min-height: 28px;
            display: flex;
            align-items: center;
            justify-content: center;
        }}

        .kpi-value {{
            font-family: 'JetBrains Mono', monospace;
            font-size: 1.25rem;
            font-weight: 700;
            color: var(--doc-blue);
            margin-top: 6px;
            letter-spacing: -0.02em;
        }}

        .kpi-value.dark {{
            color: var(--doc-navy-dark);
        }}

        /* Contenedores de Gráficos Duales con Botones de Exportación 1920x1080 */
        .doc-dual-grid {{
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 24px;
            padding: 24px 36px;
            border-bottom: 1px solid var(--doc-border);
        }}

        .doc-chart-box {{
            background: #FFFFFF;
            border: 1px solid var(--doc-border);
            border-radius: 10px;
            padding: 18px 20px;
            display: flex;
            flex-direction: column;
            position: relative;
        }}

        .chart-box-header {{
            display: flex;
            justify-content: space-between;
            align-items: center;
            border-bottom: 1.5px solid var(--doc-navy-dark);
            padding-bottom: 8px;
            margin-bottom: 16px;
        }}

        .chart-box-title {{
            font-size: 0.85rem;
            font-weight: 800;
            color: var(--doc-navy-dark);
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }}

        .btn-export-png {{
            background: #F1F5F9;
            color: var(--doc-navy-dark);
            border: 1px solid var(--doc-border-dark);
            border-radius: 6px;
            padding: 4px 10px;
            font-size: 0.72rem;
            font-weight: 700;
            cursor: pointer;
            display: flex;
            align-items: center;
            gap: 4px;
            transition: all 0.2s ease;
        }}

        .btn-export-png:hover {{
            background: var(--doc-blue);
            color: #FFFFFF;
            border-color: var(--doc-blue);
        }}

        .chart-canvas-wrapper {{
            position: relative;
            width: 100%;
            height: 270px;
        }}

        /* Estilos para Tablas Cuantitativas */
        .doc-table {{
            width: 100%;
            border-collapse: collapse;
            font-size: 0.82rem;
            margin-top: 8px;
        }}

        .doc-table th {{
            background: #F8FAFC;
            color: var(--doc-navy-dark);
            font-weight: 700;
            text-transform: uppercase;
            font-size: 0.74rem;
            letter-spacing: 0.4px;
            padding: 8px 10px;
            text-align: left;
            border-top: 1.5px solid var(--doc-navy-dark);
            border-bottom: 1px solid var(--doc-border-dark);
        }}

        .doc-table td {{
            padding: 7px 10px;
            border-bottom: 1px solid var(--doc-border);
            color: #334155;
        }}

        .doc-table tr:hover td {{
            background-color: #F8FAFC;
        }}

        .ticker-pill {{
            font-family: 'JetBrains Mono', monospace;
            font-weight: 700;
            font-size: 0.75rem;
            background: var(--doc-pill-bg);
            color: var(--doc-navy-dark);
            padding: 2px 6px;
            border-radius: 4px;
        }}

        /* Fichas de Tesis por Activo */
        .doc-stocks-grid {{
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            gap: 16px;
            padding: 24px 36px;
            border-bottom: 1px solid var(--doc-border);
        }}

        .stock-card {{
            border: 1px solid var(--doc-border);
            border-radius: 10px;
            padding: 16px;
            background: #FFFFFF;
            display: flex;
            flex-direction: column;
            gap: 12px;
            box-shadow: 0 2px 6px rgba(0,0,0,0.02);
            transition: transform 0.2s ease, border-color 0.2s ease;
        }}

        .stock-card:hover {{
            border-color: var(--doc-blue);
            transform: translateY(-2px);
            box-shadow: var(--doc-shadow);
        }}

        .stock-card-header {{
            display: flex;
            justify-content: space-between;
            align-items: flex-start;
        }}

        .stock-name-block {{
            display: flex;
            flex-direction: column;
        }}

        .stock-title {{
            font-size: 1.05rem;
            font-weight: 800;
            color: var(--doc-navy-dark);
        }}

        .stock-role-tag {{
            font-size: 0.74rem;
            font-weight: 600;
            color: var(--doc-text-muted);
        }}

        .stock-weight-badge {{
            font-family: 'JetBrains Mono', monospace;
            font-size: 1.25rem;
            font-weight: 800;
            color: var(--doc-blue);
        }}

        .stock-metrics-row {{
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            gap: 6px;
            background: #F8FAFC;
            padding: 8px 10px;
            border-radius: 6px;
            border: 1px solid var(--doc-border);
            text-align: center;
        }}

        .sm-item {{
            display: flex;
            flex-direction: column;
        }}

        .sm-label {{
            font-size: 0.65rem;
            color: var(--doc-text-muted);
            font-weight: 700;
            text-transform: uppercase;
        }}

        .sm-val {{
            font-family: 'JetBrains Mono', monospace;
            font-size: 0.85rem;
            font-weight: 700;
            color: var(--doc-navy-dark);
            margin-top: 2px;
        }}

        .stock-thesis-text {{
            font-size: 0.8rem;
            line-height: 1.5;
            color: #475569;
            text-align: justify;
        }}

        /* Matriz de Correlación */
        .matrix-table {{
            width: 100%;
            border-collapse: collapse;
            font-family: 'JetBrains Mono', monospace;
            font-size: 0.78rem;
            text-align: center;
        }}

        .matrix-table th {{
            background: #F1F5F9;
            padding: 6px;
            font-weight: 700;
            color: var(--doc-navy-dark);
            border: 1px solid var(--doc-border);
        }}

        .matrix-table td {{
            padding: 6px;
            border: 1px solid #FFFFFF;
            font-weight: 700;
        }}

        .corr-cell-high {{
            background: #0062FF;
            color: #FFFFFF;
        }}

        .corr-cell-med {{
            background: #93C5FD;
            color: #0F172A;
        }}

        .corr-cell-low {{
            background: #E0F2FE;
            color: #0F172A;
        }}

        .corr-cell-diag {{
            background: #0A192F;
            color: #FFFFFF;
        }}

        /* Sección de Firmas y Equipo */
        .team-block-bar {{
            display: flex;
            justify-content: space-between;
            align-items: center;
            background: #FAFCFE;
            border: 1px solid var(--doc-border);
            border-radius: 10px;
            padding: 16px 24px;
            margin: 24px 36px 12px 36px;
        }}

        .team-title-sec {{
            font-size: 0.82rem;
            font-weight: 800;
            color: var(--doc-navy-dark);
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }}

        .team-members-grid {{
            display: flex;
            gap: 20px;
        }}

        .team-chip {{
            display: flex;
            align-items: center;
            gap: 10px;
        }}

        .team-avatar-pill {{
            width: 32px;
            height: 32px;
            border-radius: 50%;
            background: var(--doc-navy-dark);
            color: #FFFFFF;
            font-size: 0.75rem;
            font-weight: 800;
            display: flex;
            align-items: center;
            justify-content: center;
        }}

        .team-chip-info {{
            display: flex;
            flex-direction: column;
        }}

        .chip-name {{
            font-size: 0.82rem;
            font-weight: 700;
            color: var(--doc-navy-dark);
        }}

        .chip-role {{
            font-size: 0.72rem;
            color: var(--doc-text-muted);
        }}

        /* Disclaimer y Footer */
        .doc-footer-legal {{
            padding: 14px 36px 20px 36px;
            font-size: 0.7rem;
            color: var(--doc-text-muted);
            line-height: 1.5;
            text-align: justify;
            border-top: 1px solid var(--doc-border);
            background: #F8FAFC;
        }}

        /* Toast de notificación de descarga */
        .toast-notification {{
            position: fixed;
            bottom: 24px;
            right: 24px;
            background: var(--doc-navy-dark);
            color: #FFFFFF;
            padding: 12px 20px;
            border-radius: 8px;
            font-size: 0.85rem;
            font-weight: 600;
            box-shadow: 0 10px 25px rgba(0,0,0,0.2);
            border-left: 4px solid var(--doc-cyan);
            display: none;
            z-index: 9999;
            align-items: center;
            gap: 10px;
        }}

        /* Formato de Impresión */
        @media print {{
            .app-header-controls, .btn-export-png, .toast-notification {{
                display: none !important;
            }}
            body {{
                background: #FFFFFF !important;
                padding: 0 !important;
            }}
            .doc-page-container {{
                box-shadow: none !important;
                border: none !important;
                max-width: 100% !important;
            }}
        }}
    </style>
</head>
<body>

    <!-- Barra de Control Flotante -->
    <div class="app-header-controls">
        <div class="app-brand-summary">
            <span class="app-badge-tag">REPORTE INSTITUCIONAL</span>
            <span class="app-brand-title">Club de Finanzas UBA — Cartera Renta Variable</span>
        </div>
        <div class="app-header-actions">
            <button class="btn-header" onclick="descargarTodosLosGraficos()">
                📸 Descargar Todos los Gráficos (1920x1080)
            </button>
            <button class="btn-header btn-header-primary" onclick="window.print()">
                🖨 Guardar como PDF
            </button>
        </div>
    </div>

    <!-- Contenedor Principal / Hoja Institucional -->
    <div class="doc-page-container" id="reportePrincipal">

        <!-- Top Header Strip -->
        <div class="doc-header-strip">
            <div class="doc-header-left">
                {}
                <div class="doc-category-divider"></div>
                <div class="doc-category-text">REPORTE DE INVERSIÓN</div>
            </div>
            <div class="doc-page-badge">PÁGINA 1 DE 6</div>
        </div>

        <!-- Main Hero Header -->
        <div class="doc-main-hero">
            <div class="doc-hero-left">
                <h1 class="doc-main-title">Portfolio Renta Variable:</h1>
                <h2 class="doc-main-subtitle">Estrategia y Análisis de Activos</h2>
                <div class="doc-theme-badge-row">
                    <span>Energía e Infraestructura de IA</span>
                    <span>•</span>
                    <span>6 compañías</span>
                    <span>•</span>
                    <span>Agosto 2026</span>
                </div>
                <div class="doc-authors-line">
                    Autores: <strong>Fausto Crivelli</strong> (Presidente de Portafolio) • <strong>Luciano Mora</strong> (Analista Sr) • <strong>Florencia Beluzzo</strong> (Analista Sr)
                </div>
            </div>
            <svg class="doc-hero-decor" viewBox="0 0 140 48" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M5 38 L45 28 L85 34 L130 10" stroke="#0062FF" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
                <circle cx="5" cy="38" r="3" fill="#0A192F"/>
                <circle cx="45" cy="28" r="3" fill="#0A192F"/>
                <circle cx="85" cy="34" r="3" fill="#0A192F"/>
                <polygon points="135,10 126,6 128,15" fill="#0062FF"/>
            </svg>
        </div>

        <!-- Introducción de la Estrategia -->
        <div class="doc-intro-section">
            <div class="doc-section-title-inline">
                <span>📘 INTRODUCCIÓN & DE LA ESTRATEGIA A LA CARTERA</span>
            </div>
            <p class="doc-intro-p">
                En nuestra Estrategia 2026 dimos conocimiento de que la expansión de la inteligencia artificial había dejado de ser un fenómeno exclusivo del sector tecnológico para consolidarse como un ciclo de inversión real, y que los cuellos de botella no estarían en el software sino en la <strong>infraestructura física: hardware especializado y, sobre todo, energía</strong>.
            </p>
            <p class="doc-intro-p">
                En este contexto, Bloomberg NEF proyecta que la red eléctrica estadounidense quedará 19 GW corta frente a la demanda de los centros de datos hacia 2035. Este informe presenta el vehículo concreto de aquella tesis: una cartera de seis compañías que capturan el mismo fenómeno desde ángulos económicos distintos.
            </p>
            <div class="doc-profile-callout">
                <strong>Perfil sugerido:</strong> Cartera 100% renta variable en USD, destinada a un perfil <strong>MODERADO</strong> con un beta de <strong>1,011</strong> y un horizonte temporal de <strong>2 a 3 años</strong>.
            </div>
        </div>

        <!-- Grid de KPIs Cuantitativos -->
        <div class="doc-kpis-grid">
            <div class="doc-kpi-card">
                <div class="kpi-icon-pill">📈</div>
                <div class="kpi-label">Retorno Anual (CAGR)</div>
                <div class="kpi-value">33,0%</div>
            </div>
            <div class="doc-kpi-card">
                <div class="kpi-icon-pill blue">⚡</div>
                <div class="kpi-label">Volatilidad Anualizada</div>
                <div class="kpi-value">18,4%</div>
            </div>
            <div class="doc-kpi-card">
                <div class="kpi-icon-pill">🎯</div>
                <div class="kpi-label">Ratio de Sharpe</div>
                <div class="kpi-value dark">1,54</div>
            </div>
            <div class="doc-kpi-card">
                <div class="kpi-icon-pill blue">β</div>
                <div class="kpi-label">Beta vs Benchmark</div>
                <div class="kpi-value">0,56 / 0,65</div>
            </div>
            <div class="doc-kpi-card">
                <div class="kpi-icon-pill cyan">β</div>
                <div class="kpi-label">Beta Cartera vs Mercado</div>
                <div class="kpi-value">1,011</div>
            </div>
            <div class="doc-kpi-card">
                <div class="kpi-icon-pill">📉</div>
                <div class="kpi-label">Máximo Drawdown</div>
                <div class="kpi-value dark">-17,0%</div>
            </div>
            <div class="doc-kpi-card">
                <div class="kpi-icon-pill blue">🔄</div>
                <div class="kpi-label">Correlación Promedio</div>
                <div class="kpi-value">0,25</div>
            </div>
        </div>

        <!-- Gráficos Fila 1: Asignación del Portafolio & Naturaleza de Riesgo -->
        <div class="doc-dual-grid">
            <!-- Box 1: Asignación Donut -->
            <div class="doc-chart-box" id="boxChartAsignacion">
                <div class="chart-box-header">
                    <span class="chart-box-title">ASIGNACIÓN DEL PORTAFOLIO</span>
                    <button class="btn-export-png" onclick="exportarGraficoA1080('boxChartAsignacion', 'Asignacion_Portafolio_1080p')">
                        📸 Exportar PNG (1920x1080)
                    </button>
                </div>
                <div class="chart-canvas-wrapper">
                    <canvas id="chartAsignacion"></canvas>
                </div>
            </div>

            <!-- Box 2: Naturaleza de Riesgo Barras -->
            <div class="doc-chart-box" id="boxChartRiesgo">
                <div class="chart-box-header">
                    <span class="chart-box-title">ASIGNACIÓN POR NATURALEZA DE RIESGO</span>
                    <button class="btn-export-png" onclick="exportarGraficoA1080('boxChartRiesgo', 'Naturaleza_Riesgo_1080p')">
                        📸 Exportar PNG (1920x1080)
                    </button>
                </div>
                <div class="chart-canvas-wrapper">
                    <canvas id="chartRiesgo"></canvas>
                </div>
            </div>
        </div>

        <!-- Gráficos Fila 2: Perfil Riesgo - Retorno & Frontera Eficiente -->
        <div class="doc-dual-grid">
            <!-- Box 3: Scatter Riesgo-Retorno -->
            <div class="doc-chart-box" id="boxChartScatter">
                <div class="chart-box-header">
                    <span class="chart-box-title">PERFIL RIESGO - RETORNO COMPARATIVO</span>
                    <button class="btn-export-png" onclick="exportarGraficoA1080('boxChartScatter', 'Perfil_Riesgo_Retorno_1080p')">
                        📸 Exportar PNG (1920x1080)
                    </button>
                </div>
                <div class="chart-canvas-wrapper">
                    <canvas id="chartScatter"></canvas>
                </div>
            </div>

            <!-- Box 4: Frontera Eficiente -->
            <div class="doc-chart-box" id="boxChartFrontera">
                <div class="chart-box-header">
                    <span class="chart-box-title">FRONTERA EFICIENTE & MÁXIMO SHARPE</span>
                    <button class="btn-export-png" onclick="exportarGraficoA1080('boxChartFrontera', 'Frontera_Eficiente_1080p')">
                        📸 Exportar PNG (1920x1080)
                    </button>
                </div>
                <div class="chart-canvas-wrapper">
                    <canvas id="chartFrontera"></canvas>
                </div>
            </div>
        </div>

        <!-- Gráficos Fila 3: Matriz de Correlación Exacta & Métricas de Cola -->
        <div class="doc-dual-grid">
            <!-- Box 5: Heatmap Matriz -->
            <div class="doc-chart-box" id="boxChartMatriz">
                <div class="chart-box-header">
                    <span class="chart-box-title">MATRIZ DE CORRELACIÓN EXACTA (SECCIÓN 6)</span>
                    <button class="btn-export-png" onclick="exportarGraficoA1080('boxChartMatriz', 'Matriz_Correlacion_1080p')">
                        📸 Exportar PNG (1920x1080)
                    </button>
                </div>
                <table class="matrix-table" style="margin-top: 14px;">
                    <tr><th></th><th>XOM</th><th>SO</th><th>VST</th><th>ETN</th><th>YPF</th><th>WMB</th></tr>
                    <tr><th>XOM</th><td class="corr-cell-diag">1,00</td><td class="corr-cell-low">0,18</td><td class="corr-cell-low">0,14</td><td class="corr-cell-low">0,12</td><td class="corr-cell-med">0,37</td><td class="corr-cell-high">0,52</td></tr>
                    <tr><th>SO</th><td class="corr-cell-low">0,18</td><td class="corr-cell-diag">1,00</td><td class="corr-cell-low">0,10</td><td class="corr-cell-low">0,03</td><td class="corr-cell-low">0,06</td><td class="corr-cell-med">0,32</td></tr>
                    <tr><th>VST</th><td class="corr-cell-low">0,14</td><td class="corr-cell-low">0,10</td><td class="corr-cell-diag">1,00</td><td class="corr-cell-high">0,53</td><td class="corr-cell-low">0,21</td><td class="corr-cell-med">0,37</td></tr>
                    <tr><th>ETN</th><td class="corr-cell-low">0,12</td><td class="corr-cell-low">0,03</td><td class="corr-cell-high">0,53</td><td class="corr-cell-diag">1,00</td><td class="corr-cell-low">0,23</td><td class="corr-cell-med">0,31</td></tr>
                    <tr><th>YPF</th><td class="corr-cell-med">0,37</td><td class="corr-cell-low">0,06</td><td class="corr-cell-low">0,21</td><td class="corr-cell-low">0,23</td><td class="corr-cell-diag">1,00</td><td class="corr-cell-low">0,28</td></tr>
                    <tr><th>WMB</th><td class="corr-cell-high">0,52</td><td class="corr-cell-med">0,32</td><td class="corr-cell-med">0,37</td><td class="corr-cell-med">0,31</td><td class="corr-cell-low">0,28</td><td class="corr-cell-diag">1,00</td></tr>
                </table>
                <div style="font-size: 0.75rem; color: var(--doc-text-muted); margin-top: 12px; font-weight: 600;">
                    Correlación promedio: <strong>0,25</strong> • Par más elevado: <strong>0,53 (VST-ETN)</strong> • Ningún par supera 0,60
                </div>
            </div>

            <!-- Box 6: Métricas de Cola & Validación IS/OOS -->
            <div class="doc-chart-box" id="boxChartCola">
                <div class="chart-box-header">
                    <span class="chart-box-title">RIESGO CONDICIONAL & MÉTRICAS DE COLA</span>
                    <button class="btn-export-png" onclick="exportarGraficoA1080('boxChartCola', 'Metricas_Cola_1080p')">
                        📸 Exportar PNG (1920x1080)
                    </button>
                </div>
                <table class="doc-table">
                    <tr><th>Métrica de Riesgo de Cola</th><th>Valor</th><th>Lectura Estadística</th></tr>
                    <tr><td><strong>VaR 99% 1D (Paramétrico)</strong></td><td><span class="ticker-pill">2,69%</span></td><td>Supone distribución normal estándar.</td></tr>
                    <tr><td><strong>VaR 99% 1D (Histórico)</strong></td><td><span class="ticker-pill">2,93%</span></td><td>Captura colas pesadas de la distribución real.</td></tr>
                    <tr><td><strong>CVaR (Expected Shortfall)</strong></td><td><span class="ticker-pill">4,10%</span></td><td>Pérdida esperada al superar el VaR.</td></tr>
                </table>
                <div style="font-size: 0.74rem; color: var(--doc-text-muted); margin-top: 10px; line-height: 1.4;">
                    Estrés VST – ETN: En el 90% de las ruedas la correlación es 0,40; en el decil de peores ruedas salta a 0,59 (+48%). Justifica el límite conjunto de 20%.
                </div>
            </div>
        </div>

        <!-- Fichas de Tesis por Activo -->
        <div class="doc-stocks-grid">
            <!-- Eaton -->
            <div class="stock-card">
                <div class="stock-card-header">
                    <div class="stock-name-block">
                        <span class="stock-title">Eaton Corp. (ETN)</span>
                        <span class="stock-role-tag">Equipamiento Eléctrico • Picks & Shovels</span>
                    </div>
                    <span class="stock-weight-badge">10,0%</span>
                </div>
                <div class="stock-metrics-row">
                    <div class="sm-item"><span class="sm-label">BPA '26</span><span class="sm-val">$13,40–13,60</span></div>
                    <div class="sm-item"><span class="sm-label">Backlog</span><span class="sm-val">307 GW</span></div>
                    <div class="sm-item"><span class="sm-label">Datacenter</span><span class="sm-val">+65% org.</span></div>
                </div>
                <p class="stock-thesis-text">
                    El proveedor, no el apostador: Fabrica transformadores, tableros y distribución para datacenters. Backlog para datacenters en EE.UU. de 307 GW (15 años de trabajo asegurado). R² de 0,055 vs XLE y 0,087 vs XLU.
                </p>
            </div>

            <!-- Vistra -->
            <div class="stock-card">
                <div class="stock-card-header">
                    <div class="stock-name-block">
                        <span class="stock-title">Vistra Corp. (VST)</span>
                        <span class="stock-role-tag">Generación Merchant • Convexidad Nuclear</span>
                    </div>
                    <span class="stock-weight-badge">10,0%</span>
                </div>
                <div class="stock-metrics-row">
                    <div class="sm-item"><span class="sm-label">EBITDA '26</span><span class="sm-val">$6,8–7,6 B</span></div>
                    <div class="sm-item"><span class="sm-label">FCF Libre</span><span class="sm-val">$3,9–4,7 B</span></div>
                    <div class="sm-item"><span class="sm-label">PPA Amazon</span><span class="sm-val">1.200 MW</span></div>
                </div>
                <p class="stock-thesis-text">
                    Apalancamiento puro a la demanda eléctrica 24/7 con flota nuclear despachable (Comanche Peak y PJM). Contratos PPA a 20 años por 1.200 MW con AWS. En Q2 el EBITDA creció 30% con disponibilidad >97%.
                </p>
            </div>

            <!-- YPF -->
            <div class="stock-card">
                <div class="stock-card-header">
                    <div class="stock-name-block">
                        <span class="stock-title">YPF S.A. (YPF)</span>
                        <span class="stock-role-tag">Upstream Argentina • Opción Asimétrica</span>
                    </div>
                    <span class="stock-weight-badge">10,0%</span>
                </div>
                <div class="stock-metrics-row">
                    <div class="sm-item"><span class="sm-label">Lifting Cost</span><span class="sm-val">$4,0 / bbl</span></div>
                    <div class="sm-item"><span class="sm-label">Shale 2027</span><span class="sm-val">290k bpd</span></div>
                    <div class="sm-item"><span class="sm-label">EV/EBITDA</span><span class="sm-val">6,0–7,4x</span></div>
                </div>
                <p class="stock-thesis-text">
                    Transformación operativa en Vaca Muerta: reducción del costo de extracción a $4,0/bbl. El Oleoducto VMOS en ene/27 habilita exportaciones a precios internacionales. EBITDA '26 de USD 5.800–6.200M.
                </p>
            </div>

            <!-- ExxonMobil -->
            <div class="stock-card">
                <div class="stock-card-header">
                    <div class="stock-name-block">
                        <span class="stock-title">ExxonMobil (XOM)</span>
                        <span class="stock-role-tag">O&G Integrada • El Ancla de Caja</span>
                    </div>
                    <span class="stock-weight-badge">25,0%</span>
                </div>
                <div class="stock-metrics-row">
                    <div class="sm-item"><span class="sm-label">BPA 2026</span><span class="sm-val">$10,11</span></div>
                    <div class="sm-item"><span class="sm-label">FCF Increm.</span><span class="sm-val">+$35.000M</span></div>
                    <div class="sm-item"><span class="sm-label">Div. Yield</span><span class="sm-val">2,69%</span></div>
                </div>
                <p class="stock-thesis-text">
                    Escala global y flujo predecible: ventaja geológica en Stabroek Guyana y Permian (>1,8M boed con Pioneer). Plan '26-'30 apunta a USD 35.000M de FCF incremental. Reduce la volatilidad agregada.
                </p>
            </div>

            <!-- Southern Co -->
            <div class="stock-card">
                <div class="stock-card-header">
                    <div class="stock-name-block">
                        <span class="stock-title">Southern Co. (SO)</span>
                        <span class="stock-role-tag">Utility Regulada • El Estabilizador</span>
                    </div>
                    <span class="stock-weight-badge">25,0%</span>
                </div>
                <div class="stock-metrics-row">
                    <div class="sm-item"><span class="sm-label">BPA 2026</span><span class="sm-val">$4,50–4,60</span></div>
                    <div class="sm-item"><span class="sm-label">Datacenter</span><span class="sm-val">10 GW contr.</span></div>
                    <div class="sm-item"><span class="sm-label">Div. Yield</span><span class="sm-val">3,22%</span></div>
                </div>
                <p class="stock-thesis-text">
                    Retorno regulado y líder nuclear en EE.UU.: opera Plant Vogtle 3 y 4. Cuenta con 10 GW contratados con datacenters y embudo de 75 GW. Correlaciona negativamente con el capex de IA cuando la tecnología corrige.
                </p>
            </div>

            <!-- Williams Cos -->
            <div class="stock-card">
                <div class="stock-card-header">
                    <div class="stock-name-block">
                        <span class="stock-title">Williams Cos. (WMB)</span>
                        <span class="stock-role-tag">Midstream Gas • Peaje + Sócrates</span>
                    </div>
                    <span class="stock-weight-badge">20,0%</span>
                </div>
                <div class="stock-metrics-row">
                    <div class="sm-item"><span class="sm-label">EBITDA CAGR</span><span class="sm-val">>11% anual</span></div>
                    <div class="sm-item"><span class="sm-label">Blackstone</span><span class="sm-val">$5.340M</span></div>
                    <div class="sm-item"><span class="sm-label">Div. Yield</span><span class="sm-val">2,91%</span></div>
                </div>
                <p class="stock-thesis-text">
                    Infraestructura irreproducible convertida en generador: Red Transco es un monopolio natural con contratos take-or-pay. Construye generación on-site para centros de datos (Proyecto Sócrates 200 MW).
                </p>
            </div>
        </div>

        <!-- Tabla Comparativa vs Benchmarks -->
        <div style="padding: 20px 36px;">
            <div style="font-size: 0.85rem; font-weight: 800; color: var(--doc-navy-dark); text-transform: uppercase; margin-bottom: 10px;">
                RESULTADOS DE LA CARTERA VS BENCHMARKS (TABLA OFICIAL SECCIÓN 6)
            </div>
            <table class="doc-table">
                <tr>
                    <th>MÉTRICA</th>
                    <th style="background: #EBF3FA; color: #0062FF;">CARTERA</th>
                    <th>XLE (ENERGÍA)</th>
                    <th>XLU (UTILITIES)</th>
                    <th>50/50 BLEND</th>
                </tr>
                <tr>
                    <td><strong>Rendimiento anual compuesto (CAGR)</strong></td>
                    <td style="background: #EBF3FA; font-weight: 800; color: #0062FF;">33,0%</td>
                    <td>26,6%</td>
                    <td>11,3%</td>
                    <td>19,8%</td>
                </tr>
                <tr>
                    <td><strong>Volatilidad anualizada</strong></td>
                    <td style="background: #EBF3FA; font-weight: 800; color: #0062FF;">18,4%</td>
                    <td>25,8%</td>
                    <td>17,3%</td>
                    <td>18,9%</td>
                </tr>
                <tr>
                    <td><strong>Ratio de Sharpe</strong></td>
                    <td style="background: #EBF3FA; font-weight: 800; color: #0062FF;">1,54</td>
                    <td>0,85</td>
                    <td>0,38</td>
                    <td>0,80</td>
                </tr>
                <tr>
                    <td><strong>Máxima caída pico-a-valle (Drawdown)</strong></td>
                    <td style="background: #EBF3FA; font-weight: 800; color: #0062FF;">-17,0%</td>
                    <td>-25,8%</td>
                    <td>—</td>
                    <td>-16,8%</td>
                </tr>
                <tr>
                    <td><strong>Beta vs Benchmark</strong></td>
                    <td style="background: #EBF3FA; font-weight: 800; color: #0062FF;">—</td>
                    <td>0,56</td>
                    <td>0,65</td>
                    <td>—</td>
                </tr>
            </table>
        </div>

        <!-- Equipo de Portafolio -->
        <div class="team-block-bar">
            <span class="team-title-sec">EQUIPO DE PORTAFOLIO — CLUB DE FINANZAS UBA</span>
            <div class="team-members-grid">
                <div class="team-chip">
                    <div class="team-avatar-pill">FC</div>
                    <div class="team-chip-info">
                        <span class="chip-name">Fausto Crivelli</span>
                        <span class="chip-role">Presidente de Portafolio</span>
                    </div>
                </div>
                <div class="team-chip">
                    <div class="team-avatar-pill">LM</div>
                    <div class="team-chip-info">
                        <span class="chip-name">Luciano Mora</span>
                        <span class="chip-role">Analista Sr</span>
                    </div>
                </div>
                <div class="team-chip">
                    <div class="team-avatar-pill">FB</div>
                    <div class="team-chip-info">
                        <span class="chip-name">Florencia Beluzzo</span>
                        <span class="chip-role">Analista Sr</span>
                    </div>
                </div>
            </div>
        </div>

        <!-- Disclaimer Institucional -->
        <div class="doc-footer-legal">
            <strong>Disclaimer:</strong> El presente informe tiene finalidad exclusivamente informativa y educativa. No constituye asesoramiento de inversión, recomendación de compra o venta, ni oferta de servicios financieros. Los rendimientos pasados no garantizan resultados futuros. Las cifras de desempeño corresponden a un ejercicio de simulación sobre datos históricos y no representan resultados efectivamente obtenidos por inversores. Los análisis se basan en información pública disponible a la fecha de publicación, cuya exactitud no puede garantizarse. Toda decisión de inversión debe adoptarse considerando la situación patrimonial, fiscal y de horizonte temporal de cada inversor, preferentemente con asesoramiento profesional.
        </div>

    </div>

    <!-- Toast Notification -->
    <div class="toast-notification" id="toastExport">
        <span>📸 Gráfico exportado con éxito a 1920x1080 PNG</span>
    </div>

    <script>
        // Inicialización de Gráficos con Chart.js
        document.addEventListener('DOMContentLoaded', () => {{
            // 1. Asignación Donut Chart
            const ctxAsignacion = document.getElementById('chartAsignacion').getContext('2d');
            new Chart(ctxAsignacion, {{
                type: 'doughnut',
                data: {{
                    labels: ['ExxonMobil (XOM)', 'Southern Co. (SO)', 'Williams Cos. (WMB)', 'Eaton Corp. (ETN)', 'Vistra Corp. (VST)', 'YPF S.A. (YPF)'],
                    datasets: [{{
                        data: [25, 25, 20, 10, 10, 10],
                        backgroundColor: ['#0A192F', '#0062FF', '#0284C7', '#38BDF8', '#7DD3FC', '#0F766E'],
                        borderWidth: 2,
                        borderColor: '#FFFFFF'
                    }}]
                }},
                options: {{
                    responsive: true,
                    maintainAspectRatio: false,
                    plugins: {{
                        legend: {{
                            position: 'right',
                            labels: {{
                                font: {{ family: 'Plus Jakarta Sans', size: 11, weight: '600' }},
                                color: '#0F172A',
                                padding: 12
                            }}
                        }}
                    }},
                    cutout: '62%'
                }}
            }});

            // 2. Naturaleza de Riesgo Bar Chart Horizontal
            const ctxRiesgo = document.getElementById('chartRiesgo').getContext('2d');
            new Chart(ctxRiesgo, {{
                type: 'bar',
                data: {{
                    labels: ['Ingreso Regulado/Peaje', 'Exposición al Crudo', 'Ciclo de Inversión IA', 'Utility Regulada (SO)', 'O&G Integrada (XOM)', 'Midstream Gas (WMB)'],
                    datasets: [{{
                        data: [45.0, 35.0, 20.0, 25.0, 25.0, 20.0],
                        backgroundColor: ['#0A192F', '#0062FF', '#38BDF8', '#0284C7', '#14B8A6', '#0D9488'],
                        borderRadius: 6
                    }}]
                }},
                options: {{
                    indexAxis: 'y',
                    responsive: true,
                    maintainAspectRatio: false,
                    plugins: {{
                        legend: {{ display: false }}
                    }},
                    scales: {{
                        x: {{
                            max: 50,
                            ticks: {{
                                callback: (val) => val + '%',
                                font: {{ family: 'JetBrains Mono', size: 10 }},
                                color: '#64748B'
                            }},
                            grid: {{ color: '#F1F5F9' }}
                        }},
                        y: {{
                            ticks: {{
                                font: {{ family: 'Plus Jakarta Sans', size: 11, weight: '600' }},
                                color: '#0F172A'
                            }},
                            grid: {{ display: false }}
                        }}
                    }}
                }}
            }});

            // 3. Scatter Plot Riesgo-Retorno
            const ctxScatter = document.getElementById('chartScatter').getContext('2d');
            new Chart(ctxScatter, {{
                type: 'scatter',
                data: {{
                    datasets: [
                        {{
                            label: 'CARTERA (33.0%, 18.4%)',
                            data: [{{ x: 18.4, y: 33.0 }}],
                            backgroundColor: '#0062FF',
                            pointRadius: 9,
                            pointHoverRadius: 11
                        }},
                        {{
                            label: 'XLE (25.8%, 26.6%)',
                            data: [{{ x: 25.8, y: 26.6 }}],
                            backgroundColor: '#0A192F',
                            pointRadius: 6
                        }},
                        {{
                            label: '50/50 Blend (18.9%, 19.8%)',
                            data: [{{ x: 18.9, y: 19.8 }}],
                            backgroundColor: '#64748B',
                            pointRadius: 6
                        }},
                        {{
                            label: 'XLU (17.3%, 11.3%)',
                            data: [{{ x: 17.3, y: 11.3 }}],
                            backgroundColor: '#94A3B8',
                            pointRadius: 6
                        }}
                    ]
                }},
                options: {{
                    responsive: true,
                    maintainAspectRatio: false,
                    plugins: {{
                        legend: {{
                            position: 'bottom',
                            labels: {{ font: {{ family: 'Plus Jakarta Sans', size: 10, weight: '600' }} }}
                        }}
                    }},
                    scales: {{
                        x: {{
                            title: {{ display: true, text: 'Volatilidad Anualizada (%)', font: {{ weight: '700' }} }},
                            min: 14,
                            max: 30,
                            ticks: {{ callback: (v) => v + '%' }}
                        }},
                        y: {{
                            title: {{ display: true, text: 'CAGR Retorno (%)', font: {{ weight: '700' }} }},
                            min: 8,
                            max: 38,
                            ticks: {{ callback: (v) => v + '%' }}
                        }}
                    }}
                }}
            }});

            // 4. Frontera Eficiente & Máximo Sharpe
            const ctxFrontera = document.getElementById('chartFrontera').getContext('2d');
            new Chart(ctxFrontera, {{
                type: 'line',
                data: {{
                    labels: ['16%', '18%', '20%', '24%', '28%', '32%', '36%'],
                    datasets: [
                        {{
                            label: 'Frontera de Inversión',
                            data: [18.2, 33.0, 34.5, 36.8, 38.2, 39.1, 39.8],
                            borderColor: '#0062FF',
                            borderWidth: 3,
                            fill: false,
                            tension: 0.35,
                            pointBackgroundColor: '#0062FF',
                            pointRadius: [3, 8, 3, 3, 3, 3, 3]
                        }}
                    ]
                }},
                options: {{
                    responsive: true,
                    maintainAspectRatio: false,
                    plugins: {{
                        legend: {{ display: false }}
                    }},
                    scales: {{
                        x: {{
                            title: {{ display: true, text: 'Volatilidad Anualizada σ (%)', font: {{ weight: '700' }} }}
                        }},
                        y: {{
                            title: {{ display: true, text: 'Retorno (%)', font: {{ weight: '700' }} }},
                            min: 10,
                            max: 42
                        }}
                    }}
                }}
            }});
        }});

        // Función para exportar un gráfico específico a 1920x1080 PNG con fondo y marco institucional
        function exportarGraficoA1080(elementId, fileName) {{
            const element = document.getElementById(elementId);
            if (!element) return;

            const btn = element.querySelector('.btn-export-png');
            if (btn) btn.style.visibility = 'hidden';

            html2canvas(element, {{
                scale: 3.5,
                backgroundColor: '#FFFFFF',
                logging: false,
                useCORS: true
            }}).then(canvas => {{
                if (btn) btn.style.visibility = 'visible';

                const targetCanvas = document.createElement('canvas');
                targetCanvas.width = 1920;
                targetCanvas.height = 1080;
                const ctx = targetCanvas.getContext('2d');

                const grad = ctx.createLinearGradient(0, 0, 1920, 1080);
                grad.addColorStop(0, '#FFFFFF');
                grad.addColorStop(1, '#F8FAFC');
                ctx.fillStyle = grad;
                ctx.fillRect(0, 0, 1920, 1080);

                ctx.fillStyle = '#0A192F';
                ctx.fillRect(0, 0, 1920, 80);

                ctx.fillStyle = '#FFFFFF';
                ctx.font = 'bold 28px Plus Jakarta Sans, sans-serif';
                ctx.fillText('CLUB DE FINANZAS UBA', 60, 52);

                ctx.fillStyle = '#00D2D3';
                ctx.font = 'bold 20px Plus Jakarta Sans, sans-serif';
                ctx.fillText('|  PORTFOLIO RENTA VARIABLE & ANÁLISIS CUANTITATIVO', 430, 50);

                ctx.strokeStyle = '#E2E8F0';
                ctx.lineWidth = 4;
                ctx.strokeRect(40, 120, 1840, 900);

                const scaleFactor = Math.min(1760 / canvas.width, 820 / canvas.height);
                const drawW = canvas.width * scaleFactor;
                const drawH = canvas.height * scaleFactor;
                const drawX = (1920 - drawW) / 2;
                const drawY = 140 + (840 - drawH) / 2;

                ctx.drawImage(canvas, drawX, drawY, drawW, drawH);

                ctx.fillStyle = '#64748B';
                ctx.font = '16px Plus Jakarta Sans, sans-serif';
                ctx.fillText('Reporte Oficial de Inversión — Club de Finanzas UBA | Resolución 1920 x 1080 HD', 60, 1055);

                const link = document.createElement('a');
                link.download = fileName + '.png';
                link.href = targetCanvas.toDataURL('image/png', 1.0);
                link.click();

                mostrarToast();
            }}).catch(err => {{
                if (btn) btn.style.visibility = 'visible';
                console.error('Error exportando imagen:', err);
            }});
        }}

        function descargarTodosLosGraficos() {{
            const boxes = [
                {{ id: 'boxChartAsignacion', name: '1_Asignacion_Portafolio_1080p' }},
                {{ id: 'boxChartRiesgo', name: '2_Naturaleza_Riesgo_1080p' }},
                {{ id: 'boxChartScatter', name: '3_Perfil_Riesgo_Retorno_1080p' }},
                {{ id: 'boxChartFrontera', name: '4_Frontera_Eficiente_1080p' }},
                {{ id: 'boxChartMatriz', name: '5_Matriz_Correlacion_1080p' }},
                {{ id: 'boxChartCola', name: '6_Metricas_Riesgo_Cola_1080p' }}
            ];

            let delay = 0;
            boxes.forEach(item => {{
                setTimeout(() => {{
                    exportarGraficoA1080(item.id, item.name);
                }}, delay);
                delay += 800;
            }});
        }}

        function mostrarToast() {{
            const toast = document.getElementById('toastExport');
            toast.style.display = 'flex';
            setTimeout(() => {{
                toast.style.display = 'none';
            }}, 3500);
        }}
    </script>
</body>
</html>
"##,
        logo_img_html
    )
}

fn escape_html(input: &str) -> String {
    input
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
}

fn encode_base64(data: &[u8]) -> String {
    const CHARSET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    let mut i = 0;
    while i < data.len() {
        let b0 = data[i];
        let b1 = if i + 1 < data.len() { data[i + 1] } else { 0 };
        let b2 = if i + 2 < data.len() { data[i + 2] } else { 0 };
        let n = ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32);
        out.push(CHARSET[((n >> 18) & 63) as usize] as char);
        out.push(CHARSET[((n >> 12) & 63) as usize] as char);
        if i + 1 < data.len() {
            out.push(CHARSET[((n >> 6) & 63) as usize] as char);
        } else {
            out.push('=');
        }
        if i + 2 < data.len() {
            out.push(CHARSET[(n & 63) as usize] as char);
        } else {
            out.push('=');
        }
        i += 3;
    }
    out
}
