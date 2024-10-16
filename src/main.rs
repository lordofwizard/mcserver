mod run;
mod args;
mod start;
mod build;
mod setup;
use run::Run;
fn main() {
    println!("Hello, world!");
    Run::new();
}

