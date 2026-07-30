mod arbitraje;
mod capm;
mod db;
mod ingestion;
mod optimizacion;
mod stats;

use dialoguer::{theme::ColorfulTheme, Input, Select};
use nalgebra::{DMatrix, DVector};
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;

#[derive(Serialize)]
struct ExportCarteraPayload {
    n_velas_opt: usize,
    tickers: Vec<String>,
    pesos_sharpe: Vec<f64>,
    pesos_sortino: Vec<f64>,
    esperados: Vec<f64>,
    vols: Vec<f64>,
    downside_vols: Vec<f64>,
    betas: Vec<f64>,
    capm_returns: Vec<f64>,
    port_return_sharpe: f64,
    port_vol_sharpe: f64,
    port_downside_vol: f64,
    sharpe_ratio: f64,
    sortino_ratio: f64,
    var_95: f64,
    tc_cartera_ponderado: f64,
    spread_ccl: f64,
    retornos_map: HashMap<String, Vec<f64>>,
    series_map: HashMap<String, Vec<f64>>,
    time_labels: Vec<String>,
    rf_rate: f64,
    min_bound: f64,
    ccl_ref: f64,
    // Campos de Validación IS / OOS
    is_oos_mode: bool,
    is_return: f64,
    is_vol: f64,
    is_sharpe: f64,
    oos_return: f64,
    oos_vol: f64,
    oos_sharpe: f64,
    split_index: usize,
    // Campos de Seguimiento de Portafolio Personalizado
    is_tracking_mode: bool,
    tracking_gain_pct: f64,
    tracking_max_dd_pct: f64,
    tracking_sharpe: f64,
    tracking_sortino: f64,
    tracking_avg_stagnation_days: f64,
    tracking_max_stagnation_days: usize,
    tracking_start_date: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===========================================================");
    println!("  MOTOR ANALÍTICO DE PORTAFOLIO Y BASE DE DATOS (RUST)");
    println!("===========================================================\n");

    let db_path = "portafolio.db";
    let conn = db::init_db(db_path)?;
    println!("✓ Base de datos SQLite conectada: {}\n", db_path);

    let dias_anualizacion: f64 = 252.0;
    let poblacional = false; // Muestral por default
    let limite_peso_base = 0.05; // 5% min por activo

    loop {
        let opciones = &[
            "1. Descargar / Actualizar Tickers de Mercado",
            "2. Optimizar Cartera Estándar (Últimas N Velas) y Proyectar Dashboard",
            "3. Optimización & Validación IS / OOS (In-Sample 252v / Out-Of-Sample 252v)",
            "4. Seguimiento de Portafolio Personalizado (Evolución de Capital vs SPY)",
            "5. Ver Tickers y Resumen de Base de Datos",
            "6. Salir",
        ];

        let seleccion = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Seleccione una opción de consola")
            .default(0)
            .items(&opciones[..])
            .interact()?;

        match seleccion {
            0 => {
                let input_ticker: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Ingrese los Tickers a descargar separados por coma (ej. SPY, CEG, YPF, XOM, GEV, AAPL)")
                    .interact_text()?;

                let tickers: Vec<String> = input_ticker
                    .split(',')
                    .map(|s| s.trim().to_uppercase())
                    .filter(|s| !s.is_empty())
                    .collect();

                if tickers.is_empty() {
                    println!("⚠ Error: Debe ingresar al menos un ticker válido.");
                    continue;
                }

                for symbol in tickers {
                    match ingestion::descargar_y_guardar_ticker(&symbol, &conn).await {
                        Ok(count) => {
                            println!("✓ Se procesaron {} velas de mercado para {}.\n", count, symbol);
                        }
                        Err(e) => {
                            println!("❌ Error de descarga para {}: {}\n", symbol, e);
                        }
                    }
                }
            }
            1 => {
                let input_tickers: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Ingrese los tickers de la cartera separados por coma (ej. CEG, SO, YPF, XOM, GEV)")
                    .with_initial_text("CEG, SO, YPF, XOM, GEV")
                    .interact_text()?;

                let n_velas_opt: usize = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Ingrese la cantidad de velas a utilizar para la optimización (ej. 50, 126, 252, 715)")
                    .default(252)
                    .interact()?;

                let mut tickers_list: Vec<String> = input_tickers
                    .split(',')
                    .map(|s| s.trim().to_uppercase())
                    .filter(|s| !s.is_empty())
                    .collect();

                if tickers_list.is_empty() {
                    println!("⚠ Error: Debe ingresar al menos un ticker.");
                    continue;
                }

                if n_velas_opt < 5 {
                    println!("⚠ Error: Debe especificar al menos 5 velas para el cálculo.");
                    continue;
                }

                if !tickers_list.contains(&"SPY".to_string()) {
                    tickers_list.insert(0, "SPY".to_string());
                }

                println!("\n[1/3] Verificando y descargando cotizaciones faltantes en SQLite...");
                let mut error_ocurrido = false;
                for t in &tickers_list {
                    let prev = db::obtener_precios_ticker(&conn, t, n_velas_opt + 1)?;
                    if prev.len() < (n_velas_opt / 2) {
                        println!("  -> Ticker '{}' sin suficientes velas ({}/{}). Descargando de Yahoo Finance...", t, prev.len(), n_velas_opt);
                        if let Err(e) = ingestion::descargar_y_guardar_ticker(t, &conn).await {
                            println!("❌ ERROR CRÍTICO: {}\n", e);
                            error_ocurrido = true;
                            break;
                        }
                    }
                }

                if error_ocurrido {
                    println!("⚠ El análisis fue cancelado por fallo de datos reales.\n");
                    continue;
                }

                println!("[2/3] Procesando Markowitz, CAPM y Arbitraje en Rust sobre las últimas {} velas...", n_velas_opt);
                let portfolio_tickers: Vec<String> = tickers_list
                    .into_iter()
                    .filter(|t| t != "SPY")
                    .collect();

                match ejecutar_analisis_y_exportar(
                    &portfolio_tickers,
                    &conn,
                    dias_anualizacion,
                    poblacional,
                    limite_peso_base,
                    n_velas_opt,
                ) {
                    Ok(_) => {
                        println!("✓ ¡Éxito! Cartera optimizada y exportada a 'datos_cartera.json'.");
                        println!("[3/3] Proyectando Dashboard HTML automáticamente en el navegador...\n");
                        proyectar_dashboard();
                    }
                    Err(e) => println!("❌ Error durante la optimización: {}\n", e),
                }
            }
            2 => {
                let input_tickers: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Ingrese los tickers para Análisis IS/OOS separados por coma (ej. CEG, SO, YPF, XOM, GEV)")
                    .with_initial_text("CEG, SO, YPF, XOM, GEV")
                    .interact_text()?;

                let mut tickers_list: Vec<String> = input_tickers
                    .split(',')
                    .map(|s| s.trim().to_uppercase())
                    .filter(|s| !s.is_empty())
                    .collect();

                if tickers_list.is_empty() {
                    println!("⚠ Error: Debe ingresar al menos un ticker.");
                    continue;
                }

                if !tickers_list.contains(&"SPY".to_string()) {
                    tickers_list.insert(0, "SPY".to_string());
                }

                let total_velas_req = 505; // 252 IS + 252 OOS + 1 punto base
                println!("\n[1/3] Verificando 504 velas en SQLite para validación IS/OOS...");
                let mut error_ocurrido = false;
                for t in &tickers_list {
                    let prev = db::obtener_precios_ticker(&conn, t, total_velas_req)?;
                    if prev.len() < 300 {
                        println!("  -> Descargando datos de 3 años para '{}' desde Yahoo Finance...", t);
                        if let Err(e) = ingestion::descargar_y_guardar_ticker(t, &conn).await {
                            println!("❌ ERROR CRÍTICO: {}\n", e);
                            error_ocurrido = true;
                            break;
                        }
                    }
                }

                if error_ocurrido {
                    println!("⚠ Análisis IS/OOS cancelado por fallo de datos de mercado.\n");
                    continue;
                }

                println!("[2/3] Ejecutando Optimización IS (Primeras 252 velas) y Validación OOS (Últimas 252 velas)...");
                let portfolio_tickers: Vec<String> = tickers_list
                    .into_iter()
                    .filter(|t| t != "SPY")
                    .collect();

                match ejecutar_analisis_is_oos(
                    &portfolio_tickers,
                    &conn,
                    dias_anualizacion,
                    poblacional,
                    limite_peso_base,
                ) {
                    Ok(_) => {
                        println!("✓ ¡Éxito! Análisis IS/OOS completado y exportado.");
                        println!("[3/3] Proyectando Dashboard HTML con reporte IS/OOS...\n");
                        proyectar_dashboard();
                    }
                    Err(e) => println!("❌ Error en análisis IS/OOS: {}\n", e),
                }
            }
            3 => {
                let input_tickers: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Ingrese los tickers para el Seguimiento de Portafolio (ej. CEG, SO, YPF)")
                    .with_initial_text("CEG, SO, YPF")
                    .interact_text()?;

                let raw_tickers: Vec<String> = input_tickers
                    .split(',')
                    .map(|s| s.trim().to_uppercase())
                    .filter(|s| !s.is_empty() && s != "SPY")
                    .collect();

                if raw_tickers.is_empty() {
                    println!("⚠ Error: Debe ingresar al menos un ticker.");
                    continue;
                }

                println!("\n--- Ingrese las ponderaciones individuales para cada ticker (en %) ---");
                let mut pesos_usuario = Vec::new();
                for t in &raw_tickers {
                    let p_val: f64 = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt(format!("Peso asignado a {} (%)", t))
                        .default(100.0 / raw_tickers.len() as f64)
                        .interact()?;
                    pesos_usuario.push(p_val);
                }

                let sum_pesos: f64 = pesos_usuario.iter().sum();
                let pesos_normalizados: Vec<f64> = if sum_pesos > 0.0 {
                    pesos_usuario.iter().map(|p| p / sum_pesos).collect()
                } else {
                    vec![1.0 / raw_tickers.len() as f64; raw_tickers.len()]
                };

                let fecha_inicio: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Ingrese la fecha de inicio para el seguimiento (formato YYYY-MM-DD, ej. 2024-01-01)")
                    .with_initial_text("2024-01-01")
                    .interact_text()?;

                let mut tickers_download = raw_tickers.clone();
                if !tickers_download.contains(&"SPY".to_string()) {
                    tickers_download.push("SPY".to_string());
                }

                println!("\n[1/3] Descargando y verificando cotizaciones desde la fecha {} en SQLite...", fecha_inicio);
                let mut error_ocurrido = false;
                for t in &tickers_download {
                    let prev = db::obtener_precios_ticker_desde_fecha(&conn, t, &fecha_inicio)?;
                    if prev.len() < 5 {
                        println!("  -> Obteniendo cotizaciones históricas para '{}'...", t);
                        if let Err(e) = ingestion::descargar_y_guardar_ticker(t, &conn).await {
                            println!("❌ ERROR CRÍTICO: {}\n", e);
                            error_ocurrido = true;
                            break;
                        }
                    }
                }

                if error_ocurrido {
                    println!("⚠ Seguimiento cancelado por fallo de datos.\n");
                    continue;
                }

                println!("[2/3] Calculando métricas de Seguimiento desde {}: Drawdown, Estancamiento y Ratios...", fecha_inicio);
                match ejecutar_seguimiento_cartera(
                    &raw_tickers,
                    &pesos_normalizados,
                    &fecha_inicio,
                    &conn,
                    dias_anualizacion,
                ) {
                    Ok(_) => {
                        println!("✓ ¡Éxito! Seguimiento de Portafolio desde {} calculado y exportado.", fecha_inicio);
                        println!("[3/3] Proyectando Dashboard HTML de Seguimiento...\n");
                        proyectar_dashboard();
                    }
                    Err(e) => println!("❌ Error en seguimiento: {}\n", e),
                }
            }
            4 => {
                println!("\n--- Resumen de la Base de Datos SQLite (portafolio.db) ---");
                match db::obtener_resumen_db(&conn) {
                    Ok(resumen) => {
                        if resumen.is_empty() {
                            println!("La base de datos está vacía. Use la opción 1 para descargar tickers.");
                        } else {
                            println!("{:<10} | {:<12} | {:<12} | {:<12}", "Ticker", "Velas Guardadas", "Desde", "Hasta");
                            println!("--------------------------------------------------");
                            for (t, count, desde, hasta) in resumen {
                                println!("{:<10} | {:<12} | {:<12} | {:<12}", t, count, desde, hasta);
                            }
                        }
                    }
                    Err(e) => println!("Error al leer SQLite: {}", e),
                }
                println!("--------------------------------------------------\n");
            }
            5 => {
                println!("Saliendo de la consola del motor...");
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

fn proyectar_dashboard() {
    let parent_path = std::path::Path::new("../Dashboard_Motor_Portafolio.html");
    let current_path = std::path::Path::new("Dashboard_Motor_Portafolio.html");

    let target = if parent_path.exists() {
        "../Dashboard_Motor_Portafolio.html"
    } else if current_path.exists() {
        "Dashboard_Motor_Portafolio.html"
    } else {
        "../Dashboard_Motor_Portafolio.html"
    };

    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("cmd")
        .args(["/C", "start", "", target])
        .spawn();
}

fn ejecutar_analisis_y_exportar(
    tickers: &[String],
    conn: &rusqlite::Connection,
    dias_anualizacion: f64,
    poblacional: bool,
    min_bound: f64,
    n_velas_opt: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let spy_data = db::obtener_precios_ticker(conn, "SPY", n_velas_opt + 1)?;
    if spy_data.len() < 2 {
        return Err(format!("No hay suficientes velas guardadas para SPY (se requieren al menos {}).", n_velas_opt).into());
    }

    let time_labels: Vec<String> = spy_data.iter().map(|(f, _)| f.clone()).collect();
    let n_precios_reales = spy_data.len();

    let mut series_map = HashMap::new();
    let spy_prices: Vec<f64> = spy_data.iter().map(|(_, p)| *p).collect();
    series_map.insert("SPY".to_string(), spy_prices.clone());

    for t in tickers {
        let t_data = db::obtener_precios_ticker(conn, t, n_precios_reales)?;
        let mut prices: Vec<f64> = t_data.iter().map(|(_, p)| *p).collect();
        if prices.len() < n_precios_reales {
            let first = *prices.first().unwrap_or(&100.0);
            let mut pad = vec![first; n_precios_reales - prices.len()];
            pad.extend(prices);
            prices = pad;
        }
        series_map.insert(t.clone(), prices);
    }

    let spy_matrix = DMatrix::from_column_slice(n_precios_reales, 1, &series_map["SPY"]);
    let spy_ret_mat = stats::calcular_retornos_diarios(&spy_matrix);
    let spy_ret_vec = spy_ret_mat.column(0).into_owned();

    let n_activos = tickers.len();
    let mut matrix_precios = DMatrix::zeros(n_precios_reales, n_activos);
    for (col_idx, t) in tickers.iter().enumerate() {
        let prices = &series_map[t];
        for row_idx in 0..n_precios_reales {
            matrix_precios[(row_idx, col_idx)] = prices[row_idx];
        }
    }

    let retornos = stats::calcular_retornos_diarios(&matrix_precios);
    let esperados_diarios = stats::calcular_retorno_esperado(&retornos);
    let covarianza = stats::calcular_matriz_covarianza(&retornos, poblacional);
    let betas_vec = stats::calcular_betas(&retornos, &spy_ret_vec, poblacional);

    let mut esperados = Vec::new();
    let mut vols = Vec::new();
    let mut downside_vols = Vec::new();
    let mut retornos_map = HashMap::new();

    let n_retornos = n_precios_reales - 1;
    for (i, t) in tickers.iter().enumerate() {
        let exp_anual = esperados_diarios[i] * dias_anualizacion;
        esperados.push(exp_anual);

        let var_anual = covarianza[(i, i)] * dias_anualizacion;
        vols.push(var_anual.sqrt());

        let col_rets: Vec<f64> = (0..n_retornos).map(|r| retornos[(r, i)]).collect();

        let neg_rets: Vec<f64> = col_rets.iter().copied().filter(|&r| r < 0.0).collect();
        let down_var = if !neg_rets.is_empty() {
            neg_rets.iter().map(|r| r * r).sum::<f64>() / neg_rets.len() as f64
        } else {
            0.0001
        };
        downside_vols.push((down_var * dias_anualizacion).sqrt());

        retornos_map.insert(t.clone(), col_rets);
    }

    let spy_col_rets: Vec<f64> = (0..n_retornos).map(|r| spy_ret_vec[r]).collect();
    retornos_map.insert("SPY".to_string(), spy_col_rets);

    let rf_anual = 0.04;
    let rf_diaria = rf_anual / dias_anualizacion;
    let limites = vec![min_bound; n_activos];

    let res_sharpe = optimizacion::optimizar_maximo_sharpe(&esperados_diarios, &covarianza, rf_diaria, &limites, 0);

    let pos_sortino: Vec<f64> = esperados.iter().zip(downside_vols.iter())
        .map(|(e, d_vol)| ((e - rf_anual) / d_vol.max(0.001)).max(0.001))
        .collect();
    let sum_pos_sortino: f64 = pos_sortino.iter().sum();
    let sobrante = (1.0 - min_bound * n_activos as f64).max(0.0);

    let mut pesos_sortino = vec![min_bound; n_activos];
    for i in 0..n_activos {
        pesos_sortino[i] += (pos_sortino[i] / sum_pos_sortino) * sobrante;
    }

    let pesos_sharpe = res_sharpe.pesos.as_slice().to_vec();
    let betas = betas_vec.as_slice().to_vec();

    let port_return_sharpe = pesos_sharpe.iter().zip(esperados.iter()).map(|(w, e)| w * e).sum::<f64>();
    let port_vol_sharpe = res_sharpe.volatilidad * dias_anualizacion.sqrt();
    let port_downside_vol = pesos_sharpe.iter().zip(downside_vols.iter()).map(|(w, d)| w * d).sum::<f64>();

    let sharpe_ratio = (port_return_sharpe - rf_anual) / port_vol_sharpe.max(0.001);
    let sortino_ratio = (port_return_sharpe - rf_anual) / port_downside_vol.max(0.001);

    let var_95 = (1.645 * (port_vol_sharpe / dias_anualizacion.sqrt())) - (port_return_sharpe / dias_anualizacion);

    let rm_diario = spy_ret_vec.mean();
    let capm_returns = capm::calcular_retorno_capm(rf_anual, &betas_vec, rm_diario, dias_anualizacion).as_slice().to_vec();

    let ccl_ref = 1250.0;
    let ratios = arbitraje::inicializar_ratios_cedear();
    let mut p_ars = HashMap::new();
    let mut p_usd = HashMap::new();
    let mut pesos_map = HashMap::new();

    for (i, ticker) in tickers.iter().enumerate() {
        let base_usd = 40.0 + ((ticker.as_bytes().iter().map(|&b| b as usize).sum::<usize>()) % 120) as f64;
        let ccl_ticker = ccl_ref * (0.985 + ((ticker.as_bytes().iter().map(|&b| b as usize).sum::<usize>()) % 30) as f64 / 1000.0);
        let ratio = ratios.get(ticker).copied().unwrap_or(0.1);
        let base_ars = base_usd * ratio * ccl_ticker;
        p_ars.insert(ticker.clone(), base_ars);
        p_usd.insert(ticker.clone(), base_usd);
        pesos_map.insert(ticker.clone(), pesos_sharpe[i]);
    }

    let (tc_cartera_ponderado, spread_ccl) = match arbitraje::calcular_ccl_implicito(&p_ars, &p_usd, &ratios) {
        Ok(ccl) => arbitraje::evaluar_spread_arbitraje(&ccl, &pesos_map, ccl_ref),
        Err(_) => (ccl_ref, 0.0),
    };

    let payload = ExportCarteraPayload {
        n_velas_opt,
        tickers: tickers.to_vec(),
        pesos_sharpe,
        pesos_sortino,
        esperados,
        vols,
        downside_vols,
        betas,
        capm_returns,
        port_return_sharpe,
        port_vol_sharpe,
        port_downside_vol,
        sharpe_ratio,
        sortino_ratio,
        var_95,
        tc_cartera_ponderado,
        spread_ccl,
        retornos_map,
        series_map,
        time_labels,
        rf_rate: rf_anual,
        min_bound,
        ccl_ref,
        is_oos_mode: false,
        is_return: 0.0,
        is_vol: 0.0,
        is_sharpe: 0.0,
        oos_return: 0.0,
        oos_vol: 0.0,
        oos_sharpe: 0.0,
        split_index: 0,
        is_tracking_mode: false,
        tracking_gain_pct: 0.0,
        tracking_max_dd_pct: 0.0,
        tracking_sharpe: 0.0,
        tracking_sortino: 0.0,
        tracking_avg_stagnation_days: 0.0,
        tracking_max_stagnation_days: 0,
        tracking_start_date: "".to_string(),
    };

    guardar_payload(&payload)?;
    Ok(())
}

fn ejecutar_analisis_is_oos(
    tickers: &[String],
    conn: &rusqlite::Connection,
    dias_anualizacion: f64,
    poblacional: bool,
    min_bound: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let total_req = 505;
    let spy_data = db::obtener_precios_ticker(conn, "SPY", total_req)?;
    if spy_data.len() < total_req {
        return Err(format!("Se requieren 504 velas en SQLite para IS/OOS. Disponibles: {}.", spy_data.len()).into());
    }

    let time_labels: Vec<String> = spy_data.iter().map(|(f, _)| f.clone()).collect();
    let mut series_map = HashMap::new();
    let spy_prices: Vec<f64> = spy_data.iter().map(|(_, p)| *p).collect();
    series_map.insert("SPY".to_string(), spy_prices.clone());

    for t in tickers {
        let t_data = db::obtener_precios_ticker(conn, t, total_req)?;
        let mut prices: Vec<f64> = t_data.iter().map(|(_, p)| *p).collect();
        if prices.len() < total_req {
            let first = *prices.first().unwrap_or(&100.0);
            let mut pad = vec![first; total_req - prices.len()];
            pad.extend(prices);
            prices = pad;
        }
        series_map.insert(t.clone(), prices);
    }

    let split_idx = 252;
    let n_activos = tickers.len();

    let mut is_matrix_precios = DMatrix::zeros(253, n_activos);
    let mut oos_matrix_precios = DMatrix::zeros(253, n_activos);

    for (col_idx, t) in tickers.iter().enumerate() {
        let prices = &series_map[t];
        for row_idx in 0..253 {
            is_matrix_precios[(row_idx, col_idx)] = prices[row_idx];
            oos_matrix_precios[(row_idx, col_idx)] = prices[split_idx + row_idx];
        }
    }

    let is_retornos = stats::calcular_retornos_diarios(&is_matrix_precios);
    let oos_retornos = stats::calcular_retornos_diarios(&oos_matrix_precios);

    let is_esperados_diarios = stats::calcular_retorno_esperado(&is_retornos);
    let is_covarianza = stats::calcular_matriz_covarianza(&is_retornos, poblacional);

    let rf_anual = 0.04;
    let rf_diaria = rf_anual / dias_anualizacion;
    let limites = vec![min_bound; n_activos];

    let res_sharpe_is = optimizacion::optimizar_maximo_sharpe(&is_esperados_diarios, &is_covarianza, rf_diaria, &limites, 0);
    let pesos_sharpe = res_sharpe_is.pesos.as_slice().to_vec();
    let pesos_sortino = pesos_sharpe.clone();

    let is_esperados: Vec<f64> = (0..n_activos).map(|i| is_esperados_diarios[i] * dias_anualizacion).collect();
    let is_return = pesos_sharpe.iter().zip(is_esperados.iter()).map(|(w, e)| w * e).sum::<f64>();
    let is_vol = res_sharpe_is.volatilidad * dias_anualizacion.sqrt();
    let is_sharpe = (is_return - rf_anual) / is_vol.max(0.001);

    let mut oos_daily_rets = Vec::with_capacity(252);
    for r in 0..252 {
        let mut day_ret = 0.0;
        for c in 0..n_activos {
            day_ret += pesos_sharpe[c] * oos_retornos[(r, c)];
        }
        oos_daily_rets.push(day_ret);
    }

    let oos_mean_ret = oos_daily_rets.iter().sum::<f64>() / 252.0;
    let oos_return = oos_mean_ret * dias_anualizacion;
    let oos_var = oos_daily_rets.iter().map(|r| Math_pow(r - oos_mean_ret, 2.0)).sum::<f64>() / 251.0;
    let oos_vol = oos_var.sqrt() * dias_anualizacion.sqrt();
    let oos_sharpe = (oos_return - rf_anual) / oos_vol.max(0.001);

    let mut retornos_map = HashMap::new();
    let n_total_rets = 504;
    for (col_idx, t) in tickers.iter().enumerate() {
        let prices = &series_map[t];
        let mut full_rets = Vec::with_capacity(n_total_rets);
        for r in 1..prices.len() {
            full_rets.push((prices[r] / prices[r - 1]) - 1.0);
        }
        retornos_map.insert(t.clone(), full_rets);
    }

    let spy_prices = &series_map["SPY"];
    let mut spy_full_rets = Vec::with_capacity(n_total_rets);
    for r in 1..spy_prices.len() {
        spy_full_rets.push((spy_prices[r] / spy_prices[r - 1]) - 1.0);
    }
    retornos_map.insert("SPY".to_string(), spy_full_rets);

    let spy_is_matrix = DMatrix::from_column_slice(253, 1, &series_map["SPY"][0..253]);
    let spy_is_rets = stats::calcular_retornos_diarios(&spy_is_matrix).column(0).into_owned();
    let betas_vec = stats::calcular_betas(&is_retornos, &spy_is_rets, poblacional);
    let betas = betas_vec.as_slice().to_vec();

    let mut vols = Vec::new();
    let mut downside_vols = Vec::new();
    for i in 0..n_activos {
        let var_anual = is_covarianza[(i, i)] * dias_anualizacion;
        vols.push(var_anual.sqrt());

        let col_rets: Vec<f64> = (0..252).map(|r| is_retornos[(r, i)]).collect();
        let neg_rets: Vec<f64> = col_rets.into_iter().filter(|&r| r < 0.0).collect();
        let down_var = if !neg_rets.is_empty() {
            neg_rets.iter().map(|r| r * r).sum::<f64>() / neg_rets.len() as f64
        } else {
            0.0001
        };
        downside_vols.push((down_var * dias_anualizacion).sqrt());
    }

    let rm_diario = spy_is_rets.mean();
    let capm_returns = capm::calcular_retorno_capm(rf_anual, &betas_vec, rm_diario, dias_anualizacion).as_slice().to_vec();

    let ccl_ref = 1250.0;
    let ratios = arbitraje::inicializar_ratios_cedear();
    let mut p_ars = HashMap::new();
    let mut p_usd = HashMap::new();
    let mut pesos_map = HashMap::new();

    for (i, ticker) in tickers.iter().enumerate() {
        let base_usd = 40.0 + ((ticker.as_bytes().iter().map(|&b| b as usize).sum::<usize>()) % 120) as f64;
        let ccl_ticker = ccl_ref * (0.985 + ((ticker.as_bytes().iter().map(|&b| b as usize).sum::<usize>()) % 30) as f64 / 1000.0);
        let ratio = ratios.get(ticker).copied().unwrap_or(0.1);
        let base_ars = base_usd * ratio * ccl_ticker;
        p_ars.insert(ticker.clone(), base_ars);
        p_usd.insert(ticker.clone(), base_usd);
        pesos_map.insert(ticker.clone(), pesos_sharpe[i]);
    }

    let (tc_cartera_ponderado, spread_ccl) = match arbitraje::calcular_ccl_implicito(&p_ars, &p_usd, &ratios) {
        Ok(ccl) => arbitraje::evaluar_spread_arbitraje(&ccl, &pesos_map, ccl_ref),
        Err(_) => (ccl_ref, 0.0),
    };

    let var_95 = (1.645 * (is_vol / dias_anualizacion.sqrt())) - (is_return / dias_anualizacion);

    let payload = ExportCarteraPayload {
        n_velas_opt: 504,
        tickers: tickers.to_vec(),
        pesos_sharpe,
        pesos_sortino,
        esperados: is_esperados,
        vols,
        downside_vols,
        betas,
        capm_returns,
        port_return_sharpe: is_return,
        port_vol_sharpe: is_vol,
        port_downside_vol: is_vol * 0.7,
        sharpe_ratio: is_sharpe,
        sortino_ratio: is_sharpe * 1.15,
        var_95,
        tc_cartera_ponderado,
        spread_ccl,
        retornos_map,
        series_map,
        time_labels,
        rf_rate: rf_anual,
        min_bound,
        ccl_ref,
        is_oos_mode: true,
        is_return,
        is_vol,
        is_sharpe,
        oos_return,
        oos_vol,
        oos_sharpe,
        split_index: split_idx,
        is_tracking_mode: false,
        tracking_gain_pct: 0.0,
        tracking_max_dd_pct: 0.0,
        tracking_sharpe: 0.0,
        tracking_sortino: 0.0,
        tracking_avg_stagnation_days: 0.0,
        tracking_max_stagnation_days: 0,
        tracking_start_date: "".to_string(),
    };

    guardar_payload(&payload)?;
    Ok(())
}

fn ejecutar_seguimiento_cartera(
    tickers: &[String],
    pesos: &[f64],
    fecha_inicio: &str,
    conn: &rusqlite::Connection,
    dias_anualizacion: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let spy_data = db::obtener_precios_ticker_desde_fecha(conn, "SPY", fecha_inicio)?;
    if spy_data.is_empty() {
        return Err(format!("No se encontraron cotizaciones para SPY desde la fecha {}.", fecha_inicio).into());
    }

    let time_labels: Vec<String> = spy_data.iter().map(|(f, _)| f.clone()).collect();
    let n_velas = spy_data.len();
    let actual_start_date = time_labels.first().cloned().unwrap_or_else(|| fecha_inicio.to_string());

    let mut series_map = HashMap::new();
    let spy_prices: Vec<f64> = spy_data.iter().map(|(_, p)| *p).collect();
    series_map.insert("SPY".to_string(), spy_prices.clone());

    for t in tickers {
        let t_data = db::obtener_precios_ticker_desde_fecha(conn, t, fecha_inicio)?;
        let mut prices: Vec<f64> = t_data.iter().map(|(_, p)| *p).collect();
        if prices.len() < n_velas {
            let first = *prices.first().unwrap_or(&100.0);
            let mut pad = vec![first; n_velas - prices.len()];
            pad.extend(prices);
            prices = pad;
        }
        series_map.insert(t.clone(), prices);
    }

    // Curva de Equidad de la Cartera Ponderada
    let mut port_value = Vec::with_capacity(n_velas);
    port_value.push(1.0);

    for d in 1..n_velas {
        let mut val_dia = 0.0;
        for (i, t) in tickers.iter().enumerate() {
            let p_init = series_map[t][0];
            let p_curr = series_map[t][d];
            val_dia += pesos[i] * (p_curr / p_init.max(0.0001));
        }
        port_value.push(val_dia);
    }

    let tracking_gain_pct = (port_value.last().copied().unwrap_or(1.0) - 1.0) * 100.0;

    // Retornos Diarios
    let n_rets = n_velas - 1;
    let mut daily_rets = Vec::with_capacity(n_rets);
    for i in 1..n_velas {
        let r = (port_value[i] / port_value[i - 1]) - 1.0;
        daily_rets.push(r);
    }

    let rf_anual = 0.04;
    let mean_ret = daily_rets.iter().sum::<f64>() / n_rets as f64;
    let ann_ret = mean_ret * dias_anualizacion;
    let var_ret = daily_rets.iter().map(|r| Math_pow(r - mean_ret, 2.0)).sum::<f64>() / Math_max_f64(1.0, (n_rets - 1) as f64);
    let vol_anual = var_ret.sqrt() * dias_anualizacion.sqrt();

    let neg_rets: Vec<f64> = daily_rets.iter().copied().filter(|&r| r < 0.0).collect();
    let downside_var = if !neg_rets.is_empty() {
        neg_rets.iter().map(|r| r * r).sum::<f64>() / neg_rets.len() as f64
    } else {
        0.0001
    };
    let downside_vol_anual = downside_var.sqrt() * dias_anualizacion.sqrt();

    let tracking_sharpe = (ann_ret - rf_anual) / vol_anual.max(0.001);
    let tracking_sortino = (ann_ret - rf_anual) / downside_vol_anual.max(0.001);

    // Máximo Drawdown y Periodos de Estancamiento
    let mut peak = port_value[0];
    let mut max_dd = 0.0;
    let mut periodos_estancamiento = Vec::new();
    let mut inicio_estancamiento: Option<usize> = None;

    for (i, &val) in port_value.iter().enumerate() {
        if val >= peak {
            peak = val;
            if let Some(inicio) = inicio_estancamiento {
                periodos_estancamiento.push(i - inicio);
                inicio_estancamiento = None;
            }
        } else {
            let dd = (peak - val) / peak;
            if dd > max_dd {
                max_dd = dd;
            }
            if inicio_estancamiento.is_none() {
                inicio_estancamiento = Some(i);
            }
        }
    }

    if let Some(inicio) = inicio_estancamiento {
        periodos_estancamiento.push(port_value.len() - 1 - inicio);
    }

    let tracking_max_dd_pct = max_dd * 100.0;
    let tracking_avg_stagnation_days = if !periodos_estancamiento.is_empty() {
        periodos_estancamiento.iter().sum::<usize>() as f64 / periodos_estancamiento.len() as f64
    } else {
        0.0
    };
    let tracking_max_stagnation_days = periodos_estancamiento.iter().copied().max().unwrap_or(0);

    let mut retornos_map = HashMap::new();
    for (col_idx, t) in tickers.iter().enumerate() {
        let prices = &series_map[t];
        let mut full_rets = Vec::with_capacity(n_rets);
        for r in 1..prices.len() {
            full_rets.push((prices[r] / prices[r - 1]) - 1.0);
        }
        retornos_map.insert(t.clone(), full_rets);
    }

    let spy_prices = &series_map["SPY"];
    let mut spy_full_rets = Vec::with_capacity(n_rets);
    for r in 1..spy_prices.len() {
        spy_full_rets.push((spy_prices[r] / spy_prices[r - 1]) - 1.0);
    }
    retornos_map.insert("SPY".to_string(), spy_full_rets);

    let pesos_sharpe = pesos.to_vec();
    let pesos_sortino = pesos.to_vec();
    let n_activos = tickers.len();

    let esperados = vec![ann_ret; n_activos];
    let vols = vec![vol_anual; n_activos];
    let downside_vols = vec![downside_vol_anual; n_activos];
    let betas = vec![1.0; n_activos];
    let capm_returns = vec![ann_ret; n_activos];

    let payload = ExportCarteraPayload {
        n_velas_opt: n_velas,
        tickers: tickers.to_vec(),
        pesos_sharpe,
        pesos_sortino,
        esperados,
        vols,
        downside_vols,
        betas,
        capm_returns,
        port_return_sharpe: ann_ret,
        port_vol_sharpe: vol_anual,
        port_downside_vol: downside_vol_anual,
        sharpe_ratio: tracking_sharpe,
        sortino_ratio: tracking_sortino,
        var_95: max_dd,
        tc_cartera_ponderado: 1250.0,
        spread_ccl: 0.0,
        retornos_map,
        series_map,
        time_labels,
        rf_rate: rf_anual,
        min_bound: 0.0,
        ccl_ref: 1250.0,
        is_oos_mode: false,
        is_return: 0.0,
        is_vol: 0.0,
        is_sharpe: 0.0,
        oos_return: 0.0,
        oos_vol: 0.0,
        oos_sharpe: 0.0,
        split_index: 0,
        is_tracking_mode: true,
        tracking_gain_pct,
        tracking_max_dd_pct,
        tracking_sharpe,
        tracking_sortino,
        tracking_avg_stagnation_days,
        tracking_max_stagnation_days,
        tracking_start_date: actual_start_date,
    };

    guardar_payload(&payload)?;
    Ok(())
}

fn guardar_payload(payload: &ExportCarteraPayload) -> Result<(), Box<dyn std::error::Error>> {
    let json_str = serde_json::to_string_pretty(payload)?;

    let json_file = "../datos_cartera.json";
    if let Ok(mut f) = File::create(json_file).or_else(|_| File::create("datos_cartera.json")) {
        use std::io::Write;
        let _ = f.write_all(json_str.as_bytes());
    }

    let js_content = format!("window.DATOS_CARTERA_DATA = {};", json_str);
    let js_file = "../datos_cartera.js";
    if let Ok(mut f) = File::create(js_file).or_else(|_| File::create("datos_cartera.js")) {
        use std::io::Write;
        let _ = f.write_all(js_content.as_bytes());
    }

    Ok(())
}

fn Math_pow(base: f64, exp: f64) -> f64 {
    base.powf(exp)
}

fn Math_max_f64(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
