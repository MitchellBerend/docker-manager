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
        0 => rv.push_str(std::str::from_utf8(&output.stdout).unwrap_or("").into()),
        _ => rv.push_str(std::str::from_utf8(&output.stderr).unwrap_or("").into()),
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
        0 => rv.push_str(std::str::from_utf8(&output.stdout).unwrap_or("").into()),
        _ => rv.push_str(std::str::from_utf8(&output.stderr).unwrap_or("").into()),
    };

    Ok(rv)
}
