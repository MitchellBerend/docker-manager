// This binary functions as a cli to make managing docker applications on multiple nodes easier.
// functions it will mimick are:
// ps (done)
//      This will show a list of all containers and what node it is on
// exec (done)
//      This will execute a command on the specified docker container
//      no flags will be present for now
// logs (done)
//      This will fetch the logs of specified docker containers
// restart (done)
//      This will restart a specific docker container
// run (done)
//      This will start a new container with the specified flags
// stop (done)
//      This will stop a specified container on a specified node
// rm (done)
//      This will remove a specified container on a specified node
//
// TODO 
// Add proper debug logging
// use log crate 


use std::error::Error;
use std::io::Read;
use std::str::from_utf8;

use clap::Parser;
use log::{LevelFilter};

mod logger;
mod parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = parser::MainParser::parse();
    
    match args.level {
        parser::Level::Debug => { 
            log::set_logger(&logger::MY_LOGGER).unwrap();
            log::set_max_level(LevelFilter::Debug);
        },
        parser::Level::Info => {
            log::set_logger(&logger::MY_LOGGER).unwrap();
            log::set_max_level(LevelFilter::Info);
        }
        parser::Level::Warning => {
            log::set_logger(&logger::MY_LOGGER).unwrap();
            log::set_max_level(LevelFilter::Warn);
        },
        parser::Level::Error => {
            log::set_logger(&logger::MY_LOGGER).unwrap();
            log::set_max_level(LevelFilter::Error); 
        },
    }



    match args.command {
        parser::DockerCommand::Ps {ref regex} => {
            let mut _regex: String = regex.clone();
            let mut config_buf: Vec<u8> = vec![];
            let mut _path = std::env::var("HOME")?;
            _path.push_str("/.ssh/config");
            let mut ssh_conf_file = std::fs::File::open(_path)?;

            ssh_conf_file.read_to_end(&mut config_buf)?;
            let config_str: String = String::from(from_utf8(&config_buf)?);

            let hostname_regex = regex::Regex::new(&format!("[^#]Host {}", &_regex))?;
            let regex_iter = hostname_regex.find_iter(&config_str);

            // explicit drop block since these are not needed anymore
            {
                drop(ssh_conf_file);
            }
            let mut nodes: Vec<String> = vec![];
            for host in regex_iter {
                nodes.push(String::from(host.as_str().split_once(" ").unwrap().1));
            }
            args.send_ps_command(&nodes).await?;
        }
        parser::DockerCommand::Exec { node: _, container: _, command: _ } => {
            args.send_exec_command().await?;
        }
        parser::DockerCommand::Logs { node: _, container: _ } => {
            args.send_log_command().await?;
        }
        parser::DockerCommand::Run { node: _, image: _, name: _, port: _, restart: _, env: _ } => {
            args.send_run_command().await?;
        }
        parser::DockerCommand::Stop { node: _, container: _ } => {
            args.send_stop_command().await?;
        }
        parser::DockerCommand::Rm { node: _, container: _ } => {
            args.send_rm_command().await?;
        }
    }

    Ok(())
}
