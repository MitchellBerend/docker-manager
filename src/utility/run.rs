use futures::{stream, StreamExt};

use crate::cli::Command;
use crate::client::{Client, Node, NodeError};


const CONCURRENT_REQUESTS: usize = 10;

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
                .buffer_unordered(CONCURRENT_REQUESTS);
            bodies.collect::<Vec<Result<String, CommandError>>>().await
        }
        Command::Stop { container_id } => {
            let bodies = stream::iter(client.nodes_info())
                .map(|(hostname, node)| async move {
                    match node.run_command(Command::Ps { all: false }).await {
                        Ok(result) => (hostname.clone(), Ok(result)),
                        Err(e) => (hostname.clone(), Err(e)),
                    }
                })
                .buffer_unordered(CONCURRENT_REQUESTS);

            let node_containers: Vec<(String, String)> = bodies
                .collect::<Vec<(String, Result<String, NodeError>)>>()
                .await
                .iter()
                .filter(|(_, result)| match result {
                    Ok(s) => {
                        if s.contains(&container_id) {
                            true
                        } else {
                            false
                        }
                    }
                    Err(_) => false,
                })
                .map(|(hostname, result)| match result {
                    Ok(s) => (hostname.clone(), String::from(s.split('\n').nth(0).unwrap_or_else(|| ""))),
                    Err(_) => (hostname.clone(), String::from("")),
                })
                .collect::<Vec<(String, String)>>();
            if node_containers.len() == 1 {
                // unwrap is safe here since we .unwrap()check if there is exactly 1 element
                let node_tuple = node_containers.get(0).unwrap().to_owned();
                let node = Node::new(node_tuple.1);
                match node.run_command(Command::Stop { container_id }).await {
                    Ok(s) => return vec![Ok(s)],
                    Err(e) => return vec![Err(CommandError::NodeError(e))],
                }
            } else if node_containers.len() > 1 {
                let nodes = node_containers.iter().map(|(_, result)| result.clone()).collect::<Vec<String>>();
                vec![Err(CommandError::MutlipleNodesFound(nodes))]
            } else {
                let nodes = node_containers.iter().map(|(_, result)| result.clone()).collect::<Vec<String>>();
                vec![Err(CommandError::MutlipleNodesFound(nodes))]
            }
        }
    }
}

pub enum CommandError {
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
            Self::MutlipleNodesFound(nodes) => write!(
                f,
                "Multiple nodes found with matching criteria:\n{:#?}",
                nodes
            ),
            Self::NodeError(node_error) => write!(f, "{}", node_error),
        }
    }
}
