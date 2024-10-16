mod run;
mod args;
mod start;
mod build;
mod setup;
mod log;
mod check;
mod stop;
mod get_url;
use run::Run;
fn main() {
    println!("Hello, world!");
    Run::new();
}

