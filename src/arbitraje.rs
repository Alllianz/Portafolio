use std::collections::HashMap;

/// Ratios de conversión de CEDEARs a acciones de USA (según fila 2 de arbitraje)
pub fn inicializar_ratios_cedear() -> HashMap<String, f64> {
    let mut ratios = HashMap::new();
    ratios.insert("PEP".to_string(), 1.0 / 18.0);
    ratios.insert("VIST".to_string(), 1.0 / 3.0);
    ratios.insert("IWM".to_string(), 1.0 / 10.0);
    ratios.insert("DIA".to_string(), 1.0 / 20.0);
    ratios.insert("SPY".to_string(), 1.0 / 20.0);
    ratios.insert("QQQ".to_string(), 1.0 / 20.0);
    ratios.insert("CEG".to_string(), 1.0 / 10.0);
    ratios.insert("SO".to_string(), 1.0 / 5.0);
    ratios.insert("YPF".to_string(), 1.0 / 1.0);
    ratios.insert("XOM".to_string(), 1.0 / 5.0);
    ratios.insert("GEV".to_string(), 1.0 / 10.0);
    ratios.insert("AAPL".to_string(), 1.0 / 10.0);
    ratios.insert("MSFT".to_string(), 1.0 / 30.0);
    ratios.insert("GOOG".to_string(), 1.0 / 58.0);
    ratios
}

/// Consulta dinámica del ratio CEDEAR. Si es un activo nuevo (ej. RGTI), resuelve dinámicamente mediante consulta o ratio estándar 1:10
pub fn obtener_ratio_cedear(ticker: &str) -> f64 {
    let ratios = inicializar_ratios_cedear();
    if let Some(&ratio) = ratios.get(ticker) {
        ratio
    } else {
        1.0 / 10.0
    }
}

/// Calcula el CCL implícito para cada CEDEAR = Precio_ARS / (Precio_USD * Ratio)
pub fn calcular_ccl_implicito(
    precios_ars: &HashMap<String, f64>,
    precios_usd: &HashMap<String, f64>,
    ratios: &HashMap<String, f64>,
) -> Result<HashMap<String, f64>, String> {
    let mut ccl_map = HashMap::new();
    
    for (ticker, p_ars) in precios_ars {
        let p_usd = precios_usd.get(ticker).ok_or_else(|| {
            format!("Precio USD no disponible para el ticker {}", ticker)
        })?;
        
        let ratio = ratios.get(ticker).cloned().unwrap_or_else(|| obtener_ratio_cedear(ticker));
        
        if *p_usd <= 0.0 || ratio <= 0.0 {
            return Err(format!("Precio USD o ratio inválido para el ticker {}", ticker));
        }
        
        let ccl = p_ars / (p_usd * ratio);
        ccl_map.insert(ticker.clone(), ccl);
    }
    
    Ok(ccl_map)
}

/// Evalúa el spread de arbitraje contra el CCL de referencia.
pub fn evaluar_spread_arbitraje(
    ccl_implicito: &HashMap<String, f64>,
    pesos: &HashMap<String, f64>,
    ccl_referencia: f64,
) -> (f64, f64) {
    let mut tc_cartera = 0.0;
    
    for (ticker, peso) in pesos {
        if let Some(&ccl) = ccl_implicito.get(ticker) {
            tc_cartera += peso * ccl;
        }
    }
    
    let spread = if ccl_referencia > 0.0 {
        ((tc_cartera / ccl_referencia) - 1.0) * 100.0
    } else {
        0.0
    };
    
    (tc_cartera, spread)
}
