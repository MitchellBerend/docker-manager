use std::collections::HashMap;

use crate::constants;

use crate::cli::InternalCommand;
use crate::client::{Client, NodeError};

use futures::{stream, StreamExt};

/// This function takes a `Client` and returns a list of matched node names in the form of a
/// Vec of `Container`.
pub async fn find_containers(
    client: Client,
    container_ids: &[&str],
    sudo: bool,
    all: bool,
    identity_file: Option<&str>,
) -> Vec<Container> {
    let mut rv = vec![];

    let mut inter: HashMap<String, Vec<String>> = HashMap::new();

    for container_id in container_ids {
        let bodies = stream::iter(client.nodes_info())
            .map(|(hostname, node)| async move {
                match node
                    .run_command(
                        InternalCommand::Ps {
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

        let containers = bodies
            .collect::<Vec<(String, Result<String, NodeError>)>>()
            .await
            .iter()
            .filter_map(|(hostname, result)| node_filter_map((hostname, result), container_id))
            .collect::<Vec<(String, String, String)>>();

        for container in containers {
            let hostname = container.0;
            let container_id = container.2;

            if let Some(host_containers) = inter.get_mut(&hostname) {
                host_containers.push(container_id);
            } else {
                inter.insert(hostname, vec![container_id]);
            }
        }
    }

    for (hostname, containers) in inter.iter() {
        if !containers.is_empty() {
            rv.push(Container::new(
                hostname.to_string(),
                hostname.to_string(),
                containers.to_vec(),
            ))
        }
    }

    rv
}

/// Returns (hostname, node, container_id)
fn node_filter_map(
    hostname_node: (&str, &Result<String, NodeError>),
    container_id: &str,
) -> Option<(String, String, String)> {
    match hostname_node.1 {
        Ok(s) => {
            if s.contains(container_id) {
                Some((
                    hostname_node.0.to_string(),
                    String::from(s.split('\n').next().unwrap_or("")),
                    container_id.to_string(),
                ))
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

#[derive(Debug)]
pub struct FindContainerResult {
    containers: Vec<Container>,
}

impl Iterator for FindContainerResult {
    type Item = Container;

    fn next(&mut self) -> Option<Self::Item> {
        self.containers.pop()
    }
}

#[derive(Debug)]
pub struct Container {
    hostname: String,
    node: String,
    container_id: Vec<String>,
}

impl Container {
    fn new(hostname: String, node: String, container_id: Vec<String>) -> Self {
        Self {
            hostname,
            node,
            container_id,
        }
    }

    pub fn hostname(&self) -> &str {
        &self.hostname
    }

    pub fn node(&self) -> &str {
        &self.node
    }

    pub fn id(&self) -> Vec<&str> {
        let mut rv: Vec<&str> = vec![];

        for container in &self.container_id {
            rv.push(container);
        }

        rv
    }
}

#[cfg(test)]
mod test {
    use super::{node_filter_map, NodeError};

    #[test]
    fn test_node_filter_map() {
        let container_id = String::from("123123123");
        let original: Vec<(String, Result<String, NodeError>, String)> = vec![
            (
                "123123123".into(),
                Ok("123123123".into()),
                "123123123".into(),
            ),
            (
                "123123123".into(),
                Ok("123123123".into()),
                "123123123".into(),
            ),
            (
                "123123123".into(),
                Ok("123123123".into()),
                "123123123".into(),
            ),
            (
                "asdjkfhas".into(),
                Ok("asdjkfhas".into()),
                "asdjkfhas".into(),
            ),
        ];

        let new = original
            .into_iter()
            .filter_map(|(hostname, result, _)| {
                node_filter_map((&hostname, &result), &container_id)
            })
            .collect::<Vec<(String, String, String)>>();

        let correct: Vec<(String, String, String)> = vec![
            ("123123123".into(), "123123123".into(), "123123123".into()),
            ("123123123".into(), "123123123".into(), "123123123".into()),
            ("123123123".into(), "123123123".into(), "123123123".into()),
        ];

        assert_eq!(new, correct);
    }
}
