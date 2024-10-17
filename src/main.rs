mod args;
mod build;
mod check;
mod config;
mod get_url;
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
