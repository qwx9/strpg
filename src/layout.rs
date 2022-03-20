use crate::dat::*;

use std::hash::Hash;

use petgraph::graphmap::UnGraphMap;
use rustc_hash::FxHashSet;

pub mod line;

#[derive(PartialEq, Eq, Hash)]
pub struct Point {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

#[derive(PartialEq, Eq, Hash)]
pub struct Rect {
    pub min: Point,
    pub max: Point,
}

pub trait Draw {
    fn draw(&self);
}

pub enum W {
    Node,
    Edge,
    Step,
}

#[derive(PartialEq, Eq, Hash)]
pub struct Obj {
    pub r: Rect,
    pub o: Dicks,
}

impl Draw for Node {
    fn draw(&self) {}
}

impl Draw for Edge {
    fn draw(&self) {}
}

impl Draw for Path {
    fn draw(&self) {}
}

#[derive(PartialEq, Eq, Hash)]
pub enum Dicks {
    N(Node),
    E(Edge),
    P(Path),
}

impl Draw for Dicks {
    fn draw(&self) {
        match (&self) {
            &Dicks::N(ref x) => x.draw(),
            &Dicks::E(ref x) => x.draw(),
            &Dicks::P(ref x) => x.draw(),
        }
    }
}

pub trait Layout {
    fn compute(&self, g: &UnGraphMap<Node, Edge>) -> FxHashSet<Obj>;
}
