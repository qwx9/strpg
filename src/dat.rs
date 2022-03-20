#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Node {
    pub id: u64,
}
#[derive(Eq, PartialEq, Hash)]
pub enum Edge {
    /* lol */
    FwFw,
    InvInv,
    FwInv,
    InvFw,
}
#[derive(Eq, PartialEq, Hash)]
pub struct Step {
    pub id: u64,
    pub rev: bool,
}
#[derive(Eq, PartialEq, Hash)]
pub struct Path {
    pub name: String,
    pub p: Vec<Step>,
}
