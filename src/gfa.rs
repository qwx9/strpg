use crate::*;
use petgraph::graphmap::UnGraphMap;
use rustc_hash::FxHashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn readnode(
    g: &mut UnGraphMap<Node, Edge>,
    seq: &mut FxHashMap<u64, String>,
    l: &[&str],
) -> Result<(), String> {
    if l.len() < 2 {
        return Err(format!("invalid node record: {}", l.join(" ")));
    }
    let s = l[2].to_string();
    let u = Node {
        id: l[1].parse::<u64>().map_err(|e| e.to_string())?,
    };
    seq.insert(u.id, s);
    g.add_node(u);
    Ok(())
}

fn readlink(g: &mut UnGraphMap<Node, Edge>, l: &[&str]) -> Result<(), String> {
    Ok(())
}

fn readpath(g: &mut UnGraphMap<Node, Edge>, l: &[&str]) -> Result<(), String> {
    Ok(())
}

fn verify(g: &UnGraphMap<Node, Edge>) -> Result<&UnGraphMap<Node, Edge>, String> {
    Ok(g)
}

pub fn readgfa(file: &String) -> Result<(UnGraphMap<Node, Edge>, FxHashMap<u64, String>), String> {
    let mut g = UnGraphMap::new();
    let mut seq: FxHashMap<u64, String> = FxHashMap::default();
    let bf: Box<dyn BufRead> = if *file == "".to_string() {
        Box::new(BufReader::new(std::io::stdin()))
    } else {
        Box::new(BufReader::new(File::open(file).map_err(|e| e.to_string())?))
    };
    bf.lines().try_for_each(|i| {
        i.map_err(|e| e.to_string()).and_then(|i| {
            let l = i.split_whitespace().collect::<Vec<_>>();
            match l[0] {
                "S" => readnode(&mut g, &mut seq, &l[1..]),
                "L" => readlink(&mut g, &l[1..]),
                "P" => readpath(&mut g, &l[1..]),
                _ => Ok(()),
            }
        })
    })?;
    verify(&g)?;
    Ok((g, seq))
}
