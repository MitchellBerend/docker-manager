#[derive(Debug, Default)]
pub struct NodeMemory {
    pub node: String,
    pub memtotal: u64,
    pub memfree: u64,
}
