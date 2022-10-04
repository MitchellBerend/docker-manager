mod cli;
mod client;
mod utility;

use clap::Parser;

pub async fn run() {
    let _cli = cli::App::parse();

    for word in utility::run_command(_cli.command).await {
        match word {
            Ok(s) => println!("{}", s),
            Err(e) => println!("{}", e),
        }
    }
}
