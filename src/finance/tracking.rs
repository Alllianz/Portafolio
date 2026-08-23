use rusqlite::Connection;
use std::collections::HashMap;
use std::error::Error;

use crate::finance::arbitraje;
use crate::models::{TrackingRequest, TrackingResponse};

pub fn ejecutar_seguimiento_cartera(
    req: &TrackingRequest,
    conn: &Connection,
    dias_anualizacion: f64,
) -> Result<TrackingResponse, Box<dyn Error + Send + Sync>> {
    let spy_data = crate::db::obtener_precios_ticker_desde_fecha(conn, "SPY", &req.fecha_inicio)?;
    if spy_data.is_empty() {
        return Err(format!(
            "No se encontraron cotizaciones para SPY desde la fecha {}. Asegúrese de descargar SPY.",
            req.fecha_inicio
        ).into());
    }

    let time_labels: Vec<String> = spy_data.iter().map(|(f, _)| f.clone()).collect();
    let n_velas = spy_data.len();
    let actual_start_date = time_labels.first().cloned().unwrap_or_else(|| req.fecha_inicio.clone());

    let mut series_map = HashMap::new();
    let spy_prices: Vec<f64> = spy_data.iter().map(|(_, p)| *p).collect();
    series_map.insert("SPY".to_string(), spy_prices);

    let tickers = req.tickers.clone();
    if tickers.is_empty() {
        return Err("Debe ingresar al menos un ticker para el seguimiento.".into());
    }

    for t in &tickers {
        let t_data = crate::db::obtener_precios_ticker_desde_fecha(conn, t, &req.fecha_inicio)?;
        let mut prices: Vec<f64> = t_data.iter().map(|(_, p)| *p).collect();
        if prices.len() < n_velas {
            let first = *prices.first().unwrap_or(&100.0);
            let mut pad = vec![first; n_velas - prices.len()];
            pad.extend(prices);
            prices = pad;
        }
        series_map.insert(t.clone(), prices);
    }

    // Normalizar ponderaciones
    let sum_pesos: f64 = req.pesos.iter().sum();
    let pesos_norm: Vec<f64> = if sum_pesos > 0.0 {
        req.pesos.iter().map(|p| p / sum_pesos).collect()
    } else {
        vec![1.0 / tickers.len() as f64; tickers.len()]
    };

    // Frecuencia de rebalanceo
    let rebalance_step = match req.rebalance_freq.to_lowercase().as_str() {
        "diario" | "daily" => 1,
        "semanal" | "weekly" => 5,
        "mensual" | "monthly" => 21,
        "trimestral" | "quarterly" => 63,
        "semestral" => 126,
        "anual" | "yearly" => 252,
        _ => usize::MAX, // sin_rebalanceo (Buy & Hold)
    };

    // Curva de equity de la cartera ponderada con rebalanceo (base 1.0)
    let mut port_equity = Vec::with_capacity(n_velas);
    port_equity.push(1.0);

    let mut last_rebal_idx = 0;
    let mut capital_at_rebal = 1.0;

    for d in 1..n_velas {
        let mut val_dia = 0.0;
        for (i, t) in tickers.iter().enumerate() {
            let p_rebal = series_map[t][last_rebal_idx].max(1e-12);
            let p_curr = series_map[t][d];
            val_dia += pesos_norm[i] * (p_curr / p_rebal);
        }
        let current_equity = capital_at_rebal * val_dia;
        port_equity.push(current_equity);

        if rebalance_step != usize::MAX && (d - last_rebal_idx) >= rebalance_step {
            last_rebal_idx = d;
            capital_at_rebal = current_equity;
        }
    }

    // Curva de equity de SPY (base 1.0)
    let spy_init = series_map["SPY"][0].max(0.0001);
    let spy_equity: Vec<f64> = series_map["SPY"].iter().map(|p| p / spy_init).collect();

    let tracking_gain_pct = (port_equity.last().copied().unwrap_or(1.0) - 1.0) * 100.0;

    // Retornos Diarios
    let n_rets = n_velas - 1;
    let mut daily_rets = Vec::with_capacity(n_rets);
    for i in 1..n_velas {
        let r = (port_equity[i] / port_equity[i - 1].max(1e-12)) - 1.0;
        daily_rets.push(r);
    }

    let rf_anual = req.rf_rate;
    let mean_ret = daily_rets.iter().sum::<f64>() / n_rets.max(1) as f64;
    let ann_ret = mean_ret * dias_anualizacion;
    let var_ret = daily_rets.iter().map(|r| (r - mean_ret).powi(2)).sum::<f64>() / (n_rets - 1).max(1) as f64;
    let vol_anual = var_ret.sqrt() * dias_anualizacion.sqrt();

    let neg_rets: Vec<f64> = daily_rets.iter().copied().filter(|&r| r < 0.0).collect();
    let downside_var = if !neg_rets.is_empty() {
        neg_rets.iter().map(|r| r * r).sum::<f64>() / neg_rets.len() as f64
    } else {
        0.0001
    };
    let downside_vol_anual = downside_var.sqrt() * dias_anualizacion.sqrt();

    let tracking_sharpe = (ann_ret - rf_anual) / vol_anual.max(0.001);
    let tracking_sortino = (ann_ret - rf_anual) / downside_vol_anual.max(0.001);

    // Drawdown y estancamiento
    let mut peak = port_equity[0];
    let mut max_dd = 0.0;
    let mut periodos_estancamiento = Vec::new();
    let mut inicio_estancamiento: Option<usize> = None;

    for (i, &val) in port_equity.iter().enumerate() {
        if val >= peak {
            peak = val;
            if let Some(inicio) = inicio_estancamiento {
                periodos_estancamiento.push(i - inicio);
                inicio_estancamiento = None;
            }
        } else {
            let dd = (peak - val) / peak.max(1e-12);
            if dd > max_dd {
                max_dd = dd;
            }
            if inicio_estancamiento.is_none() {
                inicio_estancamiento = Some(i);
            }
        }
    }

    if let Some(inicio) = inicio_estancamiento {
        periodos_estancamiento.push(port_equity.len() - 1 - inicio);
    }

    let tracking_max_dd_pct = max_dd * 100.0;
    let tracking_avg_stagnation_days = if !periodos_estancamiento.is_empty() {
        periodos_estancamiento.iter().sum::<usize>() as f64 / periodos_estancamiento.len() as f64
    } else {
        0.0
    };
    let tracking_max_stagnation_days = periodos_estancamiento.iter().copied().max().unwrap_or(0);

    let mut retornos_map = HashMap::new();
    for t in &tickers {
        let prices = &series_map[t];
        let mut full_rets = Vec::with_capacity(n_rets);
        for r in 1..prices.len() {
            full_rets.push((prices[r] / prices[r - 1].max(1e-12)) - 1.0);
        }
        retornos_map.insert(t.clone(), full_rets);
    }

    let spy_prices_ref = &series_map["SPY"];
    let mut spy_full_rets = Vec::with_capacity(n_rets);
    for r in 1..spy_prices_ref.len() {
        spy_full_rets.push((spy_prices_ref[r] / spy_prices_ref[r - 1].max(1e-12)) - 1.0);
    }
    retornos_map.insert("SPY".to_string(), spy_full_rets);

    let n_activos = tickers.len();
    let esperados = vec![ann_ret; n_activos];
    let vols = vec![vol_anual; n_activos];
    let downside_vols = vec![downside_vol_anual; n_activos];
    let betas = vec![1.0; n_activos];
    let capm_returns = vec![ann_ret; n_activos];

    let ratios = arbitraje::inicializar_ratios_cedear();
    let mut p_ars = HashMap::new();
    let mut p_usd = HashMap::new();
    let mut pesos_map = HashMap::new();

    for (i, ticker) in tickers.iter().enumerate() {
        let base_usd = series_map[ticker].last().copied().unwrap_or(100.0);
        let ratio = ratios.get(ticker).copied().unwrap_or(0.1);
        let base_ars = base_usd * ratio * req.ccl_ref;
        p_ars.insert(ticker.clone(), base_ars);
        p_usd.insert(ticker.clone(), base_usd);
        pesos_map.insert(ticker.clone(), pesos_norm[i]);
    }

    let (tc_cartera_ponderado, spread_ccl) = match arbitraje::calcular_ccl_implicito(&p_ars, &p_usd, &ratios) {
        Ok(ccl) => arbitraje::evaluar_spread_arbitraje(&ccl, &pesos_map, req.ccl_ref),
        Err(_) => (req.ccl_ref, 0.0),
    };

    Ok(TrackingResponse {
        success: true,
        message: format!("Seguimiento de portafolio calculado exitosamente desde {}", actual_start_date),
        tickers,
        pesos: pesos_norm.clone(),
        fecha_inicio: req.fecha_inicio.clone(),
        actual_start_date,
        n_velas,
        tracking_gain_pct,
        tracking_max_dd_pct,
        tracking_sharpe,
        tracking_sortino,
        tracking_avg_stagnation_days,
        tracking_max_stagnation_days,
        ann_ret,
        vol_anual,
        tc_cartera_ponderado,
        spread_ccl,
        port_equity_curve: port_equity,
        spy_equity_curve: spy_equity,
        time_labels,
        series_map,
        retornos_map,
        esperados,
        vols,
        downside_vols,
        betas,
        capm_returns,
        pesos_sharpe: pesos_norm.clone(),
        pesos_sortino: pesos_norm,
        port_return_sharpe: ann_ret,
        port_vol_sharpe: vol_anual,
        sharpe_ratio: tracking_sharpe,
        sortino_ratio: tracking_sortino,
        var_95: max_dd,
        ccl_ref: req.ccl_ref,
        rf_rate: rf_anual,
        frontera_puntos: vec![(vol_anual, ann_ret)],
        matriz_correlacion: vec![vec![1.0; n_activos]; n_activos],
    })
}
