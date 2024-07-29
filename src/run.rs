use crate::args::Commands;
use crate::args::MCServer;
use crate::build::build;
use crate::setup::setup;
use crate::start::start_server;
use clap::Parser;

pub struct Run {}

impl Run {
    pub fn new() {
        let server_instance = MCServer::parse();

        match server_instance.command {
            Commands::Setup => setup(),
            Commands::Build => build(),
            Commands::Start => start_server(),

            _ => println!("Nah didn't reach me bruh"),
        }
    }
}
