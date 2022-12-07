//! A sensible GUI file manager.

mod config;
mod dirs;

fn run() -> anyhow::Result<()> {
    config::read_config()?;

    Ok(())
}

fn main() {
    tracing_subscriber::fmt::init();

    if let Err(e) = run() {
        tracing::error!("{}", e);
        std::process::exit(-1);
    }
}
