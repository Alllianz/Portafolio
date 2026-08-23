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
    Ok(conn)
}
