use clap::Command;
use clap_complete::generate;

use crate::cli::flags::{LogsFlags, PsFlags};

pub fn run_completion(cmd: &mut Command) {
    generate(
        clap_complete::Shell::Bash,
        cmd,
        cmd.get_name().to_string(),
        &mut std::io::stdout(),
    );
}

pub async fn run_ps(
    hostname: String,
    session: openssh::Session,
    flags: PsFlags,
) -> Result<String, openssh::Error> {
    let mut command: Vec<String> = vec!["docker".into(), "ps".into()];

    for flag in flags.flags() {
        command.push(flag)
    }

    let output = match session.command("sudo").args(command).output().await {
        Ok(output) => output,
        Err(e) => return Err(e),
    };
    let mut rv: String = format!("{}\n", hostname);
    match output.status.code().unwrap() {
        0 => rv.push_str(std::str::from_utf8(&output.stdout).unwrap_or("")),
        _ => rv.push_str(std::str::from_utf8(&output.stderr).unwrap_or("")),
    };

    Ok(rv)
}

pub async fn run_stop(
    hostname: String,
    session: openssh::Session,
    container_id: String,
) -> Result<String, openssh::Error> {
    let command = vec!["docker", "stop", &container_id];

    let output = match session.command("sudo").args(command).output().await {
        Ok(output) => output,
        Err(e) => return Err(e),
    };

    let mut rv: String = format!("{}\n", hostname);
    match output.status.code().unwrap() {
        0 => rv.push_str(std::str::from_utf8(&output.stdout).unwrap_or("")),
        _ => rv.push_str(std::str::from_utf8(&output.stderr).unwrap_or("")),
    };

    Ok(rv)
}

pub async fn run_logs(
    hostname: String,
    session: openssh::Session,
    container_id: String,
    flags: LogsFlags,
) -> Result<String, openssh::Error> {
    let mut command: Vec<String> = vec!["docker".into(), "logs".into()];

    for item in flags.flags() {
        command.push(item)
    }

    if flags.follow {
        command.push("-f".into());
        command.push(container_id);

        let mut output = match session.command("sudo").args(command).spawn().await {
            Ok(output) => output,
            Err(e) => return Err(e),
        };
        loop {
            if let Some(stdout) = output.stdout().take() {
                println!("{:?}", stdout)
            };
        }
    } else {
        command.push(container_id);

        let output = match session.command("sudo").args(command).output().await {
            Ok(output) => output,
            Err(e) => return Err(e),
        };

        let mut rv: String = format!("{}\n", hostname);
        match output.status.code().unwrap() {
            0 => rv.push_str(std::str::from_utf8(&output.stdout).unwrap_or("")),
            _ => rv.push_str(std::str::from_utf8(&output.stderr).unwrap_or("")),
        };
        Ok(rv)
    }
}
