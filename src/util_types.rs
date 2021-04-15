use anyhow::{bail, Context, Result};

pub struct RunningMeans {
    arms: Vec<RunningMeanEntry>,
}

struct RunningMeanEntry {
    total: f64,
    num: usize,
}

pub fn create_running_means(arms: usize) -> RunningMeans {
    RunningMeans {
        arms: (0..arms)
            .map(|_| RunningMeanEntry { total: 0.0, num: 0 })
            .collect(),
    }
}

impl RunningMeans {
    pub fn get_mean(&self, arm: usize) -> Result<f64> {
        let entry = self.arms.get(arm).context("Couldn't get arm")?;
        match entry.num {
            0 => bail!("No entries yet"),
            _ => Ok(entry.total / (entry.num as f64)),
        }
    }

    pub fn get_mean_default(&self, arm: usize, default: f64) -> Result<f64> {
        match self.get_mean(arm) {
            Ok(x) => Ok(x),
            Err(_) => Ok(default),
        }
    }

    pub fn get_best_arm(&self) -> Result<usize> {
        let best = (0..self.arms.len()).max_by(|x, y| {
            let mean_x = self.get_mean(*x).expect("asdf");
            let mean_y = self.get_mean(*y).expect("asdf");

            mean_x
                .partial_cmp(&mean_y)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(best.context("asdf")?)
    }

    pub fn update_arm(&mut self, arm: usize, value: f64) -> Result<()> {
        let mut entry = self.arms.get_mut(arm).context("Couldn't get arm")?;

        entry.num += 1;
        entry.total += value;

        Ok(())
    }
}
