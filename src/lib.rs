pub mod constants;

mod cli;
mod client;
mod utility;

use clap::{CommandFactory, Parser};

pub async fn run() {
    let mut _cli = cli::App::parse();

    match _cli.command {
        cli::Command::Completion => utility::command::run_completion(&mut cli::App::command()),
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
