use crate::args::Commands;
use crate::args::MCServer;
use crate::utils::server_present;
use clap::Parser;

pub struct Run {

}

impl Run {
    pub fn new() {
        let server_instance = MCServer::parse();


    }
}

fn sgen() {
    println!("I am inside sgen");
}
