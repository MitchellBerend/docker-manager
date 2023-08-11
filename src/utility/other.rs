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
    all: bool,
    identity_file: Option<&str>,
) -> Vec<(String, String)> {
    let bodies = stream::iter(client.nodes_info())
        .map(|(hostname, node)| async move {
            match node
                .run_command(
                    Command::Ps {
                        all,
                        filter: None,
                        format: None,
                        last: false,
                        latests: false,
                        no_trunc: false,
                        quiet: false,
                        size: false,
                    },
                    sudo,
                    identity_file,
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
        .filter_map(|(hostname, result)| node_filter_map((hostname, result), container_id))
        .collect::<Vec<(String, String)>>()
}

fn node_filter_map(
    hostname_node: (&str, &Result<String, NodeError>),
    container_id: &str,
) -> Option<(String, String)> {
    match hostname_node.1 {
        Ok(s) => {
            if s.contains(container_id) {
                Some((
                    hostname_node.0.to_string(),
                    String::from(s.split('\n').next().unwrap_or("")),
                ))
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

#[cfg(test)]
mod test {
    use super::{node_filter_map, NodeError};

    #[test]
    fn test_node_filter_map() {
        let container_id = String::from("123123123");
        let original: Vec<(String, Result<String, NodeError>)> = vec![
            ("123123123".into(), Ok("123123123".into())),
            ("123123123".into(), Ok("123123123".into())),
            ("123123123".into(), Ok("123123123".into())),
            ("asdjkfhas".into(), Ok("asdjkfhas".into())),
        ];

        let new = original
            .into_iter()
            .filter_map(|(hostname, result)| node_filter_map((&hostname, &result), &container_id))
            .collect::<Vec<(String, String)>>();

        let correct: Vec<(String, String)> = vec![
            ("123123123".into(), "123123123".into()),
            ("123123123".into(), "123123123".into()),
            ("123123123".into(), "123123123".into()),
        ];

        assert_eq!(new, correct);
    }
}
