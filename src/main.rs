mod args;
mod build;
mod config;
mod java;
mod run;
mod setup;
mod start;
mod utils;

use crate::run::Run;

fn main() {
    let _ = Run::new();
}
