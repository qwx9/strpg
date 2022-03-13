pub mod draw;
pub mod gfa;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Node {
    id: u64,
}
pub enum Edge {
    /* lol */
    FwFw,
    InvInv,
    FwInv,
    InvFw,
}
pub struct Step {
    id: u64,
    rev: bool,
}
