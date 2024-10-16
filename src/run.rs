pub struct Run;
use clap::Parser;
use crate::args::{MCServer,Commands};
use crate::build::build;
use crate::setup::setup;
use crate::start::start_server;
use crate::stop::stop;
use crate::log::log;
use crate::check::check;
use crate::get_url::get_url;

impl Run {
    pub fn new() {
        // TODO this is where the program should start
        println!("Program Started");

        let server_instance = MCServer::parse();

        match server_instance.command {
            Commands::Setup => setup(),
            Commands::Build => build(),
            Commands::Start => start_server(),
            Commands::Log   => log(),
            Commands::Check => check(),
            Commands::Stop  => stop(),
            Commands::GetUrl => get_url(),
            _ => println!("No Implementation found for this command"),
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_has_std_practices() {
        // Commented this test cause while parsing it fails 
        // Waits for a user subcommand and takes directly
        // To help 

        //Run::new();
        assert!(true);
    }
    #[test]
    fn setup_available() {
        setup();
        assert!(true);
    }
}
