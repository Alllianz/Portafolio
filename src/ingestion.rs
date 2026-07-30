use rusqlite::Connection;
use serde_json::Value;
use std::error::Error;

/// Descarga las cotizaciones reales de mercado desde la API de Yahoo Finance en Rust
/// y las persiste directamente en la base de datos SQLite `portafolio.db`.
pub async fn descargar_y_guardar_ticker(
    symbol: &str,
    conn: &Connection,
) -> Result<usize, Box<dyn Error>> {
    let clean_symbol = symbol.trim().to_uppercase();
    println!("Descargando cotizaciones reales para '{}' desde Yahoo Finance...", clean_symbol);

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build()?;

    let candidates = if clean_symbol == "YPF" {
        vec![clean_symbol.clone(), "YPFD.BA".to_string()]
    } else {
        vec![clean_symbol.clone()]
    };

    let mut fetched_data: Vec<(String, f64)> = Vec::new();

    for sym in candidates {
        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}?range=3y&interval=1d",
            sym
        );

        let resp_res = client.get(&url).send().await;
        let resp = match resp_res {
            Ok(r) => r,
            Err(_) => {
                let url2 = format!(
                    "https://query2.finance.yahoo.com/v8/finance/chart/{}?range=3y&interval=1d",
                    sym
                );
                match client.get(&url2).send().await {
                    Ok(r2) => r2,
                    Err(e) => return Err(Box::new(e)),
                }
            }
        };

        if !resp.status().is_success() {
            continue;
        }

        let body: Value = resp.json().await?;
        let result = &body["chart"]["result"][0];
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
            "Error de descarga real: No se pudieron obtener cotizaciones de mercado para el ticker '{}'.",
            clean_symbol
        )
        .into());
    }

    let count = crate::db::guardar_precios(conn, &clean_symbol, &fetched_data)?;
    println!(
        "✓ ¡Éxito! Se guardaron {} registros de precios reales para '{}' en la base de datos.",
        count, clean_symbol
    );
    Ok(count)
}
