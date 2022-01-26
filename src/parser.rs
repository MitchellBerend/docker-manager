// This module defines the main cli and all its arguments


use clap::Parser;


#[derive(clap::Subcommand)]
pub enum DockerCommand {
    /// This will show a list of all containers and what node it is on.
    Ps,

    /// This will execute a command on the specified docker container,
    /// no flags will be present for now.
    Exec {
        /// The node the container is on. 
        node: String,
        /// The container id or name.
        container: String,
        /// The command that needs to be executed.
        command: String,
    },

    /// This will fetch logs from specified docker containers.
    Logs {
        /// The node the container is on. 
        node: String,
        /// The container id or name.
        container: String,
    },
}


#[derive(Parser)]
#[clap(
    author = "Mitchell Berendhuysen",
    version,
    about = "This tool mimics the functionality of the docker cli but abstracts its over all connected nodes."
)]
pub struct MainParser {
    /// The docker cli command to be executed.
    #[clap(subcommand)]
    pub command: DockerCommand,

    #[clap(short, long, default_value=".*")]
    pub regex: String,
}
