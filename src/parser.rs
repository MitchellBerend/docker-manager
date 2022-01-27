// This module defines the main cli and all its arguments

use std::error::Error;
use std::str::from_utf8;

use clap::Parser;
use futures::{stream, StreamExt};


const CONCURRENT_REQUESTS: usize = 10;


#[derive(clap::Subcommand)]
pub enum DockerCommand {
    /// This will show a list of all containers and what node it is on.
    Ps,

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
        #[clap(short = 'n', long, required = false)]
        name: String,

        /// The port map of the container
        #[clap(
            short,
            long,
            multiple_occurrences = true,
            value_delimiter = ',',
            default_value = ""
        )]
        port: String,
    }
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

    #[clap(short, long, default_value = ".*")]
    pub regex: String,
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
    
    pub async fn send_log_command(&self, node: &str, container: &str) -> Result<(), Box<dyn Error>> {
        let session = openssh::SessionBuilder::default()
            .connect_timeout(std::time::Duration::from_secs(1))
            .connect(node)
            .await;
        println!("host {:?}", &node);
        match session {
            Ok(session) => {
                let output = session
                    .command("sudo")
                    .arg("docker")
                    .arg("logs")
                    .arg(container)
                    .output()
                    .await?;
                println!(
                    "stdout: {}\n\n\n\nstderr: {}",
                    String::from(from_utf8(&output.stdout)?),
                    String::from(from_utf8(&output.stderr)?)
                );
            }
            Err(_) => {
                println!("Could not connect to {}", &node);
            }
        }
        Ok(())
    }
    
    pub async fn send_exec_command(
        &self,
        node: &str,
        container: &str,
        command: &str,
    ) -> Result<(), Box<dyn Error>> {
        let session = openssh::SessionBuilder::default()
            .connect_timeout(std::time::Duration::new(1, 0))
            .connect(node)
            .await;
        println!("host {:?}", &node);
        match session {
            Ok(session) => {
                let output = session
                    .command("sudo")
                    .arg("docker")
                    .arg("exec")
                    .arg(container)
                    .arg(command)
                    .output()
                    .await?;
                println!(
                    "stdout: {}\n\n\n\nstderr: {}",
                    String::from(from_utf8(&output.stdout)?),
                    String::from(from_utf8(&output.stderr)?)
                );
            }
            Err(_) => {
                println!("Could not connect to {}", &node);
            }
        }
        Ok(())
    }
    
}