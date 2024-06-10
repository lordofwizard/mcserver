mod config;
mod utils;
use crate::config::*;
use crate::utils::download_jdk;

fn main() {
    let config: Config = Config::new();
    download_jdk(8);
    println!("{:?}",config);
}
