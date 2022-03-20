use crate::dat::*;

use petgraph::graphmap::UnGraphMap;
use rustc_hash::FxHashSet;

pub mod line;

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Rect {
    pub min: Point,
    pub max: Point,
}

pub trait Draw {
    fn draw(&self);
}

pub struct Obj<T: Draw> {
    pub r: Rect,
    pub obj: T,
}

impl Draw for Node {
    fn draw(&self) {}
}

impl Draw for Edge {
    fn draw(&self) {}
}

impl Draw for Step {
    fn draw(&self) {}
}

pub trait Layout {
    fn compute<T: Draw>(&self, g: &UnGraphMap<Node, Edge>) -> FxHashSet<Obj<T>>;
}
