use clap::Parser;
use log::*;
use petgraph::graphmap::UnGraphMap;
use rustc_hash::FxHashSet;
use std::io::{self};
use strpg::{draw::*, gfa::*, *};

#[derive(clap::Parser)]
struct Args {
    #[clap(required = false, default_value = "")]
    gfa: String,
}

fn init() {
    env_logger::init();
}

#[macroquad::main("strpg")]
async fn main() -> Result<(), String> {
    let args = Args::parse();
    init();
    let (g, seq, paths) = readgfa(&args.gfa)?;
    draw(&g, &seq, &paths).await?;
    Ok(())
}
