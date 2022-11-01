#![feature(array_windows)]
use std::time::{Duration, Instant};

use euler_trait::Problems;

pub mod common;
mod first_twenty;
pub mod utils;

pub fn run_problem(problem: Option<usize>) -> color_eyre::Result<()> {
    if let Some(problem) = problem {
        let start = Instant::now();
        let answer = Problems::get(problem).solve();
        let elapsed = start.elapsed();
        tracing::info!("Answer for problem {problem}: {answer}");
        tracing::info!("Took: {elapsed:?}");
    } else {
        let mut total = Duration::default();
        for p in Problems::collect() {
            let start = Instant::now();
            let answer = p.solve();
            let elapsed = start.elapsed();
            total += elapsed;
            let problem = p.problem();
            tracing::info!("Answer for problem {problem}: {answer}");
            tracing::info!("#{problem} Took: {elapsed:?}");
        }
        tracing::info!("In total Took: {total:?}");
    }

    Ok(())
}
