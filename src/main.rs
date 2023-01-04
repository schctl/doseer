//! A sensible GUI file manager.

use anyhow::Context;
use iced::{Application, Settings};

mod config;
mod dirs;
mod gui;
mod path;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

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
