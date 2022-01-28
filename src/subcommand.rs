#[derive(clap::Subcommand)]
pub enum DockerCommand {
    /// Shows a list of all containers and what node it is on.
    Ps {
        /// The regex pattern that will be used to match entries from the config.
        #[clap(short, long, default_value = ".*")]
        regex: String,
    },

    /// executes a command on the specified docker container, no flags will
    /// be present for now. This will not support interactive commands. If
    /// you need a shell connected, connect to the machine and use the docker
    ///  cli.
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
        #[clap(index(2))]
        node: String,
        /// The container id or name.
        #[clap(index(1))]
        container: String,
    },

    /// Spins up a new container with the specified inputs on the target node.
    Run {
        /// The node the container is on.
        #[clap(index(1))]
        node: String,
        /// The image that needs to be run
        #[clap(index(2))]
        image: String,

        /// The name of the container
        #[clap(short = 'n', long, required = false, default_value = "")]
        name: String,

        #[clap(short = 'r', long, required = false, default_value = "always")]
        restart: String,

        /// The port map of the container
        #[clap(
            short,
            long,
            multiple_occurrences = true,
            value_delimiter = ',',
            default_value = ""
        )]
        port: String,

        /// Environment variables that need to be passed in.
        #[clap(
            short,
            long,
            multiple_occurrences = true,
            value_delimiter = ',',
            default_value = ""
        )]
        env: Vec<String>,
    },

    /// Stops a specified container on a specified node.
    Stop {
        /// The node the container is on.
        #[clap(index(2))]
        node: String,
        /// The container id or name.
        #[clap(index(1))]
        container: String,
    },

    /// Removes a specified stopped container on a specified node.
    Rm {
        /// The node the container is on.
        #[clap(index(2))]
        node: String,
        /// The container id or name.
        #[clap(index(1))]
        container: String,
    },

    /// Gets low level information of  container on a specified node.
    Inspect {
        /// The node the container is on.
        #[clap(index(2))]
        node: String,
        /// The container id or name.
        #[clap(index(1))]
        container: String,
    },
}
