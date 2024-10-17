mod args;
mod build;
mod check;
mod config;
mod get_url;
mod log;
mod java;
mod run;
mod setup;
mod start;
mod stop;
mod config_builder;

use run::Run;
fn main() {
    println!("Hello, world!");
    Run::new();
}
