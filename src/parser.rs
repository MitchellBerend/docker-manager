// This module defines the main cli

use std::str::from_utf8;

use clap::Parser;
use futures::{stream, StreamExt};
use log::{info, debug, error, warn};
use anyhow::Result;
use crate::functions::{send_command_node, get_memory_information};
use crate::{dockercommand::DockerCommand, functions::send_command_node_container};
use crate::structs::NodeMemory;

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
    async fn send_async_command(&self,nodes: &[String], commands: &[String]) -> Vec<String> {
        let bodies = stream::iter(nodes)
        .map(|node| async move {
            send_command_node(node.clone(), commands).await
        }).buffer_unordered(CONCURRENT_REQUESTS);
        bodies.collect::<Vec<String>>().await
    }

    pub async fn send_ps_command(&self, nodes: &[String]) {
        info!("searching nodes: {:?}", &nodes);
        debug!("running docker ps");
        let commands: [String; 2] = ["ps".to_string(), "-a".to_string()];
        let results = self.send_async_command(nodes, &commands).await;
        for result in results {
            println!("{result}");
        }
    }

    pub async fn send_log_command(&self) -> Result<()> {
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

    pub async fn send_exec_command(&self) -> Result<()> {
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
                error!("Could not connect to {_node}");
            }
        }
        Ok(())
    }

    pub async fn send_run_command(&self) -> Result<()> {
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
                error!("Could not connect to {_node}");
            }
        }
        Ok(())
    }

    pub async fn send_stop_command(&self) -> Result<()> {
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

    pub async fn send_rm_command(&self) -> Result<()> {
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

    pub async fn send_inspect_command(&self) -> Result<()> {
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

    pub async fn send_images_command(&self, nodes: &[String]) {
        info!("searching nodes: {:#?}", nodes);
        debug!("running docker image ls");
        let commands: [String; 1] = ["images".to_string()];
        let results = self.send_async_command(nodes, &commands).await;
        for result in results {
            println!("{result}");
        }
    }

    pub async fn send_info_command(&self, nodes: &[String]) {
        info!("searching nodes: {:?}", nodes);
        debug!("running docker info");
        let commands: [String; 1] = ["info".to_string()];
        let results = self.send_async_command(nodes, &commands).await;
        for result in results {
            println!("{result}");
        }
    }

    pub async fn send_top_command(&self) -> Result<()> {
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

    pub async fn send_start_command(&self) -> Result<()> {
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

    pub async fn send_deploy_command(&self, nodes: &[String], project_name: String, file: String) -> Result<()> {
        // get all resources of nodes
        info!("searching nodes: {:?}", nodes);
        debug!("running docker ps");
        let results = get_memory_information(nodes, CONCURRENT_REQUESTS).await;

        // decide which node is going to run the container
        let mut _picked_node: Option<NodeMemory> = Option::None;
        for result in results {
            let node = result.node.clone();
            let memtotal = result.memtotal;
            let memfree = result.memfree;
            let new_node = Some(NodeMemory {
                node,
                memtotal,
                memfree,
            });
            debug!("\nhost:\t\t{}\nMemTotal:\t{memtotal}\nMemFree\t\t{memfree}\n", &result.node);
            match _picked_node {
                Some(ref _node) => {
                    if _node.memtotal - _node.memfree > memtotal - memfree {
                        _picked_node = new_node;
                    } else {}
                },
                None => {
                    _picked_node = new_node;
                },
            }
        }
        debug!("{:?}", _picked_node.as_ref().unwrap());
        // connect to chose node
        let picked_node = _picked_node.unwrap();
        println!("Deploying image to: {}", picked_node.node);
        let session = openssh::SessionBuilder::default()
                    .connect_timeout(std::time::Duration::new(1, 0))
                    .connect(&picked_node.node)
                    .await.unwrap();

        // mkdir for config
        let args = format!("/srv/{project_name}");
        let output = session.command("sudo")
            .arg("mkdir")
            .arg(args).output().await?;
        debug!("Creating {file} dir");
        debug!("stdout: {}", from_utf8(&output.stdout)?);
        debug!("stderr: {}", from_utf8(&output.stderr)?);

        // copy the config file to the target node
        let mut local_shell = std::process::Command::new("scp");
        local_shell.arg(file.to_string())
            .arg(format!("{}:~/", &picked_node.node));
        let local_output = local_shell.output()?;
        debug!("moving {file} to {}", &picked_node.node);
        debug!("stdout: {}", from_utf8(&local_output.stdout)?);
        debug!("stderr: {}", from_utf8(&local_output.stderr)?);

        // moving file from remote home to correct dir
        let output = session.command("sudo")
            .arg("mv")
            .arg("Dockerfile")
            .arg(format!("/srv/{project_name}")).output().await?;
        debug!("moving Dockerfile to /srv/{project_name}");
        debug!("stdout: {}", from_utf8(&output.stdout)?);
        debug!("stderr: {}", from_utf8(&output.stderr)?);

        // run the command with non relative paths to volumes if they are required
        let suffix: [&str; 8] = [
            &*format!("/srv/{project_name}"),
            "&&",
            "sudo",
            "docker",
            "build",
            "-t",
            &*format!("{project_name}:latest"),
            ".",
        ];
        let output = session.command("cd")
            .raw_args(suffix)
            .output().await?;
        debug!("building docker image for /srv/{project_name}");
        debug!("remote command: {:?}", suffix);
        debug!("stdout: {}", from_utf8(&output.stdout)?);
        debug!("stderr: {}", from_utf8(&output.stderr)?);

        warn!("
        This does not actually deploy the image yet.
        You need to manually run the image.
        It the image name is the name of the project and the tag is latest.
        run the following command to actually run the image on that node:\n
            docker-manager run {} {}:latest\n", picked_node.node, project_name);
        Ok(())
    }
}
