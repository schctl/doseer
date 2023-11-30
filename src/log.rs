//! Application wide logging configurations.

use tracing::level_filters::LevelFilter;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

const fn lowest_log_level() -> LevelFilter {
    if cfg!(feature = "debug") {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    }
}

/// Initialize global tracing subscriber. Call only once.
pub fn init_tracing() {
    let targets = Targets::default()
        .with_default(lowest_log_level())
        .with_target("iced", LevelFilter::WARN)
        .with_target("wgpu", LevelFilter::OFF)
        .with_target("naga", LevelFilter::OFF)
        .with_target("cosmic_text", LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(targets)
        .init();
}
