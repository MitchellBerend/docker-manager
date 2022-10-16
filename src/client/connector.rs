use std::path::Path;

use crate::cli::flags::{ExecFlags, ImagesFlags, LogsFlags, PsFlags};
use crate::cli::Command;
use crate::utility::command;

#[derive(Debug)]
pub struct Client {
    nodes: Vec<Node>,
}

impl Client {
    pub fn from_config<A: AsRef<Path>>(config_path: A) -> Self {
        let file_contents = std::fs::read_to_string(config_path).unwrap_or_else(|_| "".into());

        let mut nodes: Vec<Node> = vec![];
        for line in file_contents.split('\n') {
            if !line.contains('#') && line.starts_with("Host") {
                let s: String = line.replace("  ", "").split(' ').nth(1).unwrap().into();
                nodes.push(Node::new(s))
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

    pub async fn run_command(&self, command: Command, sudo: bool) -> Result<String, NodeError> {
        let session = match openssh::SessionBuilder::default()
            .connect_timeout(std::time::Duration::new(1, 0))
            .connect_mux(&self.address)
            .await
        {
            Ok(session) => session,
            Err(e) => return Err(NodeError::SessionError(self.address.clone(), e)),
        };

        match command {
            Command::Completion { shell: _ } => {
                // This command should not lead to any activity
                unreachable!()
            }
            Command::Exec {
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
                    detach_keys,
                    env,
                    env_file,
                    interactive,
                    privileged,
                    user,
                    workdir,
                );
                match command::run_exec(
                    self.address.clone(),
                    session,
                    container_id,
                    sudo,
                    command,
                    flags,
                )
                .await
                {
                    Ok(result) => Ok(result),
                    Err(e) => Err(NodeError::SessionError(self.address.clone(), e)),
                }
            }
            Command::Images {
                all,
                digest,
                filter,
                format,
                no_trunc,
                quiet,
            } => {
                let flags = ImagesFlags::new(all, digest, filter, format, no_trunc, quiet);

                match command::run_images(self.address.clone(), session, sudo, flags).await {
                    Ok(result) => Ok(result),
                    Err(e) => Err(NodeError::SessionError(self.address.clone(), e)),
                }
            }
            Command::Logs {
                container_id,
                details,
                follow,
                since,
                tail,
                timestamps,
                until,
            } => {
                let flags = LogsFlags::new(details, follow, since, tail, timestamps, until);

                match command::run_logs(self.address.clone(), session, container_id, sudo, flags)
                    .await
                {
                    //, follow).await {
                    Ok(result) => Ok(result),
                    Err(e) => Err(NodeError::SessionError(self.address.clone(), e)),
                }
            }
            Command::Ps {
                all,
                filter,
                format,
                last,
                latests,
                no_trunc,
                quiet,
                size,
            } => {
                let flags = PsFlags::new(all, filter, format, last, latests, no_trunc, quiet, size);
                match command::run_ps(self.address.clone(), session, sudo, flags).await {
                    Ok(result) => Ok(result),
                    Err(e) => Err(NodeError::SessionError(self.address.clone(), e)),
                }
            }
            Command::Stop { container_id } => {
                match command::run_stop(self.address.clone(), session, sudo, container_id).await {
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
        let client = Client::from_config("test_files/mock_ssh_config");
        let correct_nodes: Vec<String> = vec!["abc".into(), "def".into(), "ghi".into()];

        let nodes: Vec<String> = client
            .nodes
            .iter()
            .map(|node| node.address.clone())
            .collect();

        assert_eq!(correct_nodes, nodes);
    }

    #[test]
    fn test_client_info() {
        let client = Client::from_config("test_files/mock_ssh_config");
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
