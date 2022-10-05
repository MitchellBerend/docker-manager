use futures::{stream, StreamExt};

use crate::constants;

use crate::cli::Command;
use crate::client::{Client, Node, NodeError};
use crate::utility::find_container;

pub async fn run_command(command: Command) -> Vec<Result<String, CommandError>> {
    let client = Client::from_config();

    match command {
        Command::Ps { all } => {
            let bodies = stream::iter(client.nodes_info())
                .map(|(_, node)| async move {
                    match node.run_command(Command::Ps { all }).await {
                        Ok(result) => Ok(result),
                        Err(e) => Err(CommandError::NodeError(e)),
                    }
                })
                .buffer_unordered(constants::CONCURRENT_REQUESTS);
            bodies.collect::<Vec<Result<String, CommandError>>>().await
        }
        Command::Stop { container_id } => {
            let node_containers: Vec<(String, String)> =
                find_container(client, &container_id).await;

            match node_containers.len() {
                0 => {
                    vec![Err(CommandError::NoNodesFound(container_id))]
                }
                1 => {
                    // unwrap is safe here since we .unwrap()check if there is exactly 1 element
                    let node_tuple = node_containers.get(0).unwrap().to_owned();
                    let node = Node::new(node_tuple.1);
                    match node.run_command(Command::Stop { container_id }).await {
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
        Command::Logs {
            container_id,
//            follow,
        } => {
            let node_containers: Vec<(String, String)> =
                find_container(client, &container_id).await;
            match node_containers.len() {
                0 => {
                    vec![Err(CommandError::NoNodesFound(container_id))]
                }
                1 => {
                    // unwrap is safe here since we .unwrap()check if there is exactly 1 element
                    let node_tuple = node_containers.get(0).unwrap().to_owned();
                    let node = Node::new(node_tuple.1);
                    match node.run_command(Command::Logs { container_id }).await { //, follow }).await {
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
