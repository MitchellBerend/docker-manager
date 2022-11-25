use futures::{stream, StreamExt};

use crate::constants;

use crate::cli::Command;
use crate::client::{Client, Node, NodeError};
use crate::utility::find_container;

pub async fn run_command(
    command: Command,
    sudo: bool,
    regex: Option<String>,
) -> Vec<Result<String, CommandError>> {
    let config_path = format!(
        "{}/.ssh/config",
        std::env::var("HOME").unwrap_or_else(|_| "/home/root".into())
    );
    let client = Client::from_config(config_path, regex);

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
            let node_containers: Vec<(String, String)> =
                find_container(client, &container_id, sudo).await;

            match node_containers.len() {
                0 => {
                    vec![Err(CommandError::NoNodesFound(container_id))]
                }
                1 => {
                    // unwrap is safe here since we .unwrap()check if there is exactly 1 element
                    let node_tuple = node_containers.get(0).unwrap().to_owned();
                    let node = Node::new(node_tuple.1);
                    match node
                        .run_command(
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
                            },
                            sudo,
                        )
                        .await
                    {
                        Ok(s) => vec![Ok(s)],
                        Err(e) => vec![Err(CommandError::NodeError(e))],
                    }
                }
                _ => {
                    let nodes = node_containers
                        .iter()
                        .map(|(_, result)| result.clone())
                        .collect::<Vec<String>>();
                    vec![Err(CommandError::MutlipleNodesFound(nodes))]
                }
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
            let bodies = stream::iter(client.nodes_info())
                .map(|(_, node)| async {
                    let _filter: Option<String> =
                        filter.as_ref().map(|filter| String::from(filter));
                    let _format: Option<String> =
                        format.as_ref().map(|format| String::from(format));
                    match node
                        .run_command(
                            Command::Images {
                                all,
                                digest,
                                filter: _filter,
                                format: _format,
                                no_trunc,
                                quiet,
                            },
                            sudo,
                        )
                        .await
                    {
                        Ok(result) => Ok(result),
                        Err(e) => Err(CommandError::NodeError(e)),
                    }
                })
                .buffer_unordered(constants::CONCURRENT_REQUESTS);
            bodies.collect::<Vec<Result<String, CommandError>>>().await
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
            let node_containers: Vec<(String, String)> =
                find_container(client, &container_id, sudo).await;
            match node_containers.len() {
                0 => {
                    vec![Err(CommandError::NoNodesFound(container_id))]
                }
                1 => {
                    // unwrap is safe here since we check if there is exactly 1 element
                    let node_tuple = node_containers.get(0).unwrap().to_owned();
                    let node = Node::new(node_tuple.1);
                    match node
                        .run_command(
                            Command::Logs {
                                container_id,
                                details,
                                follow,
                                since,
                                tail,
                                timestamps,
                                until,
                            },
                            sudo,
                        )
                        .await
                    {
                        Ok(s) => vec![Ok(s)],
                        Err(e) => vec![Err(CommandError::NodeError(e))],
                    }
                }
                _ => {
                    let nodes = node_containers
                        .iter()
                        .map(|(_, result)| result.clone())
                        .collect::<Vec<String>>();
                    vec![Err(CommandError::MutlipleNodesFound(nodes))]
                }
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
            let bodies = stream::iter(client.nodes_info())
                .map(|(_, node)| async {
                    let _filter: Option<String> =
                        filter.as_ref().map(|filter| String::from(filter));
                    let _format: Option<String> =
                        format.as_ref().map(|format| String::from(format));
                    match node
                        .run_command(
                            Command::Ps {
                                all,
                                filter: _filter,
                                format: _format,
                                last,
                                latests,
                                no_trunc,
                                quiet,
                                size,
                            },
                            sudo,
                        )
                        .await
                    {
                        Ok(result) => Ok(result),
                        Err(e) => Err(CommandError::NodeError(e)),
                    }
                })
                .buffer_unordered(constants::CONCURRENT_REQUESTS);
            bodies.collect::<Vec<Result<String, CommandError>>>().await
        }
        Command::Stop { container_id } => {
            let node_containers: Vec<(String, String)> =
                find_container(client, &container_id, sudo).await;

            match node_containers.len() {
                0 => {
                    vec![Err(CommandError::NoNodesFound(container_id))]
                }
                1 => {
                    // unwrap is safe here since we .unwrap()check if there is exactly 1 element
                    let node_tuple = node_containers.get(0).unwrap().to_owned();
                    let node = Node::new(node_tuple.1);
                    match node.run_command(Command::Stop { container_id }, sudo).await {
                        Ok(s) => vec![Ok(s)],
                        Err(e) => vec![Err(CommandError::NodeError(e))],
                    }
                }
                _ => {
                    let nodes = node_containers
                        .iter()
                        .map(|(_, result)| result.clone())
                        .collect::<Vec<String>>();
                    vec![Err(CommandError::MutlipleNodesFound(nodes))]
                }
            }
        }
    }
}

pub enum CommandError {
    NoNodesFound(String),
    MutlipleNodesFound(Vec<String>),
    NodeError(NodeError),
}

impl From<NodeError> for CommandError {
    fn from(node_error: NodeError) -> Self {
        Self::NodeError(node_error)
    }
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::NoNodesFound(container_id) => write!(
                f,
                "No node found containing the following container: {}",
                container_id
            ),
            Self::MutlipleNodesFound(nodes) => write!(
                f,
                "Multiple nodes found with matching criteria:\n{:#?}",
                nodes
            ),
            Self::NodeError(node_error) => write!(f, "{}", node_error),
        }
    }
}

#[cfg(test)]
mod test {
    use super::CommandError;

    #[test]
    fn test_command_error_multiple_found_diplay() {
        let error = CommandError::MutlipleNodesFound(vec!["abc".into(), "def".into()]);

        let correct_string: String =
            "Multiple nodes found with matching criteria:\n[\n    \"abc\",\n    \"def\",\n]".into();

        assert_eq!(correct_string, format!("{}", error));
    }

    #[test]
    fn test_command_error_no_node_found_diplay() {
        let error = CommandError::NoNodesFound("some_container_id".into());

        let correct_string: String =
            "No node found containing the following container: some_container_id".into();

        assert_eq!(correct_string, format!("{}", error));
    }
}
