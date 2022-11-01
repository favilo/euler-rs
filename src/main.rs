use clap::{arg, Parser};
use euler_trait::Problems;
use pe::run_problem;
use std::str::FromStr;

use tracing_subscriber::{filter::targets::Targets, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    problem: Option<usize>,

    #[arg(short, long, default_value_t = false)]
    last: bool,
}

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let filter_layer = Targets::from_str(std::env::var("RUST_LOG").as_deref().unwrap_or("info"))?;
    let format_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(format_layer)
        .init();

    let cli = Cli::parse();
    let problem = if cli.last {
        Some(Problems::collect().last().unwrap().problem())
    } else {
        cli.problem
    };

    tracing::info!("Running Problem");
    run_problem(problem)?;
    tracing::info!("Finished running Problem");

    Ok(())
}
