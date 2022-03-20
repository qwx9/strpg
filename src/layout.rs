use crate::dat::*;

use std::hash::Hash;

use petgraph::graphmap::UnGraphMap;
use rustc_hash::FxHashSet;

pub mod line;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Rect {
    pub p: Point,
    pub Δ: Point,
}

pub trait Draw {
    fn draw(&self, r: Rect);
}

#[derive(PartialEq, Eq, Hash)]
pub struct Obj {
    pub r: Rect,
    pub o: Dicks,
}

impl Draw for Edge {
    fn draw(&self, r: Rect) {}
}

impl Draw for Path {
    fn draw(&self, r: Rect) {}
}

#[derive(PartialEq, Eq, Hash)]
pub enum Dicks {
    N(Node),
    E(Edge),
    P(Path),
}

impl Draw for Dicks {
    fn draw(&self, r: Rect) {
        match (&self) {
            &Dicks::N(ref x) => x.draw(r),
            &Dicks::E(ref x) => x.draw(r),
            &Dicks::P(ref x) => x.draw(r),
        }
    }
}

pub trait Layout {
    fn compute(&self, g: &UnGraphMap<Node, Edge>) -> FxHashSet<Obj>;
}
