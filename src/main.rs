//! A sensible GUI file manager.

mod config;
mod dirs;

fn main() {
    config::read_config().unwrap();
}
