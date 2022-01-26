// This binary functions as a cli to make managing docker applications on multiple nodes easier.
// functions it will mimick are:
// ps
//      This will show a list of all containers and what node it is on
// exec
//      This will execute a command on the specified docker container
//      no flags will be present for now
// logs
//      This will fetch the logs of specified docker containers
// restart
//      This will restart a specific docker container

use std::error::Error;
use std::io::Read;
use std::str::from_utf8;

use tokio;
use clap::Parser;
use regex;
use futures::{stream, StreamExt};

mod parser;

const CONCURRENT_REQUESTS: usize = 10;



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = parser::MainParser::parse();

    let mut config_buf: Vec<u8> = vec!();
    let mut _path = std::env::var("HOME")?;
    _path.push_str("/.ssh/config");
    let mut ssh_conf_file = std::fs::File::open(_path)?;

    ssh_conf_file.read_to_end(&mut config_buf)?;
    let config_str = from_utf8(&config_buf)?;

    let hostname_regex = regex::Regex::new(&format!("[^#]Host {}", &args.regex))?;
    let regex_iter = hostname_regex.find_iter(config_str);

    drop(ssh_conf_file);

    match args.command {
        parser::DockerCommand::Ps => {
            let mut nodes: Vec<String> = vec!();
            for host in regex_iter {
                nodes.push(String::from(host.as_str().split_once(" ").unwrap().1));
            }
            send_ps_command(&nodes).await?;
        },
        parser::DockerCommand::Exec {node, container, command} => {
            send_exec_command(&node, &container, &command).await?;
        }
        parser::DockerCommand::Logs {node, container } => {
            send_log_command(&node, &container).await?;
        }
    }

    Ok(())
}

async fn send_ps_command(nodes: &[String]) -> Result<(), Box<dyn Error>> {
    let _bodies = stream::iter(nodes)
        .map( |node| async move {
            let mut return_str = String::new();
            let owned_node = node.clone();
            let session = openssh::SessionBuilder::default()
                .connect_timeout(std::time::Duration::new(1,0))
                .connect(&owned_node).await;
            return_str.push_str(&format!("host {:?}\n", &owned_node));
            match session {
                Ok(session) => {
                    let output = session.command("sudo")
                    .arg("docker")
                    .arg("ps")
                    .arg("-a").output().await.unwrap();
                    return_str.push_str(&format!("{}", String::from(from_utf8(&output.stdout).unwrap())));
                },
                Err(_) => {
                    return_str.push_str(&format!("Could not connect to {}", &owned_node));
                }
            }
            return_str
        }).buffer_unordered(CONCURRENT_REQUESTS);
    _bodies.for_each(|body| async move {
        println!("{}", body);
    }).await;

    Ok(())
}

async fn send_log_command(node: &str, container: &str) -> Result<(), Box<dyn Error>> {
    let session = openssh::SessionBuilder::default()
    .connect_timeout(std::time::Duration::new(1,0))
    .connect(node).await;
    println!("host {:?}", &node);
    match session {
        Ok(session) => {
            let output = session.command("sudo")
            .arg("docker")
            .arg("logs")
            .arg(container).output().await?;
            println!("stdout: {}\n\n\n\nstderr: {}", String::from(from_utf8(&output.stdout)?), String::from(from_utf8(&output.stderr)?));
        },
        Err(_) => {
            println!("Could not connect to {}", &node);
        }
    }
    Ok(())
}

async fn send_exec_command(node: &str, container: &str, command: &str) -> Result<(), Box<dyn Error>> {
    let session = openssh::SessionBuilder::default()
    .connect_timeout(std::time::Duration::new(1,0))
    .connect(node).await;
    println!("host {:?}", &node);
    match session {
        Ok(session) => {
            let output = session.command("sudo")
            .arg("docker")
            .arg("exec")
            .arg(container)
            .arg(command).output().await?;
            println!("stdout: {}\n\n\n\nstderr: {}", String::from(from_utf8(&output.stdout)?), String::from(from_utf8(&output.stderr)?));
        },
        Err(_) => {
            println!("Could not connect to {}", &node);
        }
    }
    Ok(())
}