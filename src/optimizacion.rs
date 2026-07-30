use nalgebra::{DMatrix, DVector};

pub struct ResultadoOptimizacion {
    pub pesos: DVector<f64>,
    pub retorno_esperado: f64,
    pub volatilidad: f64,
    pub sharpe_ratio: f64,
}

/// Optimiza los pesos del portafolio mediante Gradiente Proyectado Determinista (SLSQP determinista).
/// Minimiza una función de costo $f(w)$ sujeta a $\sum w_i = 1.0$ y $w_i \in [limite_i, 1.0]$.
fn optimizar_determinista<F, G>(
    n: usize,
    limites: &[f64],
    cost_fn: F,
    grad_fn: G,
) -> DVector<f64>
where
    F: Fn(&DVector<f64>) -> f64,
    G: Fn(&DVector<f64>) -> DVector<f64>,
{
    // Punto inicial: Proyección igual dentro de los límites
    let mut pesos = DVector::from_element(n, 1.0 / (n as f64));
    pesos = proyectar_simplex_con_limites(&pesos, limites);

    let mut lr = 0.01;
    let max_iter = 5_000;

    for _ in 0..max_iter {
        let grad = grad_fn(&pesos);
        let mut paso = &pesos - &grad * lr;
        paso = proyectar_simplex_con_limites(&paso, limites);

        if cost_fn(&paso) < cost_fn(&pesos) {
            pesos = paso;
            lr *= 1.05;
        } else {
            lr *= 0.5;
        }
    }

    proyectar_simplex_con_limites(&pesos, limites)
}

/// Proyecta un vector de pesos al simplex $\sum w_i = 1.0$ respetando los límites inferiores $w_i \ge limite_i$ e superiores $w_i \le 1.0$.
fn proyectar_simplex_con_limites(v: &DVector<f64>, limites: &[f64]) -> DVector<f64> {
    let n = v.len();
    let mut w = v.clone();
    
    // Aplicar cotas individuales
    for i in 0..n {
        let min_w = limites[i].clamp(0.0, 1.0);
        if w[i] < min_w {
            w[i] = min_w;
        } else if w[i] > 1.0 {
            w[i] = 1.0;
        }
    }

    // Ajustar por bisección/búsqueda de Lagrange lambda para $\sum w_i = 1.0$
    let mut low = -10.0;
    let mut high = 10.0;

    for _ in 0..100 {
        let mid = (low + high) / 2.0;
        let mut suma = 0.0;
        for i in 0..n {
            let val = (v[i] - mid).clamp(limites[i], 1.0);
            suma += val;
        }
        if suma > 1.0 {
            low = mid;
        } else {
            high = mid;
        }
    }

    let lambda = (low + high) / 2.0;
    let mut res = DVector::zeros(n);
    for i in 0..n {
        res[i] = (v[i] - lambda).clamp(limites[i], 1.0);
    }
    res
}

/// Optimiza el Sharpe Ratio determinísticamente usando álgebra lineal y gradientes.
pub fn optimizar_maximo_sharpe(
    retornos_esperados: &DVector<f64>,
    matriz_covarianza: &DMatrix<f64>,
    rf_diaria: f64,
    limites: &[f64],
    _num_simulaciones: usize,
) -> ResultadoOptimizacion {
    let n = retornos_esperados.len();

    // Maximizar Sharpe es equivalente a minimizar - (w^T mu - rf) / sqrt(w^T Sigma w)
    let cost_fn = |w: &DVector<f64>| {
        let ret = w.dot(retornos_esperados);
        let var = w.dot(&(matriz_covarianza * w));
        let vol = var.sqrt();
        if vol < 1e-12 {
            return 1e9;
        }
        - (ret - rf_diaria) / vol
    };

    let grad_fn = |w: &DVector<f64>| {
        let ret = w.dot(retornos_esperados);
        let sigma_w = matriz_covarianza * w;
        let var = w.dot(&sigma_w);
        let vol = var.sqrt();

        if vol < 1e-12 {
            return DVector::zeros(n);
        }

        let num = ret - rf_diaria;
        // Grad ( - num / vol ) = - ( vol * mu - num * (Sigma w / vol) ) / var
        let grad = - (&(retornos_esperados * vol) - &(&sigma_w * (num / vol))) / var;
        grad
    };

    let pesos = optimizar_determinista(n, limites, cost_fn, grad_fn);
    let retorno = pesos.dot(retornos_esperados);
    let varianza = pesos.dot(&(matriz_covarianza * &pesos));
    let volatilidad = varianza.sqrt();
    let sharpe = (retorno - rf_diaria) / volatilidad;

    ResultadoOptimizacion {
        pesos,
        retorno_esperado: retorno,
        volatilidad,
        sharpe_ratio: sharpe,
    }
}

/// Optimiza determinísticamente para obtener la mínima varianza bajo restricciones de límites.
pub fn optimizar_minima_varianza(
    matriz_covarianza: &DMatrix<f64>,
    retornos_esperados: &DVector<f64>,
    rf_diaria: f64,
    limites: &[f64],
    _num_simulaciones: usize,
) -> ResultadoOptimizacion {
    let n = limites.len();

    // Minimizar 1/2 w^T Sigma w
    let cost_fn = |w: &DVector<f64>| {
        0.5 * w.dot(&(matriz_covarianza * w))
    };

    let grad_fn = |w: &DVector<f64>| {
        matriz_covarianza * w
    };

    let pesos = optimizar_determinista(n, limites, cost_fn, grad_fn);
    let retorno = pesos.dot(retornos_esperados);
    let varianza = pesos.dot(&(matriz_covarianza * &pesos));
    let volatilidad = varianza.sqrt();
    let sharpe = (retorno - rf_diaria) / volatilidad;

    ResultadoOptimizacion {
        pesos,
        retorno_esperado: retorno,
        volatilidad,
        sharpe_ratio: sharpe,
    }
}

/// Calcula el Value at Risk (VaR) paramétrico.
/// nivel_confianza: ej 0.95 (para 95%)
pub fn calcular_value_at_risk(rendimiento_port: f64, volatilidad_port: f64, nivel_confianza: f64) -> f64 {
    // Usamos una aproximación de la distribución normal inversa (z-score)
    // Para 95%, z ~ 1.645. Para 99%, z ~ 2.33
    // Como no podemos usar scipy.stats.norm.ppf, implementamos una simple búsqueda binaria
    // o hardcodeamos los niveles más comunes
    let z = if (nivel_confianza - 0.99).abs() < 1e-4 {
        2.326
    } else if (nivel_confianza - 0.95).abs() < 1e-4 {
        1.645
    } else if (nivel_confianza - 0.90).abs() < 1e-4 {
        1.282
    } else {
        1.645 // Default 95%
    };
    
    // VaR = Z * sigma - mu (Asumiendo 1 periodo de tiempo)
    (z * volatilidad_port) - rendimiento_port
}
