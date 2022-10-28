use pe::run_problem;
use std::str::FromStr;

use tracing_subscriber::{filter::targets::Targets, layer::SubscriberExt, util::SubscriberInitExt};

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

    tracing::info!("Running Problem");
    run_problem(20)?;
    tracing::info!("Finished running Problem");

    Ok(())
}
