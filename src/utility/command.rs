use crate::cli::flags::{ExecFlags, LogsFlags, PsFlags};

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

        // This needs to be mutable so the stdout can be written to
        #[allow(unused_mut)]
        let mut _output = session.command("sudo").args(command).spawn().await?;

        loop {
            std::thread::sleep(std::time::Duration::new(1, 0));
        }
    } else {
        command.push(container_id);

        let output = match session.command("sudo").args(command).output().await {
            Ok(output) => output,
            Err(e) => return Err(e),
        };

        let mut rv: String = format!("{}\n", hostname);

        println!("o;{}", std::str::from_utf8(&output.stdout).unwrap_or(""));
        println!("e;{}", std::str::from_utf8(&output.stderr).unwrap_or(""));
        match output.status.code().unwrap() {
            0 => rv.push_str(std::str::from_utf8(&output.stdout).unwrap_or("")),
            _ => rv.push_str(std::str::from_utf8(&output.stderr).unwrap_or("")),
        };
        Ok(rv)
    }
}

pub async fn run_exec(
    hostname: String,
    session: openssh::Session,
    container_id: String,
    command: Vec<String>,
    //args: Option<Vec<String>>,
    flags: ExecFlags,
) -> Result<String, openssh::Error> {
    let mut _command: Vec<String> = vec!["docker".into(), "exec".into()];

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
        let mut _output = session.command("sudo").args(_command).spawn().await?;

        loop {
            std::thread::sleep(std::time::Duration::new(1, 0));
        }
    } else {
        let output = match session.command("sudo").args(_command).output().await {
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
