use crate::cli::flags::{ExecFlags, ImagesFlags, LogsFlags, PsFlags, RmFlags};
use crate::cli::{System, SystemCommand};

pub async fn run_exec(
    hostname: &str,
    session: openssh::Session,
    container_id: &str,
    sudo: bool,
    command: Vec<&str>,
    //args: Option<Vec<String>>,
    flags: ExecFlags<'_>,
) -> Result<String, openssh::Error> {
    let mut _command: Vec<&str> = vec!["exec"];

    for flag in &flags.flags() {
        _command.push(flag);
    }
    _command.push(container_id);

    for arg in &command {
        _command.push(arg);
    }

    if flags.interactive {
        // This needs to be mutable so the stdout can be written to
        #[allow(unused_mut)]
        let mut _output = match sudo {
            true => {
                session
                    .command("sudo")
                    .arg("docker")
                    .args(_command)
                    .spawn()
                    .await?
            }
            false => session.command("docker").args(_command).spawn().await?,
        };

        loop {
            std::thread::sleep(std::time::Duration::new(1, 0));
        }
    } else {
        let _output = match sudo {
            true => {
                session
                    .command("sudo")
                    .arg("docker")
                    .args(_command)
                    .output()
                    .await
            }
            false => session.command("docker").args(_command).output().await,
        };

        let output = match _output {
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

pub async fn run_images(
    hostname: &str,
    session: openssh::Session,
    sudo: bool,
    flags: ImagesFlags<'_>,
) -> Result<String, openssh::Error> {
    let mut command: Vec<&str> = vec!["images"];

    for flag in flags.flags() {
        command.push(flag)
    }

    let _output = match sudo {
        true => {
            session
                .command("sudo")
                .arg("docker")
                .args(command)
                .output()
                .await
        }
        false => session.command("docker").args(command).output().await,
    };

    let output = match _output {
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
    hostname: &str,
    session: openssh::Session,
    container_id: &str,
    sudo: bool,
    flags: LogsFlags<'_>,
) -> Result<String, openssh::Error> {
    let mut command: Vec<&str> = vec!["logs"];

    for item in flags.flags() {
        command.push(item)
    }

    if flags.follow {
        command.push("-f");
        command.push(container_id);

        // This needs to be mutable so the stdout can be written to
        #[allow(unused_mut)]
        let mut _output = match sudo {
            true => {
                session
                    .command("sudo")
                    .arg("docker")
                    .args(command)
                    .spawn()
                    .await?
            }
            false => session.command("docker").args(command).spawn().await?,
        };

        loop {
            std::thread::sleep(std::time::Duration::new(1, 0));
        }
    } else {
        command.push(container_id);
        let _output = match sudo {
            true => {
                session
                    .command("sudo")
                    .arg("docker")
                    .args(command)
                    .output()
                    .await
            }
            false => session.command("docker").args(command).output().await,
        };

        let output = match _output {
            Ok(output) => output,
            Err(e) => return Err(e),
        };

        let mut rv: String = format!("{}\n", hostname);

        // docker logs prints the logs to standard error so it does not matter if the command
        // succeeded or not
        rv.push_str(std::str::from_utf8(&output.stderr).unwrap_or(""));

        Ok(rv)
    }
}

pub async fn run_ps(
    hostname: &str,
    session: openssh::Session,
    sudo: bool,
    flags: PsFlags<'_>,
) -> Result<String, openssh::Error> {
    let mut command: Vec<&str> = vec!["ps"];

    for flag in flags.flags() {
        command.push(flag)
    }

    let _output = match sudo {
        true => {
            session
                .command("sudo")
                .arg("docker")
                .args(command)
                .output()
                .await
        }
        false => session.command("docker").args(command).output().await,
    };

    let output = match _output {
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

pub async fn run_restart(
    hostname: &str,
    session: openssh::Session,
    sudo: bool,
    time: Option<&str>,
    container_id: &[&str],
) -> Result<String, openssh::Error> {
    let mut command = vec!["restart"];

    for container in container_id {
        command.push(container);
    }

    if let Some(time) = &time {
        command.push("--time");
        command.push(time.as_ref());
    } else {
        command.push("--time");
        command.push("10");
    }

    let _output = match sudo {
        true => {
            session
                .command("sudo")
                .arg("docker")
                .args(command)
                .output()
                .await
        }
        false => session.command("docker").args(command).output().await,
    };

    let output = match _output {
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

pub async fn run_rm(
    hostname: &str,
    session: openssh::Session,
    sudo: bool,
    container_id: &Vec<&str>,
    flags: RmFlags,
) -> Result<String, openssh::Error> {
    let mut command = vec!["rm"];

    for container in container_id {
        command.push(container);
    }

    for flag in flags.flags() {
        command.push(flag)
    }

    let _output = match sudo {
        true => {
            session
                .command("sudo")
                .arg("docker")
                .args(command)
                .output()
                .await
        }
        false => session.command("docker").args(command).output().await,
    };

    let output = match _output {
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

pub async fn run_start(
    hostname: &str,
    session: openssh::Session,
    sudo: bool,
    container_id: &Vec<&str>,
    attatch: bool,
) -> Result<String, openssh::Error> {
    let mut command = vec!["start"];
    for container in container_id {
        command.push(container);
    }

    if attatch {
        command.push("-a");
    };

    let _output = match sudo {
        true => {
            session
                .command("sudo")
                .arg("docker")
                .args(command)
                .output()
                .await
        }
        false => session.command("docker").args(command).output().await,
    };

    let output = match _output {
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
    hostname: &str,
    session: openssh::Session,
    sudo: bool,
    container_id: &Vec<&str>,
) -> Result<String, openssh::Error> {
    let mut command = vec!["stop"];
    for container in container_id {
        command.push(container);
    }

    let _output = match sudo {
        true => {
            session
                .command("sudo")
                .arg("docker")
                .args(command)
                .output()
                .await
        }
        false => session.command("docker").args(command).output().await,
    };

    let output = match _output {
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

pub async fn run_system(
    hostname: &str,
    session: openssh::Session,
    sudo: bool,
    command: System,
) -> Result<String, openssh::Error> {
    let mut _command: Vec<&str> = vec!["system"];

    match command.command {
        SystemCommand::Df {
            ref format,
            verbose,
        } => {
            _command.push("df");
            if let Some(format) = format {
                _command.push(format);
            }
            if verbose {
                _command.push("-v");
            }
        }
        SystemCommand::Events {
            ref filter,
            ref format,
            ref since,
            ref until,
        } => {
            _command.push("events");

            if let Some(filter) = filter {
                _command.push("--filter");
                _command.push(filter);
            }
            if let Some(format) = format {
                _command.push("--format");
                _command.push(format);
            }
            if let Some(since) = since {
                _command.push("--since");
                _command.push(since);
            }
            if let Some(until) = until {
                _command.push("--until");
                _command.push(until);
            }
        }
        SystemCommand::Info { ref format } => {
            _command.push("info");
            if let Some(format) = format {
                _command.push("--format");
                _command.push(format);
            }
        }
        SystemCommand::Prune {
            all,
            ref filter,
            force,
            volumes,
        } => {
            _command.push("prune");

            if all {
                _command.push("--all");
            }
            if let Some(filter) = filter {
                _command.push("--filter");
                _command.push(filter);
            }
            if force {
                _command.push("--force");
            }
            if volumes {
                _command.push("--volumes");
            }
        }
    }

    let _output = match sudo {
        true => {
            session
                .command("sudo")
                .arg("docker")
                .args(_command)
                .output()
                .await
        }
        false => session.command("docker").args(_command).output().await,
    };

    let output = match _output {
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
