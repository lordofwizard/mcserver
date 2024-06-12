mod args;
mod config;
mod run;
mod utils;
use crate::config::*;
use crate::run::Run;
use crate::utils::*;

fn main() {
    let config: Config = Config::new();
    let mut run = Run::new();
    //    download_jdk(22);
    //    make_file_tree("homiecraft");
    println!("{:?}", config);
}
