use nalgebra::DMatrix;
use rusqlite::Connection;
use std::collections::HashMap;
use std::error::Error;

use crate::finance::{optimizacion, stats};
use crate::models::{IsOosRequest, IsOosResponse};

/// Parsea la especificación de benchmark (ej. "SPY", "QQQ" o "SPY: 60, QQQ: 40")
pub fn parsear_benchmark(bm_str: &str) -> Vec<(String, f64)> {
    let clean = bm_str.trim();
    if clean.is_empty() {
        return vec![("SPY".to_string(), 1.0)];
    }

    if !clean.contains(':') {
        let sym = clean.to_uppercase();
        return vec![(sym, 1.0)];
    }

    let mut items = Vec::new();
    let parts = clean.split(',');
    for p in parts {
        let mut sub = p.split(':');
        if let (Some(sym), Some(w_str)) = (sub.next(), sub.next()) {
            let s = sym.trim().to_uppercase();
            let w = w_str.trim().parse::<f64>().unwrap_or(1.0);
            if !s.is_empty() && w > 0.0 {
                items.push((s, w));
            }
        }
    }

    if items.is_empty() {
        return vec![("SPY".to_string(), 1.0)];
    }

    let sum: f64 = items.iter().map(|(_, w)| *w).sum();
    if sum > 0.0 {
        items.into_iter().map(|(s, w)| (s, w / sum)).collect()
    } else {
        vec![("SPY".to_string(), 1.0)]
    }
}

/// Calcula retorno anualizado, volatilidad anualizada, Sharpe, Max Drawdown y ganancia total de una curva de equity
fn calcular_metricas_curva(
    equity: &[f64],
    dias_anualizacion: f64,
    rf_rate: f64,
) -> (f64, f64, f64, f64, f64) {
    if equity.len() < 2 {
        return (0.0, 0.0, 0.0, 0.0, 0.0);
    }

    let n_velas = equity.len();
    let n_rets = n_velas - 1;
    let mut daily_rets = Vec::with_capacity(n_rets);
    for i in 1..n_velas {
        let r = (equity[i] / equity[i - 1].max(1e-12)) - 1.0;
        daily_rets.push(r);
    }

    let mean_ret = daily_rets.iter().sum::<f64>() / n_rets as f64;
    let ann_ret = mean_ret * dias_anualizacion;

    let var_ret = daily_rets.iter().map(|r| (r - mean_ret).powi(2)).sum::<f64>() / (n_rets - 1).max(1) as f64;
    let ann_vol = var_ret.sqrt() * dias_anualizacion.sqrt();

    let sharpe = (ann_ret - rf_rate) / ann_vol.max(0.001);
    let total_gain = ((equity.last().unwrap_or(&100.0) / equity.first().unwrap_or(&100.0).max(1e-12)) - 1.0) * 100.0;

    let mut peak = equity[0];
    let mut max_dd = 0.0;
    for &v in equity {
        if v > peak {
            peak = v;
        } else {
            let dd = (peak - v) / peak.max(1e-12);
            if dd > max_dd {
                max_dd = dd;
            }
        }
    }

    (ann_ret, ann_vol, sharpe, max_dd * 100.0, total_gain)
}

pub fn ejecutar_analisis_is_oos(
    req: &IsOosRequest,
    conn: &Connection,
    dias_anualizacion: f64,
) -> Result<IsOosResponse, Box<dyn Error + Send + Sync>> {
    let is_len = req.is_velas.max(20);
    let oos_len = req.oos_velas.max(20);
    let total_velas = is_len + oos_len + 1;

    let portfolio_tickers: Vec<String> = req.tickers.iter().map(|s| s.trim().to_uppercase()).filter(|s| !s.is_empty()).collect();
    if portfolio_tickers.is_empty() {
        return Err("Debe ingresar al menos un ticker para el análisis IS / OOS.".into());
    }

    let bm_components = parsear_benchmark(&req.benchmark);
    let bm_nombre = if bm_components.len() == 1 {
        bm_components[0].0.clone()
    } else {
        format!("Cartera Benchmark ({})", bm_components.iter().map(|(s, w)| format!("{}: {:.0}%", s, w * 100.0)).collect::<Vec<_>>().join(", "))
    };

    // Recolectar todos los tickers necesarios (Cartera + Benchmark + SPY)
    let mut all_tickers = portfolio_tickers.clone();
    for (s, _) in &bm_components {
        if !all_tickers.contains(s) {
            all_tickers.push(s.clone());
        }
    }
    if !all_tickers.contains(&"SPY".to_string()) {
        all_tickers.push("SPY".to_string());
    }

    // Obtener series de precios para todos los activos
    let mut series_map: HashMap<String, Vec<f64>> = HashMap::new();

    // Usamos SPY o el primer ticker como referencia temporal
    let ref_ticker = all_tickers.first().unwrap();
    let ref_data = crate::db::obtener_precios_ticker(conn, ref_ticker, total_velas)?;
    if ref_data.len() < total_velas {
        return Err(format!(
            "Se requieren {} velas de histórico para IS+OOS (Disponibles para {}: {}). Descargue el histórico primero.",
            total_velas, ref_ticker, ref_data.len()
        ).into());
    }
    let ref_time_labels: Vec<String> = ref_data.iter().map(|(f, _)| f.clone()).collect();
    series_map.insert(ref_ticker.clone(), ref_data.iter().map(|(_, p)| *p).collect());

    for t in &all_tickers {
        if t == ref_ticker { continue; }
        let t_data = crate::db::obtener_precios_ticker(conn, t, total_velas)?;
        let mut prices: Vec<f64> = t_data.iter().map(|(_, p)| *p).collect();
        if prices.len() < total_velas {
            let first = *prices.first().unwrap_or(&100.0);
            let mut pad = vec![first; total_velas - prices.len()];
            pad.extend(prices);
            prices = pad;
        }
        series_map.insert(t.clone(), prices);
    }

    let n_activos = portfolio_tickers.len();

    // -------------------------------------------------------------
    // FASE 1: IN-SAMPLE (Calibración & Optimización de Pesos)
    // -------------------------------------------------------------
    let is_rows = is_len + 1;
    let mut is_matrix_precios = DMatrix::zeros(is_rows, n_activos);
    for (col_idx, t) in portfolio_tickers.iter().enumerate() {
        let prices = &series_map[t];
        for row_idx in 0..is_rows {
            is_matrix_precios[(row_idx, col_idx)] = prices[row_idx];
        }
    }

    let is_retornos = stats::calcular_retornos_diarios(&is_matrix_precios);
    let is_esperados_diarios = stats::calcular_retorno_esperado(&is_retornos);
    let is_covarianza = stats::calcular_matriz_covarianza(&is_retornos, false);

    let rf_anual = req.rf_rate;
    let rf_diaria = rf_anual / dias_anualizacion;
    let limites = vec![req.min_bound; n_activos];

    let res_sharpe_is = optimizacion::optimizar_maximo_sharpe(&is_esperados_diarios, &is_covarianza, rf_diaria, &limites, 0);
    let pesos_sharpe = res_sharpe_is.pesos.as_slice().to_vec();
    let pesos_sortino = pesos_sharpe.clone();

    // Curva de Equity In-Sample (Base 100)
    let mut is_port_equity = Vec::with_capacity(is_rows);
    is_port_equity.push(100.0);

    for d in 1..is_rows {
        let mut val_dia = 0.0;
        for (i, t) in portfolio_tickers.iter().enumerate() {
            let p_init = series_map[t][0].max(1e-12);
            let p_curr = series_map[t][d];
            val_dia += pesos_sharpe[i] * (p_curr / p_init);
        }
        is_port_equity.push(100.0 * val_dia);
    }

    let (is_return, is_vol, is_sharpe, is_max_dd, is_gain_pct) =
        calcular_metricas_curva(&is_port_equity, dias_anualizacion, rf_anual);

    // -------------------------------------------------------------
    // FASE 2: OUT-OF-SAMPLE (Simulación con Rebalanceo Periódico)
    // -------------------------------------------------------------
    let oos_start_idx = is_len;
    let oos_total_steps = oos_len;

    let rebal_step = match req.rebalance_freq.to_lowercase().as_str() {
        "diario" => 1,
        "semanal" => 5,
        "mensual" => 21,
        "sin_rebalanceo" => oos_total_steps + 10,
        _ => 21, // Default mensual
    };

    let mut oos_port_equity = Vec::with_capacity(oos_total_steps + 1);
    let last_is_val = *is_port_equity.last().unwrap_or(&100.0);
    oos_port_equity.push(last_is_val);

    // Simulación del rebalanceo paso a paso
    let mut current_capital = last_is_val;
    let mut current_rebal_idx = 0;

    for day in 1..=oos_total_steps {
        // Si alcanzamos una fecha de rebalanceo, consolidamos el capital y reseteamos la base
        if (day - 1) % rebal_step == 0 {
            current_capital = *oos_port_equity.last().unwrap_or(&current_capital);
            current_rebal_idx = oos_start_idx + day - 1;
        }

        let abs_day_idx = oos_start_idx + day;
        let mut val_rel = 0.0;
        for (i, t) in portfolio_tickers.iter().enumerate() {
            let p_rebal = series_map[t][current_rebal_idx].max(1e-12);
            let p_curr = series_map[t][abs_day_idx];
            val_rel += pesos_sharpe[i] * (p_curr / p_rebal);
        }
        oos_port_equity.push(current_capital * val_rel);
    }

    let (oos_return, oos_vol, oos_sharpe, oos_max_dd, oos_gain_pct) =
        calcular_metricas_curva(&oos_port_equity, dias_anualizacion, rf_anual);

    // -------------------------------------------------------------
    // FASE 3: BENCHMARK (Evaluación Continua a lo largo de IS y OOS)
    // -------------------------------------------------------------
    let mut bm_full_equity = Vec::with_capacity(total_velas);
    bm_full_equity.push(100.0);

    for d in 1..total_velas {
        let mut val_dia = 0.0;
        for (sym, w) in &bm_components {
            let p_init = series_map[sym][0].max(1e-12);
            let p_curr = series_map[sym][d];
            val_dia += w * (p_curr / p_init);
        }
        bm_full_equity.push(100.0 * val_dia);
    }

    let bm_is_slice = &bm_full_equity[0..is_rows];
    let (bm_is_return, bm_is_vol, bm_is_sharpe, _, _) =
        calcular_metricas_curva(bm_is_slice, dias_anualizacion, rf_anual);

    let bm_oos_slice = &bm_full_equity[oos_start_idx..total_velas];
    let (bm_oos_return, bm_oos_vol, bm_oos_sharpe, _, bm_oos_gain_pct) =
        calcular_metricas_curva(bm_oos_slice, dias_anualizacion, rf_anual);

    // -------------------------------------------------------------
    // FASE 4: ENSAMBLAJE DE CURVA COMPLETA Y RESPUESTA
    // -------------------------------------------------------------
    let mut port_full_equity = is_port_equity.clone();
    // Añadimos la parte OOS a partir del día 1 de OOS
    for &v in &oos_port_equity[1..] {
        port_full_equity.push(v);
    }

    Ok(IsOosResponse {
        success: true,
        message: format!(
            "Validación IS/OOS con rebalanceo '{}' y Benchmark '{}' completada con éxito.",
            req.rebalance_freq, bm_nombre
        ),
        tickers: portfolio_tickers,
        benchmark_nombre: bm_nombre,
        rebalance_freq: req.rebalance_freq.clone(),
        is_return,
        is_vol,
        is_sharpe,
        is_max_dd,
        is_gain_pct,
        oos_return,
        oos_vol,
        oos_sharpe,
        oos_max_dd,
        oos_gain_pct,
        bm_is_return,
        bm_is_vol,
        bm_is_sharpe,
        bm_oos_return,
        bm_oos_vol,
        bm_oos_sharpe,
        bm_oos_gain_pct,
        pesos_sharpe,
        pesos_sortino,
        full_time_labels: ref_time_labels,
        port_full_equity_curve: port_full_equity,
        bm_full_equity_curve: bm_full_equity,
        split_index: is_len,
        ccl_ref: req.ccl_ref,
        rf_rate: rf_anual,
    })
}
