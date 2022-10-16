pub mod constants;

mod cli;
mod client;
mod utility;

use clap::{CommandFactory, Parser};
use clap_complete::generate;

pub async fn run() {
    let mut _cli = cli::App::parse();

    match _cli.command {
        cli::Command::Completion { shell } => {
            generate_completion(shell);
        }

        _ => {
            for word in utility::run_command(_cli.command, _cli.sudo).await {
                match word {
                    Ok(s) => println!("{}", s),
                    Err(e) => println!("{}", e),
                }
            }
        }
    }
}

fn generate_completion(shell: clap_complete::Shell) {
    let mut cmd = cli::App::command();
    let cmd_name: String = cmd.get_name().into();
    generate(shell, &mut cmd, cmd_name, &mut std::io::stdout());
}
