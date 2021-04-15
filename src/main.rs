mod algs;
mod envs;
mod util_types;

use algs::{create_etc_alg, Algorithm, ETC};
use anyhow::Result;
use envs::Environment;

fn main() -> Result<()> {
    let env = envs::create_gaussian_env(&vec![0.0, 0.1, 0.09])?;
    let etc = create_etc_alg(2, env.arms());

    let env = Box::from(env);
    let mut etc = Box::from(etc);

    run_alg_on_env(env, &mut etc, 20)?;
    Ok(())
}

fn run_alg_on_env<E: Environment, T: Algorithm>(
    env: Box<E>,
    alg: &mut Box<T>,
    rounds: usize,
) -> Result<f64> {
    let mut total_reward = 0.0;
    for round in 0..rounds {
        let choice = alg.choose_arm(round)?;

        let reward = env.get_reward(choice)?;

        alg.update_with_result(round, choice, reward)?;

        total_reward += reward;

        println!("Round {}: chose {} for reward {}", round, choice, reward);
    }

    Ok(total_reward)
}
