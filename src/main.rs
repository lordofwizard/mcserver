mod args;
mod build;
mod check;
mod get_url;
mod log;
mod run;
mod setup;
mod start;
mod stop;
mod config;


use run::Run;
fn main() {
    println!("Hello, world!");
    Run::new();
}
