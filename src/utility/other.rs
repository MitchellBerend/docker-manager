use crate::constants;

use crate::cli::Command;
use crate::client::{Client, NodeError};

use futures::{stream, StreamExt};

/// This function takes a `Client` and returns a list of matched node names in the form of a tuple
/// with (hostname, container_id).
pub async fn find_container(
    client: Client,
    container_id: &str,
    sudo: bool,
) -> Vec<(String, String)> {
    let bodies = stream::iter(client.nodes_info())
        .map(|(hostname, node)| async move {
            match node
                .run_command(
                    Command::Ps {
                        all: false,
                        filter: None,
                        format: None,
                        last: false,
                        latests: false,
                        no_trunc: false,
                        quiet: false,
                        size: false,
                    },
                    sudo,
                )
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
        .filter_map(|(hostname, result)| match result {
            Ok(s) => {
                if s.contains(&container_id) {
                    Some((
                        hostname.clone(),
                        String::from(s.split('\n').next().unwrap_or("")),
                    ))
                } else {
                    None
                }
            }
            Err(_) => None,
        })
        .collect::<Vec<(String, String)>>()
}
