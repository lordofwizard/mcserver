mod args;
mod build;
mod centro;
mod check;
mod config;
mod config_builder;
mod get_url;
mod java;
mod log;
mod run;
mod setup;
mod start;
mod stop;

use run::Run;
fn main() {
    println!("Hello, world!");
    Run::new();
}
