mod config;
use crate::config::*;


fn main() {
    let config: Config = Config::new();

    println!("{:?}",config);
}
