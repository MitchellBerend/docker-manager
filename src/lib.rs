pub mod constants {
    pub const CONCURRENT_REQUESTS: usize = 10;
}

mod cli;
mod client;
//mod formatter;
mod utility;

use clap::{CommandFactory, Parser};
use clap_complete::generate;

// This lint option should be allowed because it is the current way to print results to stdout.
#[allow(clippy::print_stdout)]
pub async fn run() {
    let mut _cli = cli::App::parse();

    let regex: Option<&str> = match &_cli.regex {
        Some(reg) => Some(reg),
        None => None,
    };

    match _cli.command {
        cli::Command::Completion { shell } => {
            generate_completion(shell);
        }
        _ => {
            for word in utility::run_command(_cli.command, _cli.sudo, regex).await {
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
