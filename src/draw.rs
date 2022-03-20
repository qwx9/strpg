use crate::{Edge::*, *};

use log::{debug, info, trace, warn};
use macroquad::prelude::*;
use petgraph::graphmap::UnGraphMap;
use rustc_hash::{FxHashMap, FxHashSet};

pub async fn draw(
    g: &UnGraphMap<Node, Edge>,
    seq: &FxHashMap<u64, String>,
    path: &FxHashMap<String, Vec<Step>>,
) -> Result<(), String> {
    loop {
        clear_background(BLACK);
        draw_text("no.", 20.0, 20.0, 20.0, DARKGRAY);
        next_frame().await
    }
    Ok(())
}
