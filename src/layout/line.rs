use crate::layout::*;

use std::hash::Hash;

use petgraph::graphmap::UnGraphMap;
use rustc_hash::FxHashSet;

struct CongaLine {}

impl Layout for CongaLine {
    fn compute(&self, g: &UnGraphMap<Node, Edge>) -> FxHashSet<Obj> {
        let mut obj: FxHashSet<Obj> = FxHashSet::default();
        let mut x = 0;
        for n in g.nodes() {
            let o = Obj {
                r: Rect {
                    min: Point { x: x, y: 0, z: 0 },
                    max: Point {
                        x: x + 1,
                        y: 0,
                        z: 0,
                    },
                },
                o: Dicks::N(n),
            };
            obj.insert(o);
            x += 1;
        }
        obj
    }
}
