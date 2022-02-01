// This module defines the main cli

use std::{error::Error};
use std::str::from_utf8;


use clap::Parser;
use futures::{stream, StreamExt};
use log::{info, debug, error};
use crate::functions::{send_command_node};
use crate::{dockercommand::DockerCommand, functions::send_command_node_container};


const CONCURRENT_REQUESTS: usize = 10;


#[derive(clap::ArgEnum, Clone)]
pub enum Level {
    Debug,
    Info,
    Warning,
    Error,
}


#[derive(Parser)]
#[clap(
    author = "Mitchell Berendhuysen",
    version,
    about = "This tool mimics the functionality of the docker cli but abstracts it over all connected nodes defined in the current users ~/.ssh/config."
)]
pub struct MainParser {
    /// The docker cli command to be executed.
    #[clap(subcommand)]
    pub command: DockerCommand,

    /// Setting information output level.
    #[clap(arg_enum, short = 'l', default_value = "warning")]
    pub level: Level,
}


impl MainParser {
    pub async fn send_ps_command(&self, nodes: &[String]) -> Result<(), Box<dyn Error>> {
        info!("searching nodes: {:?}", &nodes);
        debug!("running docker ps");
        let _bodies = stream::iter(nodes)
            .map(|node| async move {
                let commands: [String; 2] = ["ps".to_string(), "-a".to_string()];
                send_command_node(node.clone(), &commands).await
            }).buffer_unordered(CONCURRENT_REQUESTS);
        _bodies
            .for_each(|body| async move {
                println!("{body}");
            }).await;
        Ok(())
    }

    pub async fn send_log_command(&self) -> Result<(), Box<dyn Error>> {
        match &self.command {
            DockerCommand::Logs { node, container } => {
                debug!("node: {node}, container: {container}");
                send_command_node_container(
                    "logs".to_string(),
                    node.clone(),
                    container.clone(),
                ).await?
            }
            _ => {
                error!("The send_log_command was somehow called with {:#?}", &self.command);
                panic!("error in send_log_command")
            },
        };
        Ok(())
    }

    pub async fn send_exec_command(&self) -> Result<(), Box<dyn Error>> {
        let mut _node: String = String::new();
        let mut _container: String = String::new();
        let mut _command: String = String::new();
        debug!("running docker exec on {_container}");
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
            _ => {
                error!("The send_log_command was somehow called with {:#?}", &self.command);
                panic!("error in send_log_command")
            },
        };
        debug!("connecting to {_node}");
        let session = openssh::SessionBuilder::default()
            .connect_timeout(std::time::Duration::new(1, 0))
            .connect(&_node)
            .await;
        println!("host {:?}", &_node);
        match session {
            Ok(session) => {
                info!("running command docker exec on {_container}");
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
                println!("Could not connect to {_node}");
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
        debug!("running docker run on {_node}");
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
            _ => {
                error!("The send_log_command was somehow called with {:#?}", &self.command);
                panic!("error in send_log_command")
            },
        };
        debug!("connecting to {_node}");
        let session = openssh::SessionBuilder::default()
            .connect_timeout(std::time::Duration::new(1, 0))
            .connect(&_node)
            .await;
        println!("host {:?}", &_node);
        match session {
            Ok(session) => {
                info!("running command docker run on {_node}");
                let mut output = session.command("sudo");
                let _ = &output.arg("docker")
                    .arg("run")
                    .arg("-d");
                    if !&_port.is_empty() {
                        let _ = &output.arg("-p");
                        let _ = &output.arg((&_port).to_string());
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
                println!("Could not connect to {_node}");
            }
        }
        Ok(())
    }

    pub async fn send_stop_command(&self) -> Result<(), Box<dyn Error>> {
        match &self.command {
            DockerCommand::Stop { node, container } => {
                debug!("node: {node}, container: {container}");
                send_command_node_container(
                    "stop".to_string(),
                    node.clone(),
                    container.clone(),
                ).await?
            }
            _ => {
                error!("The send_log_command was somehow called with {:#?}", &self.command);
                panic!("error in send_stop_command")
            },
        };
        Ok(())
    }

    pub async fn send_rm_command(&self) -> Result<(), Box<dyn Error>> {
        match &self.command {
            DockerCommand::Rm { node, container } => {
                debug!("node: {node}, container: {container}");
                send_command_node_container(
                    "rm".to_string(),
                    node.clone(),
                    container.clone(),
                ).await?
            }
            _ => {
                error!("The send_log_command was somehow called with {:#?}", &self.command);
                panic!("error in send_rm_command")
            },
        };
        Ok(())
    }

    pub async fn send_inspect_command(&self) -> Result<(), Box<dyn Error>> {
        match &self.command {
            DockerCommand::Inspect { node, container } => {
                debug!("node: {node}, container: {container}");
                send_command_node_container(
                    "inspect".to_string(),
                    node.clone(),
                    container.clone(),
                ).await?
            }
            _ => {
                panic!("error in send_inspect_command")
            },
        };
        Ok(())
    }

    pub async fn send_images_command(&self, nodes: &[String]) -> Result<(), Box<dyn Error>> {
        info!("searching nodes: {:#?}", &nodes);
        debug!("running docker image ls");
        let _bodies = stream::iter(nodes)
            .map(|node| async move {
                let commands: [String; 1] = ["images".to_string()];
                send_command_node(node.clone(), &commands).await
            })
            .buffer_unordered(CONCURRENT_REQUESTS);
        _bodies
            .for_each(|body| async move {
                println!("{body}");
            })
            .await;
        Ok(())
    }

    pub async fn send_info_command(&self, nodes: &[String]) -> Result<(), Box<dyn Error>> {
        info!("searching nodes: {:?}", &nodes);
        debug!("running docker info");
        let _bodies = stream::iter(nodes)
            .map(|node| async move {
                let commands: [String; 1] = ["info".to_string()];
                send_command_node(node.clone(), &commands).await
            })
            .buffer_unordered(CONCURRENT_REQUESTS);
        _bodies
            .for_each(|body| async move {
                println!("{body}");
            })
            .await;
        Ok(())
    }

    pub async fn send_top_command(&self) -> Result<(), Box<dyn Error>> {
        match &self.command {
            DockerCommand::Top { node, container } => {
                send_command_node_container(
                    "top".to_string(),
                    node.clone(),
                    container.clone(),
                ).await?
            }
            _ => {
                error!("The send_log_command was somehow called with {:#?}", &self.command);
                panic!("error in send_top_command")
            },
        };
        Ok(())
    }

    pub async fn send_start_command(&self) -> Result<(), Box<dyn Error>> {
        match &self.command {
            DockerCommand::Start { node, container } => {
                debug!("node: {node}, container: {container}");
                send_command_node_container(
                    "start".to_string(),
                    node.clone(),
                    container.clone(),
                ).await?
            }
            _ => {
                error!("The send_log_command was somehow called with {:#?}", &self.command);
                panic!("error in send_start_command")
            },
        };
        Ok(())
    }
}
