use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct App {
    /// Runs the command as sudo on the remote nodes
    #[arg(short, long)]
    pub sudo: bool,

    /// Filters nodes on a given patterns
    #[arg(short, long, value_name = "regex")]
    pub regex: Option<String>,

    /// This command will be ran on the remote nodes
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, Subcommand)]
pub enum Command {
    /// Generate Bash completion to get bash shell completion to work you can add `eval
    /// "$(docker-manager completion)"` to your ~/.bashrc.
    Completion { shell: clap_complete::Shell },

    /// Execute a command on a given container unless 2 or more containers are found on remote nodes
    Exec {
        /// Container name or id
        container_id: String,

        /// Command that should be ran on the given container
        command: Vec<String>,

        /// Detached mode: run command in the background
        #[arg(short, long)]
        detach: bool,

        /// Override the key sequence for detaching a container
        #[arg(long, value_name = "string")]
        detach_keys: Option<String>,

        /// Set environment variables
        #[arg(short, long, value_name = "list")]
        env: Option<Vec<String>>,

        /// Read in a file of environment variables
        #[arg(long, value_name = "list")]
        env_file: Option<Vec<String>>,

        /// Keep STDIN open even if not attached
        #[arg(short, long)]
        interactive: bool,

        /// Give extended privileges to the command
        #[arg(long)]
        privileged: bool,

        // This one is not useful for us
        ///// Allocate a pseudo-TTY
        //#[arg(short, long)]
        //tty: bool,
        /// Username or UID (format: <name|uid>[:<group|gid>])
        #[arg(short, long, value_name = "string")]
        user: Option<String>,

        /// Working directory inside the container
        #[arg(short, long, value_name = "string")]
        workdir: Option<String>,
    },

    /// List all images on remote nodes
    Images {
        /// Show all images (default hides intermediate images)
        #[arg(short, long)]
        all: bool,

        /// Show digests
        #[arg(long)]
        digest: bool,

        /// Filter output based on conditions provided
        #[arg(short, long, value_name = "filter")]
        filter: Option<String>,

        /// Pretty-print images using a Go template
        #[arg(long, value_name = "string")]
        format: Option<String>,

        /// Don't truncate output
        #[arg(long)]
        no_trunc: bool,

        /// Only show image IDs
        #[arg(short, long)]
        quiet: bool,
    },

    /// Gets the logs of a given container unless 2 or more containers are found on remote nodes
    Logs {
        /// Container name or id
        container_id: String,

        /// Show extra details provided to logs
        #[arg(long)]
        details: bool,

        /// Follow the log output
        #[arg(short, long)]
        follow: bool,

        /// Show logs since timestamp (e.g. 2013-01-02T13:23:37Z) or relative (e.g. 42m for 42 minutes)
        #[arg(long, value_name = "string")]
        since: Option<String>,

        /// Number of lines to show from the end of the logs (default "all")
        #[arg(long, value_name = "string")]
        tail: Option<String>,

        /// Show timestamps
        #[arg(short, long)]
        timestamps: bool,

        /// Show logs before a timestamp (e.g. 2013-01-02T13:23:37Z) or relative (e.g. 42m for 42 minutes)
        #[arg(long, value_name = "string")]
        until: Option<String>,
    },

    /// Lists all containers on remote nodes
    Ps {
        /// Show all containers (default shows just running)
        #[arg(short, long)]
        all: bool,

        /// Filter output based on conditions provided
        #[arg(short, long, value_name = "filter")]
        filter: Option<String>,

        /// Pretty-print containers using a Go template
        #[arg(long, value_name = "string")]
        format: Option<String>,

        /// Show n last created containers (includes all states) (default -1)
        #[arg(short = 'n', long)]
        last: bool,

        /// Show the latest created container (includes all states)
        #[arg(short, long)]
        latests: bool,

        /// Don't truncate output
        #[arg(long)]
        no_trunc: bool,

        /// Only display container IDs
        #[arg(short, long)]
        quiet: bool,

        /// Display total file sizes
        #[arg(short, long)]
        size: bool,
    },

    /// Restart one containe unless 2 or more containers are found on remote nodes
    Restart {
        /// Seconds to wait for stop before killing the container (default 10)
        #[arg(short, long)]
        time: Option<String>,

        /// Container name or id
        container_id: String,
    },

    /// Stops a given container unless 2 or more containers are found on remote nodes
    Stop {
        /// Container name or id
        container_id: String,
    },

    /// Manage Docker
    System(System),
}

#[derive(Args, Clone, Debug)]
pub struct System {
    #[command(subcommand)]
    pub command: SystemCommand,
}

#[derive(Clone, Debug, Subcommand)]
pub enum SystemCommand {
    /// Show docker disk usage
    Df {
        /// Format the output using the given Go template
        #[arg(long, value_name = "string")]
        format: Option<String>,

        /// Show detailed information on space usage
        #[arg(short, long)]
        verbose: bool,
    },
    /// Get real time events from the server
    Events {
        /// Filter output based on conditions provided
        #[arg(short, long, value_name = "filter")]
        filter: Option<String>,

        /// Pretty-print images using a Go template
        #[arg(long, value_name = "string")]
        format: Option<String>,

        /// Show logs since timestamp (e.g. 2013-01-02T13:23:37Z) or relative (e.g. 42m for 42 minutes)
        #[arg(long, value_name = "string")]
        since: Option<String>,

        /// Show logs before a timestamp (e.g. 2013-01-02T13:23:37Z) or relative (e.g. 42m for 42 minutes)
        #[arg(long, value_name = "string")]
        until: Option<String>,
    },
    /// Display system-wide information
    Info {
        /// Pretty-print images using a Go template
        #[arg(long, value_name = "string")]
        format: Option<String>,
    },
    /// Remove unused data
    Prune {
        /// Remove all unused images not just dangling ones
        #[arg(short, long)]
        all: bool,

        /// Filter output based on conditions provided
        #[arg(long, value_name = "filter")]
        filter: Option<String>,

        /// Do not prompt for confirmation
        #[arg(short, long)]
        force: bool,

        /// Prune volumes
        #[arg(long)]
        volumes: bool,
    },
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::Completion { .. } => write!(f, "Completion",),
            Self::Ps { .. } => write!(f, "Ps",),
            Self::Images { .. } => write!(f, "Images",),
            _ => write!(f, "Not implemented",),
        }
    }
}
