use rusqlite::{params, Connection, Result};
use crate::models::TickerResumenItem;

/// Obtiene los últimos `limite_velas` registros de precios para un ticker específico, ordenados cronológicamente.
pub fn obtener_precios_ticker(conn: &Connection, ticker: &str, limite_velas: usize) -> Result<Vec<(String, f64)>> {
    let mut stmt = conn.prepare(
        "SELECT fecha, precio FROM precios_historicos WHERE ticker = ? ORDER BY fecha DESC LIMIT ?",
    )?;
    let rows = stmt.query_map(params![ticker, limite_velas as i64], |row| {
        Ok((row.get(0)?, row.get(1)?))
    })?;

    let mut res = Vec::new();
    for row in rows {
        res.push(row?);
    }
    res.reverse(); // Ordenar en sentido cronológico ascendente (del día más antiguo al más reciente)
    Ok(res)
}

/// Obtiene todos los registros de precios para un ticker desde una fecha de inicio (YYYY-MM-DD), ordenados cronológicamente.
pub fn obtener_precios_ticker_desde_fecha(conn: &Connection, ticker: &str, fecha_inicio: &str) -> Result<Vec<(String, f64)>> {
    let mut stmt = conn.prepare(
        "SELECT fecha, precio FROM precios_historicos WHERE ticker = ? AND fecha >= ? ORDER BY fecha ASC",
    )?;
    let rows = stmt.query_map(params![ticker, fecha_inicio], |row| {
        Ok((row.get(0)?, row.get(1)?))
    })?;

    let mut res = Vec::new();
    for row in rows {
        res.push(row?);
    }
    Ok(res)
}

/// Obtiene la lista de todos los tickers almacenados en la base de datos.
pub fn obtener_tickers_guardados(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT DISTINCT ticker FROM precios_historicos ORDER BY ticker")?;
    let rows = stmt.query_map([], |row| row.get(0))?;
    let mut tickers = Vec::new();
    for r in rows {
        tickers.push(r?);
    }
    Ok(tickers)
}

/// Obtiene un resumen estructurado por ticker en la base de datos.
pub fn obtener_resumen_db(conn: &Connection) -> Result<Vec<TickerResumenItem>> {
    let mut stmt = conn.prepare(
        "SELECT ticker, COUNT(*), MIN(fecha), MAX(fecha) FROM precios_historicos GROUP BY ticker ORDER BY ticker",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(TickerResumenItem {
            ticker: row.get(0)?,
            count: row.get(1)?,
            desde: row.get(2)?,
            hasta: row.get(3)?,
        })
    })?;
    let mut res = Vec::new();
    for r in rows {
        res.push(r?);
    }
    Ok(res)
}

/// Obtiene la última fecha (máxima) registrada en la base de datos para un ticker dado.
pub fn obtener_ultima_fecha_ticker(conn: &Connection, ticker: &str) -> Result<Option<String>> {
    let mut stmt = conn.prepare("SELECT MAX(fecha) FROM precios_historicos WHERE ticker = ?")?;
    let mut rows = stmt.query(params![ticker])?;
    if let Some(row) = rows.next()? {
        let max_fecha: Option<String> = row.get(0)?;
        Ok(max_fecha)
    } else {
        Ok(None)
    }
}
