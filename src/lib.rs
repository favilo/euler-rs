use color_eyre::eyre::eyre;

mod common;
mod first_twenty;

pub fn run_problem(problem: u32) -> color_eyre::Result<()> {
    let answer = match problem {
        problem if problem <= 20 => first_twenty::run(problem),
        _problem => Err(eyre!("Not defined yet")),
    }?;

    tracing::info!("Answer for problem {problem}: {answer}");

    Ok(())
}
