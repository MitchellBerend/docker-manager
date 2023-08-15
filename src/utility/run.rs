use futures::{stream, StreamExt};
use crate::constants;

use crate::cli::InternalCommand;
use crate::client::{Client, Node, NodeError};
use crate::utility::find_containers;

use super::other::Container;

pub async fn run_command<'a>(
    command: InternalCommand<'a>,
    sudo: bool,
    regex: Option<&str>,
    identity_file: Option<&str>,
) -> Vec<Result<String, CommandError<'a>>> {
    let config_path = format!(
        "{}/.ssh/config",
        std::env::var("HOME").unwrap_or_else(|_| "/home/root".into())
    );
    let client = Client::from_config(config_path, regex);

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
            let node_containers: Vec<Container> =
                find_containers(client, &[container_id], sudo, false, identity_file).await;

            match node_containers.len() {
                0 => {
                    vec![Err(CommandError::NoNodesFound(container_id))]
                }
                1 => {
                    // unwrap is safe here since we .unwrap()check if there is exactly 1 element
                    let node_tuple = node_containers.get(0).unwrap().to_owned();
                    let node = Node::new(node_tuple.hostname().to_string());
                    match node
                        .run_command(
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
                            },
                            sudo,
                            identity_file,
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
                        .map(|result| result.id().to_string())
                        .collect::<Vec<String>>();
                    vec![Err(CommandError::MutlipleNodesFound(nodes))]
                }
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
            let bodies = stream::iter(client.nodes_info())
                .map(|(_, node)| async {
                    // let _filter: Option<String> = filter.map(String::from);
                    // let _format: Option<String> = format.map(String::from);
                    match node
                        .run_command(
                            InternalCommand::Images {
                                all,
                                digest,
                                // filter: _filter,
                                // format: _format,
                                filter,
                                format,
                                no_trunc,
                                quiet,
                            },
                            sudo,
                            identity_file,
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
        InternalCommand::Logs {
            container_id,
            details,
            follow,
            since,
            tail,
            timestamps,
            until,
        } => {
            let node_containers: Vec<Container> =
                find_containers(client, &[container_id], sudo, false, identity_file).await;
            match node_containers.len() {
                0 => {
                    vec![Err(CommandError::NoNodesFound(container_id))]
                }
                1 => {
                    // unwrap is safe here since we check if there is exactly 1 element
                    let node_tuple = node_containers.get(0).unwrap().to_owned();
                    let node = Node::new(node_tuple.hostname().to_string());
                    match node
                        .run_command(
                            InternalCommand::Logs {
                                container_id,
                                details,
                                follow,
                                since,
                                tail,
                                timestamps,
                                until,
                            },
                            sudo,
                            identity_file,
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
                        .map(|result| result.id().to_string())
                        .collect::<Vec<String>>();
                    vec![Err(CommandError::MutlipleNodesFound(nodes))]
                }
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
            let bodies = stream::iter(client.nodes_info())
                .map(|(_, node)| async {
                    // let _filter: Option<String> = filter.map(String::from);
                    // let _format: Option<String> = format.map(String::from);
                    match node
                        .run_command(
                            InternalCommand::Ps {
                                all,
                                // filter: _filter,
                                // format: _format,
                                filter,
                                format,
                                last,
                                latests,
                                no_trunc,
                                quiet,
                                size,
                            },
                            sudo,
                            identity_file,
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
        InternalCommand::Restart { time, container_id } => {
            let node_containers: Vec<Container> =
                find_containers(client, &container_id, sudo, true, identity_file).await;

            match node_containers.len() {
                0 => {
                    vec![Err(CommandError::NoMultipleNodesFound(container_id))]
                }
                1 => {
                    // unwrap is safe here since we .unwrap()check if there is exactly 1 element
                    let node_tuple = node_containers.get(0).unwrap().to_owned();
                    let node = Node::new(node_tuple.hostname().to_string());
                    match node
                        .run_command(
                            InternalCommand::Restart { time, container_id },
                            sudo,
                            identity_file,
                        )
                        .await
                    {
                        Ok(s) => vec![Ok(s)],
                        Err(e) => vec![Err(CommandError::NodeError(e))],
                    }
                }
                _ => {
                    let bodies = stream::iter(node_containers)
                        .map(|container| {
                            async move {
                                let node = Node::new(container.node().to_string());
                                match node
                                    .run_command(
                                        InternalCommand::Restart {
                                            time,
                                            container_id: vec![container.id()],
                                        },
                                        sudo,
                                        identity_file,
                                    )
                                    .await
                                {
                                    Ok(result) => (container.hostname().to_string(), Ok(result)),
                                    Err(e) => (container.hostname().to_string(), Err(e)),
                                }
                            }
                        })
                        .buffer_unordered(constants::CONCURRENT_REQUESTS);

                    let _rv = bodies
                        .collect::<Vec<(String, Result<String, NodeError>)>>()
                        .await;

                    let mut rv = vec![];

                    for (_, res) in _rv {
                        match res {
                            Ok(s) => rv.push(Ok(s)),
                            Err(e) => rv.push(Err(CommandError::NodeError(e))),
                        }
                    }
                    rv
                }
            }
        }
        InternalCommand::Rm {
            container_id,
            force,
            volumes,
        } => {
            let node_containers: Vec<Container> =
                find_containers(client, &container_id, sudo, true, identity_file).await;

            match node_containers.len() {
                0 => {
                    vec![Err(CommandError::NoMultipleNodesFound(container_id))]
                }
                1 => {
                    // unwrap is safe here since we .unwrap()check if there is exactly 1 element
                    let node_tuple = node_containers.get(0).unwrap().to_owned();
                    let node = Node::new(node_tuple.hostname().to_string());
                    match node
                        .run_command(
                            InternalCommand::Rm {
                                container_id,
                                force,
                                volumes,
                            },
                            sudo,
                            identity_file,
                        )
                        .await
                    {
                        Ok(s) => vec![Ok(s)],
                        Err(e) => vec![Err(CommandError::NodeError(e))],
                    }
                }
                _ => {
                    let bodies = stream::iter(node_containers)
                        .map(|container| async move {
                            let node = Node::new(container.node().to_string());
                            match node
                                .run_command(
                                    InternalCommand::Rm {
                                        container_id: vec![container.id()],
                                        force,
                                        volumes,
                                    },
                                    sudo,
                                    identity_file,
                                )
                                .await
                            {
                                Ok(result) => (container.hostname().to_string(), Ok(result)),
                                Err(e) => (container.hostname().to_string(), Err(e)),
                            }
                        })
                        .buffer_unordered(constants::CONCURRENT_REQUESTS);

                    let _rv = bodies
                        .collect::<Vec<(String, Result<String, NodeError>)>>()
                        .await;

                    let mut rv = vec![];

                    for (_, res) in _rv {
                        match res {
                            Ok(s) => rv.push(Ok(s)),
                            Err(e) => rv.push(Err(CommandError::NodeError(e))),
                        }
                    }
                    rv
                }
            }
        }
        InternalCommand::Start {
            container_id,
            attach,
        } => {
            let node_containers: Vec<Container> =
                find_containers(client, &container_id, sudo, true, identity_file).await;

            match node_containers.len() {
                0 => {
                    vec![Err(CommandError::NoMultipleNodesFound(container_id))]
                }
                1 => {
                    // unwrap is safe here since we .unwrap()check if there is exactly 1 element
                    let node_tuple = node_containers.get(0).unwrap().to_owned();
                    let node = Node::new(node_tuple.hostname().to_string());
                    match node
                        .run_command(
                            InternalCommand::Start {
                                container_id,
                                attach,
                            },
                            sudo,
                            identity_file,
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
                        .map(|result| result.id().to_string())
                        .collect::<Vec<String>>();
                    vec![Err(CommandError::MutlipleNodesFound(nodes))]
                }
            }
        }
        InternalCommand::Stop { container_id } => {
            let node_containers: Vec<Container> =
                find_containers(client, &container_id, sudo, false, identity_file).await;

            match node_containers.len() {
                0 => {
                    vec![Err(CommandError::NoMultipleNodesFound(container_id))]
                }
                1 => {
                    // unwrap is safe here since we .unwrap()check if there is exactly 1 element
                    let node_tuple = node_containers.get(0).unwrap().to_owned();
                    let node = Node::new(node_tuple.hostname().to_string());
                    match node
                        .run_command(InternalCommand::Stop { container_id }, sudo, identity_file)
                        .await
                    {
                        Ok(s) => vec![Ok(s)],
                        Err(e) => vec![Err(CommandError::NodeError(e))],
                    }
                }
                _ => {
                    let nodes = node_containers
                        .iter()
                        .map(|result| result.id().to_string())
                        .collect::<Vec<String>>();
                    vec![Err(CommandError::MutlipleNodesFound(nodes))]
                }
            }
        }
        InternalCommand::System(command) => {
            let bodies = stream::iter(client.nodes_info())
                .map(|(_, node)| async {
                    match node
                        .run_command(
                            InternalCommand::System(command.clone()),
                            sudo,
                            identity_file,
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
    }
}

pub enum CommandError<'a> {
    NoNodesFound(&'a str),
    NoMultipleNodesFound(Vec<&'a str>),
    MutlipleNodesFound(Vec<String>),
    NodeError(NodeError),
}

impl<'a> From<NodeError> for CommandError<'a> {
    fn from(node_error: NodeError) -> Self {
        Self::NodeError(node_error)
    }
}

impl<'a> std::fmt::Display for CommandError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::NoNodesFound(container_id) => write!(
                f,
                "No node found containing the following container: {}",
                container_id
            ),
            Self::NoMultipleNodesFound(container_ids) => write!(
                f,
                "No nodes found containing the following containers:\n{}",
                container_ids.join("\n")
            ),
            Self::MutlipleNodesFound(nodes) => write!(
                f,
                "Multiple nodes found with matching criteria:\n{}",
                nodes.join("\n")
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
            "Multiple nodes found with matching criteria:\nabc\ndef".into();

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
