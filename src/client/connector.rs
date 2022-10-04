use crate::cli::Command;
use crate::utility::command;

pub struct Client {
    nodes: Vec<Node>,
}

impl Client {
    pub fn from_config() -> Self {
        let config_path = format!(
            "{}/.ssh/config",
            std::env::var("HOME").unwrap_or_else(|_| "/home/root".into())
        );
        let file_contents = std::fs::read_to_string(config_path).unwrap_or_else(|_| "".into());
        let mut nodes: Vec<Node> = vec![];
        for line in file_contents.split('\n') {
            if !line.contains('#') && line.contains("Host") {
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

pub struct Node {
    address: String,
}

impl Node {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub async fn run_command(&self, command: Command) -> Result<String, NodeError> {
        let session = match openssh::SessionBuilder::default()
            .connect_timeout(std::time::Duration::new(1, 0))
            .connect_mux(&self.address)
            .await
        {
            Ok(session) => session,
            Err(e) => return Err(NodeError::SessionError(self.address.clone(), e)),
        };

        match command {
            Command::Ps { all } => {
                match command::run_ps(self.address.clone(), session, all).await {
                    Ok(result) => Ok(result),
                    Err(e) => Err(NodeError::SessionError(self.address.clone(), e)),
                }
            },
            Command::Stop { container_id } => {
                match command::run_stop(self.address.clone(), session, container_id).await {
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
