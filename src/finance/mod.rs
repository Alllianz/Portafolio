pub mod arbitraje;
pub mod capm;
pub mod frontera;
pub mod is_oos;
pub mod optimizacion;
pub mod stats;
pub mod tracking;

use nalgebra::DMatrix;
use rusqlite::Connection;
use std::collections::HashMap;
use std::error::Error;

use crate::models::{OptimizarRequest, OptimizarResponse};

/// Ejecuta el análisis de Markowitz, CAPM, Sharpe, Sortino, VaR 95% y CCL implícito.
pub fn ejecutar_optimizacion_cartera(
    req: &OptimizarRequest,
    conn: &Connection,
    dias_anualizacion: f64,
) -> Result<OptimizarResponse, Box<dyn Error + Send + Sync>> {
    let mut tickers_list = req.tickers.clone();
    if tickers_list.is_empty() {
        return Err("Debe ingresar al menos un ticker para optimizar.".into());
    }

    if !tickers_list.contains(&"SPY".to_string()) {
        tickers_list.insert(0, "SPY".to_string());
    }

    let spy_data = crate::db::obtener_precios_ticker(conn, "SPY", req.n_velas + 1)?;
    if spy_data.len() < 2 {
        return Err(format!(
            "No hay suficientes velas guardadas para SPY (se requieren al menos {}). Descárguelas primero.",
            req.n_velas
        ).into());
    }

    let time_labels: Vec<String> = spy_data.iter().map(|(f, _)| f.clone()).collect();
    let n_precios_reales = spy_data.len();

    let mut series_map = HashMap::new();
    let spy_prices: Vec<f64> = spy_data.iter().map(|(_, p)| *p).collect();
    series_map.insert("SPY".to_string(), spy_prices);

    let portfolio_tickers: Vec<String> = req.tickers.iter().filter(|t| *t != "SPY").cloned().collect();
    if portfolio_tickers.is_empty() {
        return Err("Debe especificar al menos un activo además de SPY.".into());
    }

    for t in &portfolio_tickers {
        let t_data = crate::db::obtener_precios_ticker(conn, t, n_precios_reales)?;
        let mut prices: Vec<f64> = t_data.iter().map(|(_, p)| *p).collect();
        if prices.len() < n_precios_reales {
            let first = *prices.first().unwrap_or(&100.0);
            let mut pad = vec![first; n_precios_reales - prices.len()];
            pad.extend(prices);
            prices = pad;
        }
        series_map.insert(t.clone(), prices);
    }

    let spy_matrix = DMatrix::from_column_slice(n_precios_reales, 1, &series_map["SPY"]);
    let spy_ret_mat = stats::calcular_retornos_diarios(&spy_matrix);
    let spy_ret_vec = spy_ret_mat.column(0).into_owned();

    let n_activos = portfolio_tickers.len();
    let mut matrix_precios = DMatrix::zeros(n_precios_reales, n_activos);
    for (col_idx, t) in portfolio_tickers.iter().enumerate() {
        let prices = &series_map[t];
        for row_idx in 0..n_precios_reales {
            matrix_precios[(row_idx, col_idx)] = prices[row_idx];
        }
    }

    let retornos = stats::calcular_retornos_diarios(&matrix_precios);
    let esperados_diarios = stats::calcular_retorno_esperado(&retornos);
    let covarianza = stats::calcular_matriz_covarianza(&retornos, req.poblacional);
    let betas_vec = stats::calcular_betas(&retornos, &spy_ret_vec, req.poblacional);

    let mut esperados = Vec::new();
    let mut vols = Vec::new();
    let mut downside_vols = Vec::new();
    let mut retornos_map = HashMap::new();

    let n_retornos = n_precios_reales - 1;
    for (i, t) in portfolio_tickers.iter().enumerate() {
        let exp_anual = esperados_diarios[i] * dias_anualizacion;
        esperados.push(exp_anual);

        let var_anual = covarianza[(i, i)] * dias_anualizacion;
        vols.push(var_anual.sqrt());

        let col_rets: Vec<f64> = (0..n_retornos).map(|r| retornos[(r, i)]).collect();
        let neg_rets: Vec<f64> = col_rets.iter().copied().filter(|&r| r < 0.0).collect();
        let down_var = if !neg_rets.is_empty() {
            neg_rets.iter().map(|r| r * r).sum::<f64>() / neg_rets.len() as f64
        } else {
            0.0001
        };
        downside_vols.push((down_var * dias_anualizacion).sqrt());

        retornos_map.insert(t.clone(), col_rets);
    }

    let spy_col_rets: Vec<f64> = (0..n_retornos).map(|r| spy_ret_vec[r]).collect();
    retornos_map.insert("SPY".to_string(), spy_col_rets);

    let rf_anual = req.rf_rate;
    let rf_diaria = rf_anual / dias_anualizacion;
    let limites = vec![req.min_bound; n_activos];

    let res_sharpe = optimizacion::optimizar_maximo_sharpe(&esperados_diarios, &covarianza, rf_diaria, &limites, 0);

    let pos_sortino: Vec<f64> = esperados.iter().zip(downside_vols.iter())
        .map(|(e, d_vol)| ((e - rf_anual) / d_vol.max(0.001)).max(0.001))
        .collect();
    let sum_pos_sortino: f64 = pos_sortino.iter().sum();
    let sobrante = (1.0 - req.min_bound * n_activos as f64).max(0.0);

    let mut pesos_sortino = vec![req.min_bound; n_activos];
    for i in 0..n_activos {
        if sum_pos_sortino > 0.0 {
            pesos_sortino[i] += (pos_sortino[i] / sum_pos_sortino) * sobrante;
        }
    }

    let pesos_sharpe = res_sharpe.pesos.as_slice().to_vec();
    let betas = betas_vec.as_slice().to_vec();

    let port_return_sharpe = pesos_sharpe.iter().zip(esperados.iter()).map(|(w, e)| w * e).sum::<f64>();
    let port_vol_sharpe = res_sharpe.volatilidad * dias_anualizacion.sqrt();
    let port_downside_vol = pesos_sharpe.iter().zip(downside_vols.iter()).map(|(w, d)| w * d).sum::<f64>();

    let sharpe_ratio = (port_return_sharpe - rf_anual) / port_vol_sharpe.max(0.001);
    let sortino_ratio = (port_return_sharpe - rf_anual) / port_downside_vol.max(0.001);

    let var_95 = (1.645 * (port_vol_sharpe / dias_anualizacion.sqrt())) - (port_return_sharpe / dias_anualizacion);

    let rm_diario = spy_ret_vec.mean();
    let capm_returns = capm::calcular_retorno_capm(rf_anual, &betas_vec, rm_diario, dias_anualizacion).as_slice().to_vec();

    let ratios = arbitraje::inicializar_ratios_cedear();
    let mut p_ars = HashMap::new();
    let mut p_usd = HashMap::new();
    let mut pesos_map = HashMap::new();

    for (i, ticker) in portfolio_tickers.iter().enumerate() {
        let base_usd = series_map[ticker].last().copied().unwrap_or(100.0);
        let ratio = ratios.get(ticker).copied().unwrap_or(0.1);
        let base_ars = base_usd * ratio * req.ccl_ref;
        p_ars.insert(ticker.clone(), base_ars);
        p_usd.insert(ticker.clone(), base_usd);
        pesos_map.insert(ticker.clone(), pesos_sharpe[i]);
    }

    let (tc_cartera_ponderado, spread_ccl) = match arbitraje::calcular_ccl_implicito(&p_ars, &p_usd, &ratios) {
        Ok(ccl) => arbitraje::evaluar_spread_arbitraje(&ccl, &pesos_map, req.ccl_ref),
        Err(_) => (req.ccl_ref, 0.0),
    };

    let mut matriz_correlacion = vec![vec![0.0; n_activos]; n_activos];
    for r in 0..n_activos {
        let std_r = covarianza[(r, r)].sqrt().max(1e-12);
        for c in 0..n_activos {
            let std_c = covarianza[(c, c)].sqrt().max(1e-12);
            let corr = covarianza[(r, c)] / (std_r * std_c);
            matriz_correlacion[r][c] = (corr * 100.0).round() / 100.0;
        }
    }

    let frontera_puntos = frontera::calcular_frontera_eficiente(&esperados_diarios, &covarianza, &limites, dias_anualizacion, rf_anual, 30);

    Ok(OptimizarResponse {
        success: true,
        message: "Cartera optimizada exitosamente según Markowitz.".to_string(),
        tickers: portfolio_tickers,
        n_velas: req.n_velas,
        pesos_sharpe,
        pesos_sortino,
        esperados,
        vols,
        downside_vols,
        betas,
        capm_returns,
        port_return_sharpe,
        port_vol_sharpe,
        port_downside_vol,
        sharpe_ratio,
        sortino_ratio,
        var_95,
        tc_cartera_ponderado,
        spread_ccl,
        ccl_ref: req.ccl_ref,
        rf_rate: rf_anual,
        retornos_map,
        series_map,
        time_labels,
        frontera_puntos,
        matriz_correlacion,
    })
}
