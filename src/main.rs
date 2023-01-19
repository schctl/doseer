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

fn run() -> anyhow::Result<()> {
    let config = config::read_config().context("failed to get configuration")?;
    gui::Gui::run(Settings::with_flags(config))?;
    Ok(())
}

fn main() {
    tracing_subscriber::fmt::init();

    if let Err(e) = run() {
        tracing::error!("{}", e);
        std::process::exit(-1);
    }
}
