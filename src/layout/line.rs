use crate::layout::*;

use petgraph::graphmap::UnGraphMap;
use rustc_hash::FxHashSet;

struct CongaLine {}

impl Layout for CongaLine {
    fn compute<T: Draw>(&self, _g: &UnGraphMap<Node, Edge>) -> FxHashSet<Obj<T>> {
        let obj = FxHashSet::default();
        obj
    }
}
