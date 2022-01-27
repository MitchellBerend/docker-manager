// This binary functions as a cli to make managing docker applications on multiple nodes easier.
// functions it will mimick are:
// ps (done)
//      This will show a list of all containers and what node it is on
// exec (done)
//      This will execute a command on the specified docker container
//      no flags will be present for now
// logs (done)
//      This will fetch the logs of specified docker containers
// restart
//      This will restart a specific docker container
// run
//      This will start a new container with the specified flags

use std::error::Error;
use std::io::Read;
use std::str::from_utf8;

use clap::Parser;

mod parser;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = parser::MainParser::parse();

    let mut config_buf: Vec<u8> = vec![];
    let mut _path = std::env::var("HOME")?;
    _path.push_str("/.ssh/config");
    let mut ssh_conf_file = std::fs::File::open(_path)?;

    ssh_conf_file.read_to_end(&mut config_buf)?;
    let config_str: String = String::from(from_utf8(&config_buf)?);

    let hostname_regex = regex::Regex::new(&format!("[^#]Host {}", &args.regex))?;
    let regex_iter = hostname_regex.find_iter(&config_str);

    // explicit drop block since these are not needed anymore
    {
        drop(ssh_conf_file);
    }

    match args.command {
        parser::DockerCommand::Ps => {
            let mut nodes: Vec<String> = vec![];
            for host in regex_iter {
                nodes.push(String::from(host.as_str().split_once(" ").unwrap().1));
            }
            args.send_ps_command(&nodes).await?;
        },
        parser::DockerCommand::Exec {
            ref node,
            ref container,
            ref command,
        } => {
            args.send_exec_command(node, container, command).await?;
        },
        parser::DockerCommand::Logs { ref node, ref container } => {
            args.send_log_command(&node, &container).await?;
        },
        parser::DockerCommand::Run { node: _, image: _, name: _, port:_ } => {
            println!("docker run");
        },
    }

    Ok(())
}
