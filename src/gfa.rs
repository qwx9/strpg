use crate::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use petgraph::graphmap::UnGraphMap;
use rustc_hash::FxHashSet;

fn readnode(g: &mut UnGraphMap<Node, Edge>, l: &[&str]) {

}

fn readlink(g: &mut UnGraphMap<Node, Edge>, l: &[&str]) {

}

fn readpath(g: &mut UnGraphMap<Node, Edge>, l: &[&str]) {

}

fn verify(g: UnGraphMap<Node, Edge>) -> Result<UnGraphMap<Node, Edge>, io::Error> {
	Ok(g)
}

pub fn readgfa(file: &String) -> Result<UnGraphMap<Node, Edge>, io::Error> {
	let mut g = UnGraphMap::new();
	let bf: Box<dyn BufRead> = if *file == "".to_string() {
		Box::new(BufReader::new(std::io::stdin()))
	} else {
		Box::new(BufReader::new(File::open(file).expect("open")))
	};
	for i in bf.lines() {
		if let Ok(x) = i {
		let l = x.split_whitespace().collect::<Vec<_>>();
		if l.len() < 1 {
			continue;
		}
		match l[0] {
		"S" => readnode(&mut g, &l[1..]),
		"L" => readlink(&mut g, &l[1..]),
		"P" => readpath(&mut g, &l[1..]),
		_ => (),
		}
		}
	}
	verify(g)
}
