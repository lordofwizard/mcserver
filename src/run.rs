use crate::args::Commands;
use crate::args::MCServer;
use crate::generate::server_generate;
use crate::start::start_server;
use clap::Parser;

pub struct Run {}

impl Run {
    pub fn new() {
        let server_instance = MCServer::parse();

        match server_instance.command {
            Commands::ServerGenerate => server_generate(),
            Commands::StartServer => start_server(),
            //            Commands::Log => log(),
            //            Commands::Check => check(),
            //            Commands::GetUrl => get_url()
            _ => println!("Nah didn't reach me bruh"),
        }
    }
}
