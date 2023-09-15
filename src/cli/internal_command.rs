use super::System;

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
        container_id: Vec<&'a str>,
    },

    /// Stops a given container unless 2 or more containers are found on remote nodes
    Stop {
        /// Container name or id
        container_id: Vec<&'a str>,
    },

    /// Manage Docker
    System(System),
}
