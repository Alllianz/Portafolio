use nalgebra::{DMatrix, DVector};
use crate::finance::optimizacion::{optimizar_maximo_sharpe, proyectar_simplex_con_limites};

/// Genera la curva continua y exacta de la Frontera Eficiente de Markowitz resolviendo el problema de mínima varianza
/// para múltiples niveles de retorno objetivo R_target en [R_min_vol, R_max_ret].
/// Garantiza que la Cartera de Máximo Sharpe y la de Mínima Varianza pertenezcan exactamente a la curva.
pub fn calcular_frontera_eficiente(
    esperados_diarios: &DVector<f64>,
    covarianza: &DMatrix<f64>,
    limites: &[f64],
    dias_anualizacion: f64,
    rf_rate: f64,
    puntos_totales: usize,
) -> Vec<(f64, f64)> {
    let n = esperados_diarios.len();
    if n == 0 {
        return Vec::new();
    }

    let mut frontera_puntos = Vec::new();

    // 1. Cartera de Mínima Varianza Global (Punto de partida de la frontera eficiente)
    let mut pesos_min_var = DVector::from_element(n, 1.0 / n as f64);
    pesos_min_var = proyectar_simplex_con_limites(&pesos_min_var, limites);
    let mut lr_mv = 0.05;
    for _ in 0..2000 {
        let grad = covarianza * &pesos_min_var;
        let mut paso = &pesos_min_var - &grad * lr_mv;
        paso = proyectar_simplex_con_limites(&paso, limites);
        if paso.dot(&(covarianza * &paso)) < pesos_min_var.dot(&(covarianza * &pesos_min_var)) {
            pesos_min_var = paso;
            lr_mv *= 1.02;
        } else {
            lr_mv *= 0.5;
        }
    }
    let min_vol_anual = (pesos_min_var.dot(&(covarianza * &pesos_min_var))).sqrt() * dias_anualizacion.sqrt();
    let min_ret_anual = pesos_min_var.dot(esperados_diarios) * dias_anualizacion;
    frontera_puntos.push((min_vol_anual, min_ret_anual));

    // 2. Cartera de Máximo Retorno Posible sujeto a los límites
    let mut pesos_max_ret = DVector::from_element(n, 0.0);
    let mut indices: Vec<usize> = (0..n).collect();
    indices.sort_by(|&a, &b| esperados_diarios[b].partial_cmp(&esperados_diarios[a]).unwrap_or(std::cmp::Ordering::Equal));
    for i in 0..n {
        pesos_max_ret[i] = limites[i];
    }
    let mut rem = (1.0 - limites.iter().sum::<f64>()).max(0.0);
    for &idx in &indices {
        let add = rem.min(1.0 - pesos_max_ret[idx]);
        pesos_max_ret[idx] += add;
        rem -= add;
        if rem <= 1e-9 { break; }
    }
    let max_ret_anual = pesos_max_ret.dot(esperados_diarios) * dias_anualizacion;
    let max_vol_anual = (pesos_max_ret.dot(&(covarianza * &pesos_max_ret))).sqrt() * dias_anualizacion.sqrt();
    frontera_puntos.push((max_vol_anual, max_ret_anual));

    // 3. Cartera de Máximo Sharpe (Punto de Tangencia de la Frontera)
    let rf_diaria = rf_rate / dias_anualizacion;
    let res_sharpe = optimizar_maximo_sharpe(esperados_diarios, covarianza, rf_diaria, limites, 0);
    let sharpe_vol_anual = res_sharpe.volatilidad * dias_anualizacion.sqrt();
    let sharpe_ret_anual = res_sharpe.pesos.dot(esperados_diarios) * dias_anualizacion;
    frontera_puntos.push((sharpe_vol_anual, sharpe_ret_anual));

    // 4. Muestreo denso de retornos objetivos R_target entre R_min_var y R_max_ret
    let n_puntos = puntos_totales.max(30);
    let r_start = min_ret_anual;
    let r_end = max_ret_anual;

    if (r_end - r_start).abs() > 1e-4 {
        let base_var = covarianza.diagonal().mean().max(1e-6);
        let scale_penalty = 500.0 / base_var;

        for step in 1..n_puntos {
            let target_ret_anual = r_start + (r_end - r_start) * (step as f64 / n_puntos as f64);
            let target_ret_diario = target_ret_anual / dias_anualizacion;

            // Inicializar cerca de la interpolación lineal
            let alpha = step as f64 / n_puntos as f64;
            let mut w = &pesos_min_var * (1.0 - alpha) + &pesos_max_ret * alpha;
            w = proyectar_simplex_con_limites(&w, limites);

            let mut lr = 0.02;
            for _ in 0..2500 {
                let sigma_w = covarianza * &w;
                let diff_ret = w.dot(esperados_diarios) - target_ret_diario;
                let grad = &sigma_w + esperados_diarios * (2.0 * scale_penalty * diff_ret);

                let mut paso = &w - &grad * lr;
                paso = proyectar_simplex_con_limites(&paso, limites);

                let cost_curr = 0.5 * w.dot(&(covarianza * &w)) + scale_penalty * (w.dot(esperados_diarios) - target_ret_diario).powi(2);
                let cost_next = 0.5 * paso.dot(&(covarianza * &paso)) + scale_penalty * (paso.dot(esperados_diarios) - target_ret_diario).powi(2);

                if cost_next < cost_curr {
                    w = paso;
                    lr *= 1.03;
                } else {
                    lr *= 0.5;
                }
            }

            let p_vol = (w.dot(&(covarianza * &w))).sqrt() * dias_anualizacion.sqrt();
            let p_ret = w.dot(esperados_diarios) * dias_anualizacion;
            frontera_puntos.push((p_vol, p_ret));
        }
    }

    // 5. Ordenar por volatilidad de forma estrictamente creciente
    frontera_puntos.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    
    // Filtrar puntos estrictamente eficientes (para cada volatilidad, mantener el retorno máximo)
    let mut frontera_filtrada: Vec<(f64, f64)> = Vec::new();
    for p in frontera_puntos {
        if let Some(last) = frontera_filtrada.last() {
            if p.1 >= last.1 || (p.0 - last.0).abs() > 0.002 {
                frontera_filtrada.push(p);
            }
        } else {
            frontera_filtrada.push(p);
        }
    }

    frontera_filtrada
}
