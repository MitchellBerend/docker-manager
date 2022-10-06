pub async fn run_ps(
    hostname: String,
    session: openssh::Session,
    all: bool,
) -> Result<String, openssh::Error> {
    let mut command = vec!["docker", "ps"];
    if all {
        command.push("-a");
    };

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

pub struct LogsFlags {
    pub details: bool,
    pub follow: bool,
    pub since: Option<String>,
    pub tail: Option<String>,
    pub timestamps: bool,
    pub until: Option<String>,
}

pub async fn run_logs(
    hostname: String,
    session: openssh::Session,
    container_id: String,
    flags: LogsFlags,
    //details: bool,
    //follow: bool,
    //since: Option<String>,
    //tail: Option<String>,
    //timestamps: bool,
    //until: Option<String>,
) -> Result<String, openssh::Error> {
    let mut command: Vec<String> = vec!["docker".into(), "logs".into()];

    if flags.details {
        command.push("--details".into());
    }

    if flags.timestamps {
        command.push("--timestamps".into());
    }

    if let Some(s) = flags.since {
        command.push("--since".into());
        command.push(s);
    }
    if let Some(s) = flags.tail {
        command.push("--tail".into());
        command.push(s);
    }
    if let Some(s) = flags.until {
        command.push("--until".into());
        command.push(s);
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
