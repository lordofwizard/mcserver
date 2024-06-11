mod args;
mod config;
mod utils;
mod run;
use crate::config::*;
use crate::utils::*;
use crate::run::Run;

fn main() {
    let config: Config = Config::new();
    let mut run = Run::new();
    //    download_jdk(22);
    //    make_file_tree("homiecraft");
    println!("{:?}", config);
}
