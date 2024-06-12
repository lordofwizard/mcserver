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
    ServerGenerate,
    StartServer,
    StopServer,
}
