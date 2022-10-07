use crate::constants;

use crate::cli::Command;
use crate::client::{Client, NodeError};

use futures::{stream, StreamExt};

/// This function takes a `Client` and returns a list of matched node names in the form of a tuple
/// with (hostname, container_id).
pub async fn find_container(client: Client, container_id: &str) -> Vec<(String, String)> {
    let bodies = stream::iter(client.nodes_info())
        .map(|(hostname, node)| async move {
            match node
                .run_command(Command::Ps {
                    all: false,
                    filter: None,
                    format: None,
                    last: false,
                    latests: false,
                    no_trunc: false,
                    quiet: false,
                    size: false,
                })
                .await
            {
                Ok(result) => (hostname.clone(), Ok(result)),
                Err(e) => (hostname.clone(), Err(e)),
            }
        })
        .buffer_unordered(constants::CONCURRENT_REQUESTS);

    bodies
        .collect::<Vec<(String, Result<String, NodeError>)>>()
        .await
        .iter()
        .filter(|(_, result)| match result {
            Ok(s) => s.contains(&container_id),
            Err(_) => false,
        })
        .map(|(hostname, result)| match result {
            Ok(s) => (
                hostname.clone(),
                String::from(s.split('\n').next().unwrap_or("")),
            ),
            Err(_) => (hostname.clone(), String::from("")),
        })
        .collect::<Vec<(String, String)>>()
}
