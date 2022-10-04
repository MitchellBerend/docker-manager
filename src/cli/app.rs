use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct App {
    /// This command will be ran on the remote nodes
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Lists all containers on remote nodes
    Ps {
        /// include inactive containers
        #[arg(short, long)]
        all: bool,
    },

    /// Stops a given container unless 2 or more containers are found on remote nodes
    Stop {
        /// Container name or id
        container_id: String,
    },
}
