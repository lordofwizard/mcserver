mod args;
mod config;
mod utils;
mod run;
use crate::config::*;
use crate::utils::*;
use crate::run::Run;
use crate::args::MCServer;
use crate::args::Commands;
use clap::Parser;

fn main() {
    let config: Config = Config::new();
    let run = Run::new();
    //    download_jdk(22);
    //    make_file_tree("homiecraft");
    MCServer::parse();
    println!("{:?}", config);
}
