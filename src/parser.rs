// This module defines the main cli and all its arguments

use std::error::Error;
use std::str::from_utf8;

use clap::Parser;
use futures::{stream, StreamExt};

const CONCURRENT_REQUESTS: usize = 10;

#[derive(clap::Subcommand)]
pub enum DockerCommand {
    /// This will show a list of all containers and what node it is on.
    Ps {
        /// The regex pattern that will be used to match entries from the config.
        #[clap(short, long, default_value = ".*")]
        regex: String,
    },

    /// This will execute a command on the specified docker container,
    /// no flags will be present for now. This will not support interactive
    /// commands. If you need a shell connected, connect to the machine and
    /// use the docker cli.
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
        #[clap(index(1))]
        node: String,
        /// The container id or name.
        #[clap(index(2))]
        container: String,
    },

    /// This will spin up a new container with the specified inputs on the
    /// target node.
    Run {
        /// The node the container is on.
        #[clap(index(2))]
        node: String,
        /// The image that needs to be run
        #[clap(index(1))]
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

    /// This will stop a specified container on a specified node.
    Stop {
        /// The node the container is on.
        #[clap(index(1))]
        node: String,
        /// The container id or name.
        #[clap(index(2))]
        container: String,
    },

    /// This will remove a specified stopped container on a specified node.
    Rm {
        /// The node the container is on.
        #[clap(index(1))]
        node: String,
        /// The container id or name.
        #[clap(index(2))]
        container: String,
    },
}

#[derive(Parser)]
#[clap(
    author = "Mitchell Berendhuysen",
    version,
    about = "This tool mimics the functionality of the docker cli but abstracts its over all connected nodes defined in the current users ~/.ssh/config."
)]
pub struct MainParser {
    /// The docker cli command to be executed.
    #[clap(subcommand)]
    pub command: DockerCommand,
}

impl MainParser {
    pub async fn send_ps_command(&self, nodes: &[String]) -> Result<(), Box<dyn Error>> {
        let _bodies = stream::iter(nodes)
            .map(|node| async move {
                let mut return_str = String::new();
                let owned_node = node.clone();
                let session = openssh::SessionBuilder::default()
                    .connect_timeout(std::time::Duration::new(1, 0))
                    .connect(&owned_node)
                    .await;
                return_str.push_str(&format!("host {:?}\n", &owned_node));
                match session {
                    Ok(session) => {
                        let output = session
                            .command("sudo")
                            .arg("docker")
                            .arg("ps")
                            .arg("-a")
                            .output()
                            .await
                            .unwrap();
                        return_str.push_str(&String::from(from_utf8(&output.stdout).unwrap()));
                    }
                    Err(_) => {
                        return_str.push_str(&format!("Could not connect to {}", &owned_node));
                    }
                }
                return_str
            })
            .buffer_unordered(CONCURRENT_REQUESTS);
        _bodies
            .for_each(|body| async move {
                println!("{}", body);
            })
            .await;

        Ok(())
    }

    pub async fn send_log_command(&self) -> Result<(), Box<dyn Error>> {
        let mut _node: String = String::new();
        let mut _container: String = String::new();
        match &self.command {
            DockerCommand::Logs { node, container } => {
                _node = node.clone();
                _container = container.clone();
            }
            _ => panic!("error in send_log_command"),
        };
        let session = openssh::SessionBuilder::default()
            .connect_timeout(std::time::Duration::from_secs(1))
            .connect(&_node)
            .await;
        println!("host {:?}", &_node);

        match session {
            Ok(session) => {
                let output = session
                    .command("sudo")
                    .arg("docker")
                    .arg("logs")
                    .arg(_container)
                    .output()
                    .await?;
                println!(
                    "stdout: {}\n\n\n\nstderr: {}",
                    String::from(from_utf8(&output.stdout)?),
                    String::from(from_utf8(&output.stderr)?)
                );
            }
            Err(_) => {
                println!("Could not connect to {}", &_node);
            }
        }
        Ok(())
    }

    pub async fn send_exec_command(&self) -> Result<(), Box<dyn Error>> {
        let mut _node: String = String::new();
        let mut _container: String = String::new();
        let mut _command: String = String::new();
        match &self.command {
            DockerCommand::Exec {
                node,
                container,
                command,
            } => {
                _node = node.clone();
                _container = container.clone();
                _command = command.clone();
            }
            _ => panic!("error in send_log_command"),
        };
        let session = openssh::SessionBuilder::default()
            .connect_timeout(std::time::Duration::new(1, 0))
            .connect(&_node)
            .await;
        println!("host {:?}", &_node);
        match session {
            Ok(session) => {
                let output = session
                    .command("sudo")
                    .arg("docker")
                    .arg("exec")
                    .arg(&_container)
                    .arg(&_command)
                    .output()
                    .await?;
                println!(
                    "stdout: {}\n\n\n\nstderr: {}",
                    String::from(from_utf8(&output.stdout)?),
                    String::from(from_utf8(&output.stderr)?)
                );
            }
            Err(_) => {
                println!("Could not connect to {}", &_node);
            }
        }
        Ok(())
    }

    pub async fn send_run_command(&self) -> Result<(), Box<dyn Error>> {
        let mut _node: String = String::new();
        let mut _image: String = String::new();
        let mut _name: String = String::new();
        let mut _port: String = String::new();
        let mut _restart: String = String::new();
        let mut _env: Vec<String> = vec!();
        match &self.command {
            DockerCommand::Run {
                node,
                image,
                name,
                port,
                restart,
                env,
            } => {
                _node = node.clone();
                _image = image.clone();
                _name = name.clone();
                _port = port.clone();
                _restart = restart.clone();
                _env = env.clone();
            }
            _ => panic!("error in send_log_command"),
        };
        let session = openssh::SessionBuilder::default()
            .connect_timeout(std::time::Duration::new(1, 0))
            .connect(&_node)
            .await;
        println!("host {:?}", &_node);
        match session {
            Ok(session) => {
                let mut output = session.command("sudo");
                let _ = &output.arg("docker")
                    .arg("run")
                    .arg("-d");
                    // working vvvv
                    if !&_port.is_empty() {
                        let _ = &output.arg("-p");
                        let _ = &output.arg(format!("{}", &_port));
                    }
                    if !&_name.is_empty() {
                        let _ = &output.arg("--name");
                        let _ = &output.arg(&_name);
                    }
                    if !&_restart.is_empty() {
                        let _ = &output.arg("--restart");
                        let _ = &output.arg(&_restart);
                    }
                    if !&_env.is_empty() {
                        for item in &_env {
                            // The default is a vector with an empty string
                            // so this needs to also be checked.
                            if !item.is_empty() {
                                let _ = &output.arg("-e");
                                let _ = &output.arg(item);
                            }
                        }
                    }
                    let shell = output.arg(&_image).output().await?;
                println!(
                    "stdout: {}\n\n\n\nstderr: {}",
                    String::from(from_utf8(&shell.stdout)?),
                    String::from(from_utf8(&shell.stderr)?)
                );
            }
            Err(_) => {
                println!("Could not connect to {}", &_node);
            }
        }
        Ok(())
    }

    pub async fn send_stop_command(&self) -> Result<(), Box<dyn Error>> {
        let mut _node: String = String::new();
        let mut _container: String = String::new();
        match &self.command {
            DockerCommand::Stop { node, container } => {
                _node = node.clone();
                _container = container.clone();
            }
            _ => panic!("error in send_log_command"),
        };
        let session = openssh::SessionBuilder::default()
            .connect_timeout(std::time::Duration::from_secs(1))
            .connect(&_node)
            .await;
        println!("host {:?}", &_node);

        match session {
            Ok(session) => {
                let output = session
                    .command("sudo")
                    .arg("docker")
                    .arg("stop")
                    .arg(_container)
                    .output()
                    .await?;
                println!(
                    "stdout: {}\n\n\n\nstderr: {}",
                    String::from(from_utf8(&output.stdout)?),
                    String::from(from_utf8(&output.stderr)?)
                );
            }
            Err(_) => {
                println!("Could not connect to {}", &_node);
            }
        }
        Ok(())
    }

    pub async fn send_rm_command(&self) -> Result<(), Box<dyn Error>> {
        let mut _node: String = String::new();
        let mut _container: String = String::new();
        match &self.command {
            DockerCommand::Rm { node, container } => {
                _node = node.clone();
                _container = container.clone();
            }
            _ => panic!("error in send_log_command"),
        };
        let session = openssh::SessionBuilder::default()
            .connect_timeout(std::time::Duration::from_secs(1))
            .connect(&_node)
            .await;
        println!("host {:?}", &_node);

        match session {
            Ok(session) => {
                let output = session
                    .command("sudo")
                    .arg("docker")
                    .arg("rm")
                    .arg(_container)
                    .output()
                    .await?;
                println!(
                    "stdout: {}\n\n\n\nstderr: {}",
                    String::from(from_utf8(&output.stdout)?),
                    String::from(from_utf8(&output.stderr)?)
                );
            }
            Err(_) => {
                println!("Could not connect to {}", &_node);
            }
        }
        Ok(())
    }

}
