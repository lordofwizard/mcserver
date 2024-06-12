use crate::args::Commands;
use crate::args::MCServer;
use crate::utils::server_present;
use clap::Parser;

pub struct Run {
    server_instance: MCServer,
}

impl Run {
    pub fn new() {
        if server_present() {
            println!("Starting new server");

            let server = MCServer::parse();

            println!("{:?}",server);
            match &server.command {

                Commands::ServerGenerate => sgen(),

                Commands::StartServer | &Commands::StopServer => todo!(),
                _ => println!("NO OPTION PROVIDED")
            };
            println!("Hellow");
        } else {
            println!("Server not found");
        }
    }
}

fn sgen(){
    println!("I am inside sgen");
}
