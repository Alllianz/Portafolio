use std::process::Command;

/// Abre el navegador predeterminado o Microsoft Edge / Google Chrome en modo App / Pantalla Completa sin bloquear el runtime.
pub fn abrir_navegador_pantalla_completa(url: &str) {
    #[cfg(target_os = "windows")]
    {
        // 1. Intentar abrir en modo App con Microsoft Edge
        let edge_paths = [
            "msedge",
            r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
            r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
        ];

        for &exe in &edge_paths {
            if Command::new(exe)
                .args([
                    &format!("--app={}", url),
                    "--start-maximized",
                    "--window-size=1920,1080",
                ])
                .spawn()
                .is_ok()
            {
                println!("✓ Aplicación ejecutada en ventana nativa con Microsoft Edge.");
                return;
            }
        }

        // 2. Intentar abrir con Chrome en modo App
        let chrome_paths = [
            "chrome",
            r"C:\Program Files\Google\Chrome\Application\chrome.exe",
            r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
        ];

        for &exe in &chrome_paths {
            if Command::new(exe)
                .args([
                    &format!("--app={}", url),
                    "--start-maximized",
                ])
                .spawn()
                .is_ok()
            {
                println!("✓ Aplicación ejecutada en ventana nativa con Google Chrome.");
                return;
            }
        }

        // 3. Fallback con comando start de Windows
        let _ = Command::new("cmd")
            .args(["/C", "start", "", url])
            .spawn();
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = open::that(url);
    }
}
