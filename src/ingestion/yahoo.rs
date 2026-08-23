use rusqlite::Connection;
use serde_json::Value;
use std::error::Error;
use std::time::Duration;

/// Descarga las cotizaciones de mercado desde Yahoo Finance vía HTTP asíncrono con headers y timeout optimizado.
pub async fn descargar_cotizaciones_yahoo(
    symbol: &str,
) -> Result<Vec<(String, f64)>, Box<dyn Error + Send + Sync>> {
    let clean_symbol = symbol.trim().to_uppercase();
    if clean_symbol.is_empty() {
        return Err("El símbolo de ticker no puede estar vacío.".into());
    }

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
        .timeout(Duration::from_secs(12))
        .build()?;

    let candidates = if clean_symbol == "YPF" {
        vec![clean_symbol.clone(), "YPFD.BA".to_string()]
    } else {
        vec![clean_symbol.clone()]
    };

    let mut fetched_data: Vec<(String, f64)> = Vec::new();

    for sym in candidates {
        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}?range=10y&interval=1d",
            sym
        );

        let resp_res = client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await;

        let resp = match resp_res {
            Ok(r) if r.status().is_success() => r,
            _ => {
                let url2 = format!(
                    "https://query2.finance.yahoo.com/v8/finance/chart/{}?range=10y&interval=1d",
                    sym
                );
                match client.get(&url2).header("Accept", "application/json").send().await {
                    Ok(r2) if r2.status().is_success() => r2,
                    _ => continue,
                }
            }
        };

        let body: Value = match resp.json().await {
            Ok(b) => b,
            Err(_) => continue,
        };

        let result = match body["chart"]["result"].get(0) {
            Some(r) => r,
            None => continue,
        };

        let timestamps = match result["timestamp"].as_array() {
            Some(ts) => ts,
            None => continue,
        };

        let quote_close = &result["indicators"]["quote"][0]["close"];
        let adj_close = &result["indicators"]["adjclose"][0]["adjclose"];

        for i in 0..timestamps.len() {
            let ts = timestamps[i].as_i64().unwrap_or(0);
            let price_opt = adj_close[i]
                .as_f64()
                .or_else(|| quote_close[i].as_f64());

            if let Some(price) = price_opt {
                if !price.is_nan() && price > 0.0 {
                    let dt = chrono::DateTime::from_timestamp(ts, 0)
                        .map(|d| d.format("%Y-%m-%d").to_string())
                        .unwrap_or_else(|| format!("{}", ts));
                    fetched_data.push((dt, price));
                }
            }
        }

        if fetched_data.len() > 5 {
            break;
        }
    }

    if fetched_data.is_empty() {
        return Err(format!(
            "No se pudieron descargar cotizaciones para '{}' desde Yahoo Finance.",
            clean_symbol
        )
        .into());
    }

    Ok(fetched_data)
}

/// Guarda las cotizaciones descargadas en la base de datos SQLite filtrando duplicados existentes.
pub fn persistir_cotizaciones(
    conn: &Connection,
    symbol: &str,
    mut fetched_data: Vec<(String, f64)>,
) -> Result<usize, Box<dyn Error + Send + Sync>> {
    let clean_symbol = symbol.trim().to_uppercase();
    let ultima_fecha = crate::db::obtener_ultima_fecha_ticker(conn, &clean_symbol)
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

    if let Some(ref max_dt) = ultima_fecha {
        fetched_data.retain(|(dt, _)| dt > max_dt);
    } else if fetched_data.len() > 2520 {
        let start = fetched_data.len() - 2520;
        fetched_data = fetched_data[start..].to_vec();
    }

    if fetched_data.is_empty() {
        return Ok(0);
    }

    let count = crate::db::guardar_precios(conn, &clean_symbol, &fetched_data)
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
    Ok(count)
}

/// Descarga de Yahoo Finance y persiste en SQLite en un solo flujo.
#[allow(dead_code)]
pub async fn descargar_y_guardar_ticker(
    symbol: &str,
    conn: &Connection,
) -> Result<usize, Box<dyn Error + Send + Sync>> {
    let raw_data = descargar_cotizaciones_yahoo(symbol).await?;
    persistir_cotizaciones(conn, symbol, raw_data)
}
