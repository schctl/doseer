//! A sensible GUI file manager.

use anyhow::Context;
use iced::{Application, Settings};

mod config;
mod gui;
mod icons;
mod item;
mod log;
mod pane;
mod side_bar;
mod tab;
mod theme;

use config::Config;
use icons::Icon;
use tab::Tab;
use theme::Theme;

/// Use mimalloc as our global allocator.
///
/// The benefits of using mimalloc here haven't been tested scientifically, but it seems to be
/// *generally* more performant, especially when viewing large directories and helps the UI *feel*
/// more responsive overall, so it is left here for now.
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn run() -> anyhow::Result<()> {
    let config = Config::load().context("failed to get configuration")?;
    gui::Gui::run(Settings::with_flags(config))?;
    Ok(())
}

fn main() {
    log::init_tracing();

    if let Err(e) = run() {
        tracing::error!("{:?}", e);
        std::process::exit(-1);
    }
}
