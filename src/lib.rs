use std::io::Write;

pub mod constants {
    pub const CONCURRENT_REQUESTS: usize = 10;
}

mod cli;
mod client;
mod formatter;
mod utility;

use clap::{CommandFactory, Parser};
use clap_complete::generate;

pub async fn run() {
    let mut _cli = cli::App::parse();

    let regex: Option<&str> = match &_cli.regex {
        Some(reg) => Some(reg),
        None => None,
    };

    let mut result: String = String::new();
    let mut parse: bool = false;

    match _cli.command {
        cli::Command::Completion { shell } => {
            generate_completion(shell);
        }
        _ => {
            parse = true;
            for word in utility::run_command(_cli.command, _cli.sudo, regex).await {
                match word {
                    Ok(s) => result.push_str(&s),
                    Err(e) => {
                        writeln!(std::io::stdout(), "{}", e).unwrap();
                    }
                }
            }
        }
    }

    let mut parser = crate::formatter::Parser::from_command_results(result);

    if parse {
        parser.print();
    }
}

fn generate_completion(shell: clap_complete::Shell) {
    let mut cmd = cli::App::command();
    let cmd_name: String = cmd.get_name().into();
    generate(shell, &mut cmd, cmd_name, &mut std::io::stdout());
}
