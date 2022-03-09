use clap::Parser;
use log::*;
use petgraph::graphmap::UnGraphMap;
use rustc_hash::FxHashSet;
use std::io::{self};
use strpg::{gfa::*, *};

#[derive(clap::Parser)]
struct Args {
    #[clap(required = false, default_value = "")]
    gfa: String,
}

fn show(g: &UnGraphMap<Node, Edge>) {}

fn init() {
    env_logger::init();
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    init();
    trace!("loading gfa file {}", args.gfa);
    let (g, seq) = readgfa(&args.gfa)?;
    show(&g);
    Ok(())
}
