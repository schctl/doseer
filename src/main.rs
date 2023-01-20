//! A sensible GUI file manager.

use anyhow::Context;
use iced::{Application, Settings};
use m7_core::config;

mod gui;

mod tab;
pub use tab::Tab;

mod pane;
pub use pane::Pane;

mod theme;
pub use theme::Theme;

mod icons;
pub use icons::Icon;

mod item;

mod side_bar;
pub use side_bar::SideBar;

/// Use mimalloc as our global allocator.
///
/// The benefits of using mimalloc here haven't been tested scientifically, but it seems to be
/// *generally* more performant, especially when viewing large directories and helps the UI *feel*
/// more responsive overall, so it is left here for now.
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[allow(unused)]
fn setup_tracing() {
    use tracing_subscriber::filter::{EnvFilter, LevelFilter};

    let mut env_filter = EnvFilter::from_default_env()
        // Default -> WARN
        .add_directive("m7=warn".parse().unwrap());

    #[cfg(debug_assertions)]
    {
        // Debug mode -> INFO
        env_filter = env_filter.add_directive("m7=info".parse().unwrap());
    }
    #[cfg(feature = "debug")]
    {
        // Full debug -> DEBUG
        env_filter = env_filter
            .add_directive("m7=debug".parse().unwrap())
            .add_directive("iced=info".parse().unwrap());
    }

    tracing_subscriber::fmt().with_env_filter(env_filter).init();
}

fn run() -> anyhow::Result<()> {
    let config = config::read_config().context("failed to get configuration")?;
    gui::Gui::run(Settings::with_flags(config))?;
    Ok(())
}

fn main() {
    setup_tracing();

    if let Err(e) = run() {
        tracing::error!("{}", e);
        std::process::exit(-1);
    }
}
