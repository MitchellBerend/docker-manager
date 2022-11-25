use crate::cli::flags::{ExecFlags, ImagesFlags, LogsFlags, PsFlags};

pub async fn run_exec(
    hostname: &str,
    session: openssh::Session,
    container_id: String,
    sudo: bool,
    command: Vec<String>,
    //args: Option<Vec<String>>,
    flags: ExecFlags,
) -> Result<String, openssh::Error> {
    let mut _command: Vec<&str> = vec!["exec"];

    for flag in &flags.flags() {
        _command.push(flag);
    }
    _command.push(&container_id);

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
    flags: ImagesFlags,
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
    container_id: String,
    sudo: bool,
    flags: LogsFlags,
) -> Result<String, openssh::Error> {
    let mut command: Vec<&str> = vec!["logs"];

    for item in flags.flags() {
        command.push(item)
    }

    if flags.follow {
        command.push("-f".into());
        command.push(&container_id);

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
        command.push(&container_id);
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
    flags: PsFlags,
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

pub async fn run_stop(
    hostname: &str,
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
