//! A sensible GUI file manager.

use anyhow::Context;
use iced::{Application, Settings};

mod gui;
mod config;
mod dirs;

fn run() -> anyhow::Result<()> {
    let config = config::read_config().context("failed to get configuration")?;

    // Run the GUI
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
