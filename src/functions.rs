// This module defines all helper functions for the main program

use std::str::from_utf8;
use std::{io::Read, process::Output};

use anyhow::Result;
use futures::{stream, StreamExt};
use log::{debug, error, info};
use openssh::Session;

use crate::structs::NodeMemory;

pub fn get_nodes(regex: String) -> Result<Vec<String>> {
    let mut config_buf: Vec<u8> = vec![];
    let mut _path = std::env::var("HOME")?;
    _path.push_str("/.ssh/config");
    let mut ssh_conf_file = std::fs::File::open(_path)?;

    ssh_conf_file.read_to_end(&mut config_buf)?;
    let config_str: String = String::from(from_utf8(&config_buf)?);
    debug!("regex pattern: ^Host {regex}");
    let hostname_regex = regex::Regex::new(&format!(r###"[^#]Host {}"###, &regex))?;
    let regex_iter = hostname_regex.find_iter(&config_str);
    let mut nodes: Vec<String> = vec![];
    for host in regex_iter {
        nodes.push(String::from(host.as_str().split_once(" ").unwrap().1));
    }
    Ok(nodes)
}

pub async fn send_command_node_container(
    command: String,
    node: String,
    container: String,
) -> Result<()> {
    info!("node: {node}, container: {container}");
    debug!("running docker {command}");
    debug!("connecting to {node}");
    let session = openssh::SessionBuilder::default()
        .connect_timeout(std::time::Duration::from_secs(1))
        .connect(&node)
        .await;

    debug!("running command docker {command} on {container}");
    match session {
        Ok(session) => {
            println!("host {:?}", &node);
            let output = session
                .command("sudo")
                .arg("docker")
                .arg(command)
                .arg(container)
                .output()
                .await?;
            println!(
                "stdout: {}\n\n\n\nstderr: {}",
                String::from(from_utf8(&output.stdout)?),
                String::from(from_utf8(&output.stderr)?)
            );
        }
        Err(e) => {
            error!("Running check since there was a connection error with node: {node}");
            error!("{}", e);
            let session = openssh::SessionBuilder::default()
                .connect_timeout(std::time::Duration::from_secs(1))
                .connect(&node)
                .await?;
            let _ = session.check().await?;
            error!("Could not connect to {node}");
        }
    }
    Ok(())
}

pub async fn send_command_node(node: String, commands: &[String]) -> String {
    let mut return_str = String::new();
    debug!("connecting to {node}");
    let session = openssh::SessionBuilder::default()
        .connect_timeout(std::time::Duration::new(1, 0))
        .connect(&node)
        .await;
    return_str.push_str(&format!("host {:?}\n", &node));
    match session {
        Ok(session) => {
            let output = build_output(session, commands).await;
            info!("running command docker {:?} on {node}", commands);
            match output {
                Ok(output) => {
                    return_str.push_str(&String::from(from_utf8(&output.stdout).unwrap()));
                }
                Err(e) => {
                    error!("{}", e)
                }
            }
        }
        Err(e) => {
            error!("Running check since there was a connection error with node: {node}");
            error!("{}", e);
            println!("Could not connect to {node}");
        }
    }
    return_str
}

async fn build_output(session: Session, commands: &[String]) -> Result<Output> {
    debug!("building command: {:#?}", commands);
    let mut output = session.command("sudo");
    output.arg("docker");
    for command in commands {
        output.arg(command);
    }
    Ok(output.output().await?)
}

pub async fn get_memory_information(
    nodes: &[String],
    concurrent_requests: usize,
) -> Vec<NodeMemory> {
    let bodies = stream::iter(nodes)
        .map(|node| async move {
            let mut return_str = String::new();
            debug!("connecting to {node}");
            let session = openssh::SessionBuilder::default()
                .connect_timeout(std::time::Duration::new(1, 0))
                .connect(&node)
                .await
                .unwrap();
            let mut return_value = NodeMemory::default();
            let mut output = session.command("sudo");
            output.arg("cat").arg("/proc/meminfo");
            debug!("running cat /proc/meminfo on {}", &node);
            match output.output().await {
                Ok(output) => {
                    let memory_regex = regex::Regex::new(r###"MemTotal.*\nMemFree.*"###).unwrap();
                    let regex_iter = memory_regex.find_iter(from_utf8(&output.stdout).unwrap());
                    for _match in regex_iter {
                        return_str.push_str(_match.as_str());
                        return_str.push('\n');
                        let placeholder = &_match.as_str().split_once("\n").unwrap();
                        let memtotal = placeholder.0.split_whitespace().nth(1).unwrap();
                        let memfree = placeholder.1.split_whitespace().nth(1).unwrap();
                        return_value = NodeMemory {
                            node: node.clone(),
                            memtotal: str::parse::<u64>(memtotal).unwrap(),
                            memfree: str::parse::<u64>(memfree).unwrap(),
                        }
                    }
                }
                Err(e) => {
                    error!("{}", e)
                }
            }
            return_value
        })
        .buffer_unordered(concurrent_requests);
    bodies.collect::<Vec<NodeMemory>>().await
}
