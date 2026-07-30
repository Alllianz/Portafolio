use rusqlite::{params, Connection, Result};

/// Inicializa la base de datos SQLite creando la tabla de precios si no existe.
pub fn init_db(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS precios_historicos (
            ticker TEXT NOT NULL,
            fecha TEXT NOT NULL,
            precio REAL NOT NULL,
            PRIMARY KEY (ticker, fecha)
        )",
        [],
    )?;
    Ok(conn)
}

/// Guarda o reemplaza los registros de precios de un ticker en la base de datos.
pub fn guardar_precios(conn: &Connection, ticker: &str, datas: &[(String, f64)]) -> Result<usize> {
    let tx = conn.unchecked_transaction()?;
    let mut count = 0;
    {
        let mut stmt = tx.prepare(
            "INSERT OR REPLACE INTO precios_historicos (ticker, fecha, precio) VALUES (?, ?, ?)",
        )?;
        for (fecha, precio) in datas {
            stmt.execute(params![ticker, fecha, precio])?;
            count += 1;
        }
    }
    tx.commit()?;
    Ok(count)
}

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

/// Obtiene un resumen cuantitativo de los datos por ticker en la base de datos.
pub fn obtener_resumen_db(conn: &Connection) -> Result<Vec<(String, usize, String, String)>> {
    let mut stmt = conn.prepare(
        "SELECT ticker, COUNT(*), MIN(fecha), MAX(fecha) FROM precios_historicos GROUP BY ticker ORDER BY ticker",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
    })?;
    let mut res = Vec::new();
    for r in rows {
        res.push(r?);
    }
    Ok(res)
}
