use strpg::{*, gfa::*};
use std::io::{self};
use clap::Parser;
use petgraph::graphmap::UnGraphMap;
use rustc_hash::FxHashSet;

#[derive(clap::Parser)]
struct Args {
	#[clap(
		required = false,
		default_value = "",
	)]
	gfa: String,
}

fn show(g: &UnGraphMap<Node, Edge>) {

}

fn init() {

}

fn main() -> Result<(), io::Error> {
	let args = Args::parse();
	init();
	let g = readgfa(&args.gfa)?;
	show(&g);
	Ok(())
}
