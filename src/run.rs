pub struct Run;
use clap::Parser;
use crate::args::{MCServer,Commands};
use crate::build::build;
use crate::setup::setup;
use crate::start::start_server;


impl Run {
    pub fn new() {
        // TODO this is where the program should start
        println!("Program Started");

        let server_instance = MCServer::parse();

        match server_instance.command {
            Commands::Setup => setup(),
            Commands::Build => build(),
            Commands::Start => start_server(),

            _ => println!("Nah didn't reach me bruh"),
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_has_std_practices() {
        //Run::new();
        assert!(true);
    }
    #[test]
    fn setup_available() {
        setup();
        assert!(true);
    }
}
