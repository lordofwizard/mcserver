use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(
    name = "MCServer",
    about = "MCServer a cli utility created to host a Fully Customizable Minecraft Server",
    author = "LordOfWizard",
    version = "0.0.1"
)]
pub struct MCServer {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Generates a new Minecraft Server instance.
    ServerGenerate,
    /// Starts the Minecraft Server.
    StartServer,
    /// Stops the Minecraft Server.
    StopServer,
    /// Prints the logs of the latest run.
    Log,
    /// Gives debug information on Currently running server.
    Check,
    /// Prints the connectable url of the Minecraft Server.
    GetUrl,
}
