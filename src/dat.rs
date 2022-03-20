#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Node {
    pub id: u64,
}
pub enum Edge {
    /* lol */
    FwFw,
    InvInv,
    FwInv,
    InvFw,
}
pub struct Step {
    pub id: u64,
    pub rev: bool,
}
