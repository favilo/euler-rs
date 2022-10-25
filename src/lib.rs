#![feature(array_windows)]
use std::time::Instant;

use color_eyre::eyre::eyre;

mod common;
mod first_twenty;

pub fn run_problem(problem: u32) -> color_eyre::Result<()> {
    let start = Instant::now();
    let answer = match problem {
        problem if problem <= 20 => first_twenty::run(problem),
        _problem => Err(eyre!("Not defined yet")),
    }?;
    let elapsed = start.elapsed();

    tracing::info!("Answer for problem {problem}: {answer}");
    tracing::info!("Took: {elapsed:?}");

    Ok(())
}
