use rusqlite::{params, Connection, Result};
use crate::models::{CarteraGuardadaItem, GuardarCarteraRequest, TickerResumenItem};
use chrono::Local;

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

/// Guarda una cartera de inversión completa en SQLite.
pub fn guardar_cartera(conn: &Connection, req: &GuardarCarteraRequest) -> Result<i64> {
    let fecha_creacion = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let tickers_json = serde_json::to_string(&req.tickers).unwrap_or_else(|_| "[]".to_string());
    let pesos_json = serde_json::to_string(&req.pesos).unwrap_or_else(|_| "[]".to_string());
    let descripcion = req.descripcion.clone().unwrap_or_default();

    conn.execute(
        "INSERT INTO carteras_guardadas (
            nombre, descripcion, fecha_creacion, tipo_ponderacion,
            tickers_json, pesos_json, retorno_esperado, volatilidad,
            sharpe_ratio, ccl_ref, rf_rate
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            req.nombre,
            descripcion,
            fecha_creacion,
            req.tipo_ponderacion,
            tickers_json,
            pesos_json,
            req.retorno_esperado,
            req.volatilidad,
            req.sharpe_ratio,
            req.ccl_ref,
            req.rf_rate
        ],
    )?;

    Ok(conn.last_insert_rowid())
}

/// Obtiene la lista de todas las carteras guardadas en la base de datos, ordenadas de más reciente a más antigua.
pub fn listar_carteras(conn: &Connection) -> Result<Vec<CarteraGuardadaItem>> {
    let mut stmt = conn.prepare(
        "SELECT id, nombre, descripcion, fecha_creacion, tipo_ponderacion,
                tickers_json, pesos_json, retorno_esperado, volatilidad,
                sharpe_ratio, ccl_ref, rf_rate
         FROM carteras_guardadas
         ORDER BY id DESC",
    )?;

    let rows = stmt.query_map([], |row| {
        let tickers_json: String = row.get(5)?;
        let pesos_json: String = row.get(6)?;

        let tickers: Vec<String> = serde_json::from_str(&tickers_json).unwrap_or_default();
        let pesos: Vec<f64> = serde_json::from_str(&pesos_json).unwrap_or_default();

        Ok(CarteraGuardadaItem {
            id: row.get(0)?,
            nombre: row.get(1)?,
            descripcion: row.get(2)?,
            fecha_creacion: row.get(3)?,
            tipo_ponderacion: row.get(4)?,
            tickers,
            pesos,
            retorno_esperado: row.get(7)?,
            volatilidad: row.get(8)?,
            sharpe_ratio: row.get(9)?,
            ccl_ref: row.get(10)?,
            rf_rate: row.get(11)?,
        })
    })?;

    let mut list = Vec::new();
    for r in rows {
        list.push(r?);
    }
    Ok(list)
}

/// Obtiene una cartera guardada específica por su ID.
pub fn obtener_cartera_por_id(conn: &Connection, id: i64) -> Result<Option<CarteraGuardadaItem>> {
    let mut stmt = conn.prepare(
        "SELECT id, nombre, descripcion, fecha_creacion, tipo_ponderacion,
                tickers_json, pesos_json, retorno_esperado, volatilidad,
                sharpe_ratio, ccl_ref, rf_rate
         FROM carteras_guardadas
         WHERE id = ?",
    )?;

    let mut rows = stmt.query(params![id])?;
    if let Some(row) = rows.next()? {
        let tickers_json: String = row.get(5)?;
        let pesos_json: String = row.get(6)?;

        let tickers: Vec<String> = serde_json::from_str(&tickers_json).unwrap_or_default();
        let pesos: Vec<f64> = serde_json::from_str(&pesos_json).unwrap_or_default();

        Ok(Some(CarteraGuardadaItem {
            id: row.get(0)?,
            nombre: row.get(1)?,
            descripcion: row.get(2)?,
            fecha_creacion: row.get(3)?,
            tipo_ponderacion: row.get(4)?,
            tickers,
            pesos,
            retorno_esperado: row.get(7)?,
            volatilidad: row.get(8)?,
            sharpe_ratio: row.get(9)?,
            ccl_ref: row.get(10)?,
            rf_rate: row.get(11)?,
        }))
    } else {
        Ok(None)
    }
}

/// Elimina una cartera guardada por su ID.
pub fn eliminar_cartera(conn: &Connection, id: i64) -> Result<bool> {
    let rows = conn.execute("DELETE FROM carteras_guardadas WHERE id = ?", params![id])?;
    Ok(rows > 0)
}
