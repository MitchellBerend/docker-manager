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
// images (done)
//      This will show all images on a node
// info (done)
//      This will show docker info of all nodes
// start (done)
//      This will start a container on a specific node
// deploy (done)
//      This will copy a dockerfile to an automatically picked node and build the image
// inspect (done)
//      This gives low level information on a specified container

// TODO 
// Add proper debug logging (done)


use std::error::Error;

use clap::Parser;
use log::{LevelFilter};
use dockercommand::DockerCommand;

mod logger;
mod parser;
mod dockercommand;
mod functions;
mod structs;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = parser::MainParser::parse();
    log::set_logger(&logger::MY_LOGGER).unwrap();

    match args.level {
        parser::Level::Debug => { 
            log::set_max_level(LevelFilter::Debug);
        },
        parser::Level::Info => {
            log::set_max_level(LevelFilter::Info);
        }
        parser::Level::Warning => {
            log::set_max_level(LevelFilter::Warn);
        },
        parser::Level::Error => {
            log::set_max_level(LevelFilter::Error); 
        },
    }

    match args.command {
        DockerCommand::Ps {ref regex} => {
            let nodes = functions::get_nodes(regex.clone())?;
            args.send_ps_command(&nodes).await;
        }
        DockerCommand::Exec { node: _, container: _, command: _ } => {
            args.send_exec_command().await?;
        }
        DockerCommand::Logs { node: _, container: _ } => {
            args.send_log_command().await?;
        }
        DockerCommand::Run { node: _, image: _, name: _, port: _, restart: _, env: _ } => {
            args.send_run_command().await?;
        }
        DockerCommand::Stop { node: _, container: _ } => {
            args.send_stop_command().await?;
        }
        DockerCommand::Rm { node: _, container: _ } => {
            args.send_rm_command().await?;
        }
        DockerCommand::Inspect { node: _, container: _ } => {
            args.send_inspect_command().await?;
        }
        DockerCommand::Images { ref regex} => {
            let nodes = functions::get_nodes(regex.clone())?;
            args.send_images_command(&nodes).await;
        }
        DockerCommand::Info { ref regex } => {
            let nodes = functions::get_nodes(regex.clone())?;
            args.send_info_command(&nodes).await;
        }
        DockerCommand::Top { node: _, container: _ } => {
            args.send_top_command().await?;
        }
        DockerCommand::Start { node: _, container: _ } => {
            args.send_start_command().await?;
        }
        DockerCommand::Deploy { ref regex,ref project_name, ref file } => {
            let nodes = functions::get_nodes(regex.clone())?;
            args.send_deploy_command(&nodes, project_name.clone(), file.clone()).await?;
        }
    }
    Ok(())
}
