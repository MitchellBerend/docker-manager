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

    /// Identity file that will be used to identify this machine over ssh
    #[arg(short, long, value_name = "identity-file")]
    pub identity_file: Option<String>,

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

    /// Restart one or more containers on remote nodes
    Restart {
        /// Seconds to wait for stop before killing the container (default 10)
        #[arg(short, long)]
        time: Option<String>,

        /// Container name or id
        container_id: Vec<String>,
    },

    /// Remove one or more containers
    Rm {
        /// Container name or id
        container_id: Vec<String>,

        /// Force the removal of a running container (uses SIGKILL)
        #[arg(short, long)]
        force: bool,

        /// Remove anonymous volumes associated with the container
        #[arg(short, long)]
        volumes: bool,
    },

    /// Starts a given container unless 2 or more containers are found on remote nodes
    Start {
        /// Attach STDOUT/STDERR and forward signals
        #[arg(short, long)]
        attach: bool,

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

impl Command {
    pub fn internal_reference_command(&self) -> InternalCommand {
        match self {
            Self::Exec {
                container_id,
                command,
                detach,
                detach_keys,
                env,
                env_file,
                interactive,
                privileged,
                user,
                workdir,
            } => {
                let mut commands: Vec<&str> = vec![];
                for com in command {
                    commands.push(com);
                }

                let mut _env: Option<Vec<&str>> = match env {
                    Some(environ) => {
                        let mut int: Vec<&str> = vec![];
                        for e in environ {
                            int.push(e);
                        }

                        Some(int)
                    }
                    None => None,
                };

                let mut _env_file: Option<Vec<&str>> = match env_file {
                    Some(environ) => {
                        let mut int: Vec<&str> = vec![];
                        for e in environ {
                            int.push(e);
                        }

                        Some(int)
                    }
                    None => None,
                };

                InternalCommand::Exec {
                    container_id: &container_id,
                    command: commands,
                    detach: *detach,
                    detach_keys: detach_keys.as_deref(),
                    env: _env,
                    env_file: _env_file,
                    interactive: *interactive,
                    privileged: *privileged,
                    user: user.as_deref(),
                    workdir: workdir.as_deref(),
                }
            }
            Self::Images {
                all,
                digest,
                filter,
                format,
                no_trunc,
                quiet,
            } => {
                let _filter: Option<&str> = match filter {
                    Some(f) => Some(f),
                    None => None,
                };

                let _format: Option<&str> = match format {
                    Some(f) => Some(f),
                    None => None,
                };

                InternalCommand::Images {
                    all: *all,
                    digest: *digest,
                    filter: _filter,
                    format: _format,
                    no_trunc: *no_trunc,
                    quiet: *quiet,
                }
            }
            Self::Logs {
                container_id,
                details,
                follow,
                since,
                tail,
                timestamps,
                until,
            } => {
                let _since: Option<&str> = match since {
                    Some(s) => Some(s),
                    None => None,
                };

                let _tail: Option<&str> = match tail {
                    Some(t) => Some(t),
                    None => None,
                };

                let _until: Option<&str> = match until {
                    Some(t) => Some(t),
                    None => None,
                };

                InternalCommand::Logs {
                    container_id: &container_id,
                    details: *details,
                    follow: *follow,
                    since: _since,
                    tail: _tail,
                    timestamps: *timestamps,
                    until: _until,
                }
            }
            Self::Ps {
                all,
                filter,
                format,
                last,
                latests,
                no_trunc,
                quiet,
                size,
            } => {
                let _filter: Option<&str> = match filter {
                    Some(t) => Some(t),
                    None => None,
                };

                let _format: Option<&str> = match format {
                    Some(t) => Some(t),
                    None => None,
                };

                InternalCommand::Ps {
                    all: *all,
                    filter: _filter,
                    format: _format,
                    last: *last,
                    latests: *latests,
                    no_trunc: *no_trunc,
                    quiet: *quiet,
                    size: *size,
                }
            }
            Self::Restart { time, container_id } => {
                let _time: Option<&str> = match time {
                    Some(t) => Some(t),
                    None => None,
                };

                let mut _container_id: Vec<&str> = vec![];

                for cont in container_id {
                    _container_id.push(cont)
                }

                InternalCommand::Restart {
                    time: _time,
                    container_id: _container_id,
                }
            }
            Self::Rm {
                container_id,
                force,
                volumes,
            } => {
                let mut _container_id: Vec<&str> = vec![];

                for cont in container_id {
                    _container_id.push(cont)
                }
                InternalCommand::Rm {
                    container_id: _container_id,
                    force: *force,
                    volumes: *volumes,
                }
            }
            Self::Start {
                attach,
                container_id,
            } => {
                // let mut _container_id: Vec<&str> = vec![];
                //
                // for cont in container_id {
                //     _container_id.push(cont)
                // }

                InternalCommand::Start {
                    attach: *attach,
                    container_id,
                }
            }
            Self::Stop { container_id } => {
                //
                // let mut _container_id: Vec<&str> = vec![];
                //
                // for cont in container_id {
                //     _container_id.push(cont)
                // }
                InternalCommand::Stop { container_id }
            }
            Self::System(s) => InternalCommand::System(s.clone()),
            _ => unreachable!(),
        }
    }
}

pub enum InternalCommand<'a> {
    /// Execute a command on a given container unless 2 or more containers are found on remote nodes
    Exec {
        /// Container name or id
        container_id: &'a str,

        /// Command that should be ran on the given container
        command: Vec<&'a str>,

        /// Detached mode: run command in the background
        detach: bool,

        /// Override the key sequence for detaching a container
        detach_keys: Option<&'a str>,

        /// Set environment variables
        env: Option<Vec<&'a str>>,

        /// Read in a file of environment variables
        env_file: Option<Vec<&'a str>>,

        /// Keep STDIN open even if not attached
        interactive: bool,

        /// Give extended privileges to the command
        privileged: bool,

        /// Username or UID (format: <name|uid>[:<group|gid>])
        user: Option<&'a str>,

        /// Working directory inside the container
        workdir: Option<&'a str>,
    },

    /// List all images on remote nodes
    Images {
        /// Show all images (default hides intermediate images)
        all: bool,

        /// Show digests
        digest: bool,

        /// Filter output based on conditions provided
        filter: Option<&'a str>,

        /// Pretty-print images using a Go template
        format: Option<&'a str>,

        /// Don't truncate output
        no_trunc: bool,

        /// Only show image IDs
        quiet: bool,
    },

    /// Gets the logs of a given container unless 2 or more containers are found on remote nodes
    Logs {
        /// Container name or id
        container_id: &'a str,

        /// Show extra details provided to logs
        details: bool,

        /// Follow the log output
        follow: bool,

        /// Show logs since timestamp (e.g. 2013-01-02T13:23:37Z) or relative (e.g. 42m for 42 minutes)
        since: Option<&'a str>,

        /// Number of lines to show from the end of the logs (default "all")
        tail: Option<&'a str>,

        /// Show timestamps
        timestamps: bool,

        /// Show logs before a timestamp (e.g. 2013-01-02T13:23:37Z) or relative (e.g. 42m for 42 minutes)
        until: Option<&'a str>,
    },

    /// Lists all containers on remote nodes
    Ps {
        /// Show all containers (default shows just running)
        all: bool,

        /// Filter output based on conditions provided
        filter: Option<&'a str>,

        /// Pretty-print containers using a Go template
        format: Option<&'a str>,

        /// Show n last created containers (includes all states) (default -1)
        last: bool,

        /// Show the latest created container (includes all states)
        latests: bool,

        /// Don't truncate output
        no_trunc: bool,

        /// Only display container IDs
        quiet: bool,

        /// Display total file sizes
        size: bool,
    },

    /// Restart one or more containers on remote nodes
    Restart {
        /// Seconds to wait for stop before killing the container (default 10)
        time: Option<&'a str>,

        /// Container name or id
        container_id: Vec<&'a str>,
    },

    /// Remove one or more containers
    Rm {
        /// Container name or id
        container_id: Vec<&'a str>,

        /// Force the removal of a running container (uses SIGKILL)
        force: bool,

        /// Remove anonymous volumes associated with the container
        volumes: bool,
    },

    /// Starts a given container unless 2 or more containers are found on remote nodes
    Start {
        /// Attach STDOUT/STDERR and forward signals
        attach: bool,

        /// Container name or id
        container_id: &'a str,
    },

    /// Stops a given container unless 2 or more containers are found on remote nodes
    Stop {
        /// Container name or id
        container_id: &'a str,
    },

    /// Manage Docker
    System(System),
}
