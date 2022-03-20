use crate::layout::*;

use std::hash::Hash;

use petgraph::graphmap::UnGraphMap;
use rustc_hash::FxHashSet;

pub struct CongaLine {}

impl Layout for CongaLine {
    fn compute(&self, g: &UnGraphMap<Node, Edge>) -> FxHashSet<Obj> {
        let mut obj: FxHashSet<Obj> = FxHashSet::default();
        let mut x = 0;
        for n in g.nodes() {
            let o = Obj {
                r: Rect {
                    p: Point { x: x, y: 0, z: 0 },
                    Δ: Point {
                        x: 2,
                        y: 1,
                        z: 0,
                    },
                },
                o: Dicks::N(n),
            };
            obj.insert(o);
            x += 5;
        }
        obj
    }
}
