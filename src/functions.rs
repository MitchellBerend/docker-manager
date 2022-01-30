// This module defines all helper functions for the main program


use std::{io::Read, process::Output};
use std::str::from_utf8;
use std::error::Error;

use log::{info, debug};
use openssh::Session;


pub fn get_nodes(regex: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut config_buf: Vec<u8> = vec![];
    let mut _path = std::env::var("HOME")?;
    _path.push_str("/.ssh/config");
    let mut ssh_conf_file = std::fs::File::open(_path)?;

    ssh_conf_file.read_to_end(&mut config_buf)?;
    let config_str: String = String::from(from_utf8(&config_buf)?);

    let hostname_regex = regex::Regex::new(&format!("[^#]Host {}", &regex))?;
    let regex_iter = hostname_regex.find_iter(&config_str);
    let mut nodes: Vec<String> = vec![];
        for host in regex_iter {
            nodes.push(String::from(host.as_str().split_once(" ").unwrap().1));
        }
    Ok(nodes)
}

pub async fn send_command_node_container(command: String, node: String, container: String) -> Result<(), Box<dyn Error>> {
    info!("node: {}, container: {}", &node, &container);
    debug!("running docker {}", &command);
    debug!("connecting to {}", &node);
    let session = openssh::SessionBuilder::default()
        .connect_timeout(std::time::Duration::from_secs(1))
        .connect(&node)
        .await;

    debug!("running command docker {} on {}", &command, &container);
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
        Err(_) => {
            println!("Could not connect to {}", &node);
        }
    }
    Ok(())
}

pub async fn get_remote_output_command(node: String, commands: &[String]) -> String {
    let mut return_str = String::new();
    debug!("connecting to {}", &node);
    let session = openssh::SessionBuilder::default()
        .connect_timeout(std::time::Duration::new(1, 0))
        .connect(&node)
        .await;
    return_str.push_str(&format!("host {:?}\n", &node));
    match session {
        Ok(session) => {
            info!("running command docker ps on {}", &node);
            let output = build_output(session, &commands).await;
            return_str.push_str(&String::from(from_utf8(&output.stdout).unwrap()));
        }
        Err(_) => {
            return_str.push_str(&format!("Could not connect to {}", &node));
        }
    }
    return_str
}

async fn build_output(session: Session, commands: &[String]) -> Output {
    let mut output = session.command("sudo");
    output.arg("docker");
    for command in commands {
        output.arg(command);
    }
    output.output().await.unwrap()
}