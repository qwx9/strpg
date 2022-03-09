pub mod gfa;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Node {
    id: u64,
}
pub type Edge = u64;
