use crate::cli::flags::{ExecFlags, ImagesFlags, LogsFlags, PsFlags};

pub async fn run_exec(
    hostname: String,
    session: openssh::Session,
    container_id: String,
    sudo: bool,
    command: Vec<String>,
    //args: Option<Vec<String>>,
    flags: ExecFlags,
) -> Result<String, openssh::Error> {
    let mut _command: Vec<String> = vec!["exec".into()];

    for flag in &flags.flags() {
        _command.push(flag.clone());
    }
    _command.push(container_id);

    for arg in command {
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
    hostname: String,
    session: openssh::Session,
    sudo: bool,
    flags: ImagesFlags,
) -> Result<String, openssh::Error> {
    let mut command: Vec<String> = vec!["images".into()];

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
    hostname: String,
    session: openssh::Session,
    container_id: String,
    sudo: bool,
    flags: LogsFlags,
) -> Result<String, openssh::Error> {
    let mut command: Vec<String> = vec!["logs".into()];

    for item in flags.flags() {
        command.push(item)
    }

    if flags.follow {
        command.push("-f".into());
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
    hostname: String,
    session: openssh::Session,
    sudo: bool,
    flags: PsFlags,
) -> Result<String, openssh::Error> {
    let mut command: Vec<String> = vec!["ps".into()];

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

pub async fn run_stop(
    hostname: String,
    session: openssh::Session,
    sudo: bool,
    container_id: String,
) -> Result<String, openssh::Error> {
    let command = vec!["stop", &container_id];

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
