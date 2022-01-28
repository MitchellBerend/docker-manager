// This module defines the main cli and all its arguments

use std::error::Error;
use std::str::from_utf8;

use clap::Parser;
use futures::{stream, StreamExt};
use log::{debug};
use crate::dockercommand::DockerCommand;


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
    about = "This tool mimics the functionality of the docker cli but abstracts its over all connected nodes defined in the current users ~/.ssh/config."
)]
pub struct MainParser {
    /// The docker cli command to be executed.
    #[clap(subcommand)]
    pub command: DockerCommand,

    /// Setting information output level.
    #[clap(arg_enum, short = 'l', default_value = "info")]
    pub level: Level,
}


impl MainParser {
    pub async fn send_ps_command(&self, nodes: &[String]) -> Result<(), Box<dyn Error>> {
        debug!("searching nodes: {:?}", &nodes);
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
            _ => {
                // debug!("send_log_command was called with {:?}", &self.command);
                panic!("error in send_log_command")
                // replace with proper error log and return Ok(())
            },
        };
        debug!("_node: {}, _container: {}", &_node, &_container);
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
            _ => {
                panic!("error in send_log_command")
                // replace with proper error log and return Ok(())
            },
        };
        debug!("_node: {}, _container: {}, _command: {}", &_node, &_container, &_command);
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
            _ => {
                panic!("error in send_log_command")
                // replace with proper error log and return Ok(())
            },
        };
        debug!("_node: {}, _image: {}", &_node, &_image);
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
            _ => {
                panic!("error in send_stop_command")
                // replace with proper error log and return Ok(())
            },
        };
        debug!("_node: {}, _container: {}", &_node, &_container);
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
            _ => {
                panic!("error in send_rm_command")
                // replace with proper error log and return Ok(())
            },
        };
        debug!("_node: {}, _container: {}", &_node, &_container);
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

    pub async fn send_inspect_command(&self) -> Result<(), Box<dyn Error>> {
        let mut _node: String = String::new();
        let mut _container: String = String::new();
        match &self.command {
            DockerCommand::Inspect { node, container } => {
                _node = node.clone();
                _container = container.clone();
            }
            _ => {
                panic!("error in send_inspect_command")
                // replace with proper error log and return Ok(())
            },
        };
        debug!("_node: {}, _container: {}", &_node, &_container);
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
                    .arg("inspect")
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

    pub async fn send_image_command(&self, nodes: &[String]) -> Result<(), Box<dyn Error>> {
        debug!("searching nodes: {:?}", &nodes);
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
                            .arg("image")
                            .arg("ls")
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
}
