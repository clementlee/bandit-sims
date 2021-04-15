use anyhow::{Context, Result};
use rand_distr::{Distribution, Normal};
pub trait Environment {
    fn arms(&self) -> usize;

    fn get_reward(&self, arm: usize) -> Result<f64>;

    fn get_regret(&self, reward: f64) -> f64;
}

pub struct GaussianEnv {
    dists: Vec<Normal<f64>>,

    optimal: f64,
}

pub fn create_gaussian_env(means: &Vec<f64>) -> Result<GaussianEnv> {
    let dists = means
        .iter()
        .map(|mean| Ok(Normal::from_mean_cv(*mean, 1.0)?))
        .collect::<Result<Vec<Normal<f64>>>>()?;

    Ok(GaussianEnv {
        dists: dists,
        optimal: *means
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .context("what")?,
    })
}

impl Environment for GaussianEnv {
    fn arms(&self) -> usize {
        self.dists.len()
    }

    fn get_reward(&self, arm: usize) -> Result<f64> {
        let norm = self.dists.get(arm).context("Out of bounds?")?;

        let sample = norm.sample(&mut rand::thread_rng());

        Ok(sample)
    }

    fn get_regret(&self, reward: f64) -> f64 {
        self.optimal - reward
    }
}
