use crate::{dat::*, layout::Layout, line::CongaLine, render::*};

use log::{debug, info, trace, warn};
use macroquad::prelude::*;
use petgraph::graphmap::UnGraphMap;
use rustc_hash::{FxHashMap, FxHashSet};

fn flushscreen() {}

pub async fn draw(
    g: &UnGraphMap<Node, Edge>,
    seq: &FxHashMap<u64, String>,
    path: &FxHashMap<String, Vec<Step>>,
    lf: &dyn Layout,
) -> Result<(), String> {
    let o = lf.compute(&g);
    clear_background(BLACK);
    loop {
        render(&o);
        flushscreen();

        next_frame().await;
    }
    Ok(())
}
