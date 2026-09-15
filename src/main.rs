mod app;
mod db;
mod finance;
mod ingestion;
mod models;
mod reportes;
mod server;
mod ui;

use app::{AppConfig, AppState};
use std::net::SocketAddr;
use std::sync::atomic::Ordering;
use std::time::Duration;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===========================================================");
    println!("  MOTOR ANALÍTICO DE PORTAFOLIO Y BASE DE DATOS (RUST)");
    println!("  Club de Finanzas UBA — Aplicación en Pantalla Completa");
    println!("===========================================================\n");

    let db_path = "portafolio.db";
    let dias_anualizacion = 252.0;

    let state = AppState::new(db_path, dias_anualizacion)?;
    println!("✓ Base de datos SQLite conectada: {}", db_path);

    let config = AppConfig::default();
    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    
    let listener = match TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(_) => {
            // Intentar con puerto dinámico en caso de conflicto
            let fallback_addr: SocketAddr = format!("{}:0", config.host).parse()?;
            TcpListener::bind(fallback_addr).await?
        }
    };

    let local_addr = listener.local_addr()?;
    let url = format!("http://{}", local_addr);

    println!("✓ Servidor HTTP cuantitativo iniciado en: {}", url);
    println!("✓ Abriendo aplicación en navegador...\n");

    let app_router = server::crear_router(state);

    // Lanzar el navegador en modo App / Pantalla Completa
    let app_url = url.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(300)).await;
        app::abrir_navegador_pantalla_completa(&app_url);
    });

    println!("La consola se cerrará automáticamente al cerrar la ventana de la aplicación.\n");
    axum::serve(listener, app_router).await?;

    Ok(())
}
