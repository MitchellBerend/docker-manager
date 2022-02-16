#[derive(Debug)]
pub struct NodeMemory {
    pub node: String,
    pub memtotal: u64,
    pub memfree: u64,
}

impl Default for NodeMemory {
    fn default() -> Self {
        Self { node: Default::default(), memtotal: Default::default(), memfree: Default::default() }
    }
}