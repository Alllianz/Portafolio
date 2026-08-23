use std::sync::{
    atomic::{AtomicI64, Ordering},
    Arc,
};
use tokio::sync::Mutex;
use rusqlite::Connection;

#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)]
    pub db_path: String,
    pub dias_anualizacion: f64,
    // Tokio Mutex para soporte seguro y concurrente en handlers async de Axum
    pub db_conn: Arc<Mutex<Connection>>,
    // Timestamp del último latido (heartbeat) recibido desde la ventana del navegador
    pub last_heartbeat: Arc<AtomicI64>,
}

impl AppState {
    pub fn new(db_path: &str, dias_anualizacion: f64) -> Result<Self, rusqlite::Error> {
        let conn = crate::db::inicializar_esquema(db_path)?;
        let now = chrono::Utc::now().timestamp();
        Ok(Self {
            db_path: db_path.to_string(),
            dias_anualizacion,
            db_conn: Arc::new(Mutex::new(conn)),
            last_heartbeat: Arc::new(AtomicI64::new(now)),
        })
    }

    pub fn registrar_heartbeat(&self) {
        let now = chrono::Utc::now().timestamp();
        self.last_heartbeat.store(now, Ordering::SeqCst);
    }
}
