use rusqlite::{params, Connection, Result};

/// Guarda o actualiza los registros de precios de un ticker en SQLite usando una transacción atómica.
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
