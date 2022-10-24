use pe::run_problem;
use std::str::FromStr;

use tracing_subscriber::{filter::targets::Targets, layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let filter_layer = Targets::from_str(std::env::var("RUST_LOG").as_deref().unwrap_or("info"))?;
    let format_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(format_layer)
        .init();

    tracing::info!("Running Problem");
    run_problem(3)?;
    tracing::info!("Finished running Problem");

    Ok(())
}
