/// Estimador de covarianza con contracción analítica de Ledoit-Wolf (1998/2004)
/// Garantiza matrices de covarianza semi-definidas positivas bien condicionadas.
#[derive(Debug, Clone)]
pub struct LedoitWolfEstimator;

impl LedoitWolfEstimator {
    /// Calcula la matriz de covarianza con Shrinkage analítico para N activos en una ventana W
    pub fn estimate_covariance(returns_matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let n_assets = returns_matrix.len();
        if n_assets == 0 {
            return Vec::new();
        }
        let t_obs = returns_matrix[0].len();
        if t_obs < 2 {
            // Matriz identidad diagonal como fallback
            let mut cov = vec![vec![0.0; n_assets]; n_assets];
            for i in 0..n_assets {
                cov[i][i] = 0.04;
            }
            return cov;
        }

        let t_f = t_obs as f64;

        // 1. Medias muestrales
        let mut means = vec![0.0; n_assets];
        for i in 0..n_assets {
            means[i] = returns_matrix[i].iter().sum::<f64>() / t_f;
        }

        // 2. Retornos centrados
        let mut x_c = vec![vec![0.0; t_obs]; n_assets];
        for i in 0..n_assets {
            for t in 0..t_obs {
                x_c[i][t] = returns_matrix[i][t] - means[i];
            }
        }

        // 3. Matriz de covarianza empírica muestral S
        let mut s = vec![vec![0.0; n_assets]; n_assets];
        for i in 0..n_assets {
            for j in 0..n_assets {
                let mut sum = 0.0;
                for t in 0..t_obs {
                    sum += x_c[i][t] * x_c[j][t];
                }
                s[i][j] = sum / (t_f - 1.0);
            }
        }

        // 4. Matriz target F (Varianza constante e identidad diagonal estructurada)
        let _mean_var: f64 = (0..n_assets).map(|i| s[i][i]).sum::<f64>() / (n_assets as f64);
        
        let mut total_corr = 0.0;
        let mut count_corr = 0.0;
        for i in 0..n_assets {
            for j in (i + 1)..n_assets {
                let std_i = s[i][i].max(1e-12).sqrt();
                let std_j = s[j][j].max(1e-12).sqrt();
                let r_ij = s[i][j] / (std_i * std_j);
                total_corr += r_ij;
                count_corr += 1.0;
            }
        }
        let avg_corr = if count_corr > 0.0 { (total_corr / count_corr).clamp(-0.99, 0.99) } else { 0.0 };

        let mut f_target = vec![vec![0.0; n_assets]; n_assets];
        for i in 0..n_assets {
            for j in 0..n_assets {
                if i == j {
                    f_target[i][j] = s[i][i];
                } else {
                    let std_i = s[i][i].max(1e-12).sqrt();
                    let std_j = s[j][j].max(1e-12).sqrt();
                    f_target[i][j] = avg_corr * std_i * std_j;
                }
            }
        }

        // 5. Parámetro pi-hat (varianza asintótica muestral)
        let mut pi_hat = 0.0;
        for i in 0..n_assets {
            for j in 0..n_assets {
                let mut p_ij = 0.0;
                for t in 0..t_obs {
                    let term = x_c[i][t] * x_c[j][t] - s[i][j];
                    p_ij += term * term;
                }
                pi_hat += p_ij / t_f;
            }
        }

        // 6. Gamma-hat (distancia Frobenius al cuadrado)
        let mut gamma_hat = 0.0;
        for i in 0..n_assets {
            for j in 0..n_assets {
                let diff = s[i][j] - f_target[i][j];
                gamma_hat += diff * diff;
            }
        }

        // 7. Intensidad óptima de contracción alpha*
        let shrinkage_intensity = if gamma_hat > 1e-12 {
            let kappa = (pi_hat) / gamma_hat;
            (kappa / t_f).clamp(0.0, 1.0)
        } else {
            0.0
        };

        // 8. Matriz final regularizada: Sigma = (1 - alpha)*S + alpha*F
        let mut sigma_lw = vec![vec![0.0; n_assets]; n_assets];
        for i in 0..n_assets {
            for j in 0..n_assets {
                sigma_lw[i][j] = (1.0 - shrinkage_intensity) * s[i][j] + shrinkage_intensity * f_target[i][j];
            }
            // Eigen-regularización diagonal de seguridad
            sigma_lw[i][i] += 1e-7;
        }

        sigma_lw
    }
}
