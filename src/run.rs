use crate::args::Commands;
use crate::args::MCServer;
use crate::generate::build::server_generate;
use crate::start::start_server;
use clap::Parser;

pub struct Run {}

impl Run {
    pub fn new() {
        let server_instance = MCServer::parse();

        match server_instance.command {
            Commands::Build => server_generate(),
            Commands::Start => start_server(),

            _ => println!("Nah didn't reach me bruh"),
        }
    }
}
