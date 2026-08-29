use rusqlite::{Connection, Result};

/// Inicializa el esquema SQLite de la base de datos de precios históricos.
pub fn inicializar_esquema(db_path: &str) -> Result<Connection> {
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
    conn.execute(
        "CREATE TABLE IF NOT EXISTS carteras_guardadas (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nombre TEXT NOT NULL,
            descripcion TEXT NOT NULL,
            fecha_creacion TEXT NOT NULL,
            tipo_ponderacion TEXT NOT NULL,
            tickers_json TEXT NOT NULL,
            pesos_json TEXT NOT NULL,
            retorno_esperado REAL,
            volatilidad REAL,
            sharpe_ratio REAL,
            ccl_ref REAL,
            rf_rate REAL
        )",
        [],
    )?;
    Ok(conn)
}
