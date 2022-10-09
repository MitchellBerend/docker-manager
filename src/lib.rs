pub mod constants;

mod cli;
mod client;
mod utility;

use clap::{CommandFactory, Parser};
use clap_complete::generate;

pub async fn run() {
    let mut _cli = cli::App::parse();

    match _cli.command {
        cli::Command::Completion => {
            let mut cmd = cli::App::command();
            let cmd_name: String = cmd.get_name().into();
            generate(
                clap_complete::Shell::Bash,
                &mut cmd,
                cmd_name,
                &mut std::io::stdout(),
            );
        }

        //utility::command::run_completion(&mut cli::App::command()),
        _ => {
            for word in utility::run_command(_cli.command).await {
                match word {
                    Ok(s) => println!("{}", s),
                    Err(e) => println!("{}", e),
                }
            }
        }
    }
}
