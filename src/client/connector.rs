use std::path::Path;

use crate::cli::flags::{ExecFlags, ImagesFlags, LogsFlags, PsFlags, RmFlags};
use crate::cli::InternalCommand;
use crate::utility::command;

use regex::Regex;

#[derive(Debug)]
pub struct Client {
    nodes: Vec<Node>,
}

impl Client {
    pub fn from_config<A: AsRef<Path>>(config_path: A, regex: Option<&str>) -> Self {
        let file_contents = std::fs::read_to_string(config_path).unwrap_or_else(|_| "".into());

        let _re = match regex {
            Some(pattern) => Regex::new(pattern),
            None => Regex::new(".*"),
        };

        let re = match _re {
            Ok(regex) => regex,
            Err(e) => {
                eprintln!(
                    "Some error has occured while compiling your regex patterns {}\n{}",
                    regex.unwrap(),
                    e
                );
                std::process::exit(1)
            }
        };

        let mut nodes: Vec<Node> = vec![];
        for line in file_contents.split('\n') {
            if !line.contains('#') && line.starts_with("Host") {
                let s: String = line.replace("  ", "").split(' ').nth(1).unwrap().into();
                if re.is_match(&s) {
                    nodes.push(Node::new(s));
                }
            }
        }

        Self { nodes }
    }

    pub fn nodes_info(&self) -> Vec<(&String, &Node)> {
        self.nodes
            .iter()
            .map(|node| (&node.address, node))
            .collect()
    }
}

#[derive(Debug)]
pub struct Node {
    address: String,
}

impl Node {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub async fn run_command(
        &self,
        command: InternalCommand<'_>,
        sudo: bool,
        identity_file: Option<&str>,
    ) -> Result<String, NodeError> {
        let mut builder = openssh::SessionBuilder::default();
        builder.connect_timeout(std::time::Duration::new(1, 0));

        if let Some(id_file) = identity_file {
            builder.keyfile(id_file);
        };

        let session = match builder.connect_mux(&self.address).await {
            Ok(session) => session,
            Err(e) => return Err(NodeError::SessionError(self.address.clone(), e)),
        };

        match command {
            InternalCommand::Exec {
                container_id,
                command,
                detach,
                detach_keys,
                env,
                env_file,
                interactive,
                privileged,
                user,
                workdir,
            } => {
                let flags = ExecFlags::new(
                    detach,
                    &detach_keys,
                    env,
                    env_file,
                    interactive,
                    privileged,
                    &user,
                    &workdir,
                );
                match command::run_exec(&self.address, session, container_id, sudo, command, flags)
                    .await
                {
                    Ok(result) => Ok(result),
                    Err(e) => Err(NodeError::SessionError(self.address.clone(), e)),
                }
            }
            InternalCommand::Images {
                all,
                digest,
                filter,
                format,
                no_trunc,
                quiet,
            } => {
                let flags = ImagesFlags::new(all, digest, &filter, &format, no_trunc, quiet);

                match command::run_images(&self.address, session, sudo, flags).await {
                    Ok(result) => Ok(result),
                    Err(e) => Err(NodeError::SessionError(self.address.clone(), e)),
                }
            }
            InternalCommand::Logs {
                container_id,
                details,
                follow,
                since,
                tail,
                timestamps,
                until,
            } => {
                let flags = LogsFlags::new(details, follow, &since, &tail, timestamps, &until);

                match command::run_logs(&self.address, session, container_id, sudo, flags).await {
                    //, follow).await {
                    Ok(result) => Ok(result),
                    Err(e) => Err(NodeError::SessionError(self.address.clone(), e)),
                }
            }
            InternalCommand::Ps {
                all,
                filter,
                format,
                last,
                latests,
                no_trunc,
                quiet,
                size,
            } => {
                let flags =
                    PsFlags::new(all, &filter, &format, last, latests, no_trunc, quiet, size);
                match command::run_ps(&self.address, session, sudo, flags).await {
                    Ok(result) => Ok(result),
                    Err(e) => Err(NodeError::SessionError(self.address.clone(), e)),
                }
            }
            InternalCommand::Restart { time, container_id } => {
                match command::run_restart(&self.address, session, sudo, time, &container_id).await
                {
                    Ok(result) => Ok(result),
                    Err(e) => Err(NodeError::SessionError(self.address.clone(), e)),
                }
            }
            InternalCommand::Rm {
                container_id,
                force,
                volumes,
            } => {
                let flags = RmFlags::new(force, volumes);
                match command::run_rm(&self.address, session, sudo, &container_id, flags).await {
                    Ok(result) => Ok(result),
                    Err(e) => Err(NodeError::SessionError(self.address.clone(), e)),
                }
            }
            InternalCommand::Start {
                container_id,
                attach,
            } => {
                match command::run_start(&self.address, session, sudo, &container_id, attach).await
                {
                    Ok(result) => Ok(result),
                    Err(e) => Err(NodeError::SessionError(self.address.clone(), e)),
                }
            }
            InternalCommand::Stop { container_id } => {
                match command::run_stop(&self.address, session, sudo, &container_id).await {
                    Ok(result) => Ok(result),
                    Err(e) => Err(NodeError::SessionError(self.address.clone(), e)),
                }
            }
            InternalCommand::System(command) => {
                match command::run_system(&self.address, session, sudo, command).await {
                    Ok(result) => Ok(result),
                    Err(e) => Err(NodeError::SessionError(self.address.clone(), e)),
                }
            }
        }
    }
}

pub enum NodeError {
    SessionError(String, openssh::Error),
}

impl std::fmt::Display for NodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::SessionError(hostname, e) => write!(f, "[NodeError] {}: {}", hostname, e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Client, Node};

    #[test]
    fn test_from_config() {
        let client = Client::from_config("test_files/mock_ssh_config", Some(".*"));
        let correct_nodes: Vec<String> = vec!["abc".into(), "def".into(), "ghi".into()];

        let nodes: Vec<String> = client
            .nodes
            .iter()
            .map(|node| node.address.clone())
            .collect();

        assert_eq!(correct_nodes, nodes);
    }

    #[test]
    fn test_from_config_regex() {
        let client =
            Client::from_config("test_files/mock_ssh_config_regex", Some("regex_pattern.*"));
        let correct_nodes: Vec<String> = vec![
            "regex_pattern".into(),
            "regex_patterndef".into(),
            "regex_patternghi".into(),
        ];

        let nodes: Vec<String> = client
            .nodes
            .iter()
            .map(|node| node.address.clone())
            .collect();

        assert_eq!(correct_nodes, nodes);
    }

    #[test]
    fn test_client_info() {
        let client = Client::from_config("test_files/mock_ssh_config", Some(".*"));
        let correct_nodes: Vec<(String, Node)> = vec![
            (String::from("abc"), Node::new("abc".into())),
            (String::from("def"), Node::new("def".into())),
            (String::from("ghi"), Node::new("ghi".into())),
        ];
        let nodes: Vec<(&String, &Node)> = client
            .nodes_info()
            .iter()
            .map(|(hostname, node)| (*hostname, *node))
            .collect();

        let mut index = 0;
        for (hostname, node) in nodes {
            let correct_node = &correct_nodes.get(index).unwrap();
            let correct_hostname = &correct_node.0;
            let correct_node_ref = &correct_node.1;
            assert_eq!(correct_hostname, hostname);
            assert_eq!(correct_node_ref.address, node.address);
            index += 1;
        }
    }
}
