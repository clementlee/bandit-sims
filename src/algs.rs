use anyhow::{Context, Result};
use std::collections::HashMap;

use crate::util_types::{create_running_means, RunningMeans};

pub trait Algorithm {
    fn choose_arm(&self, round: usize) -> Result<usize>;

    fn update_with_result(&mut self, round: usize, choice: usize, reward: f64) -> Result<()>;
}

pub struct ETC {
    k: usize,

    num_arms: usize,

    /// once we've made a choice, we'll just stick with that forever
    chosen: Option<usize>,

    arms: RunningMeans,
}

pub fn create_etc_alg(k: usize, arms: usize) -> ETC {
    ETC {
        k: k,
        num_arms: arms,
        chosen: None,
        arms: create_running_means(arms),
    }
}

impl Algorithm for ETC {
    fn choose_arm(&self, round: usize) -> Result<usize> {
        match self.chosen {
            Some(x) => Ok(x),
            None => {
                if round == 0 {
                    Ok(0)
                } else if round < self.k * self.num_arms {
                    Ok(round / self.k)
                } else {
                    panic!("Should never happen!")
                }
            }
        }
    }

    fn update_with_result(&mut self, round: usize, choice: usize, reward: f64) -> Result<()> {
        if round < self.k * self.num_arms {
            self.arms.update_arm(choice, reward)?;
        }
        if round + 1 == self.k * self.num_arms {
            self.chosen = Some(self.arms.get_best_arm()?)
        }

        Ok(())
    }
}
