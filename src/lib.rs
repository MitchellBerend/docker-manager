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
    let mut command: String = String::new();

    match _cli.command {
        cli::Command::Completion { shell } => {
            generate_completion(shell);
        }
        _ => {
            command = _cli.command.to_string();
            match &_cli.command {
                cli::Command::Ps { .. } => parse = true,
                cli::Command::Images { .. } => parse = true,
                _ => (),
            }
            for word in utility::run_command(_cli.command, _cli.sudo, regex).await {
                match word {
                    Ok(s) => result.push_str(&s),
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
        }
    }

    if parse {
        let mut parser: Option<formatter::Parser> = None;

        match command.as_str() {
            "Ps" => parser = Some(formatter::Parser::from_ps_results(&result)),
            "Images" => parser = Some(formatter::Parser::from_images_results(&result)),

            _ => (),
        }

        if let Some(mut parser) = parser {
            parser.print();
        }
    } else {
        println!("{}", result);
    }
}

fn generate_completion(shell: clap_complete::Shell) {
    let mut cmd = cli::App::command();
    let cmd_name: String = cmd.get_name().into();
    generate(shell, &mut cmd, cmd_name, &mut std::io::stdout());
}
