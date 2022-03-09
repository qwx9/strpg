use crate::{Edge::*, *};
use petgraph::graphmap::UnGraphMap;
use regex::Regex;
use rustc_hash::FxHashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/* GFAv1 only */

/* FIXME: node names are strings */

fn readnode(
    g: &mut UnGraphMap<Node, Edge>,
    l: &[&str],
    seq: &mut FxHashMap<u64, String>,
) -> Result<(), String> {
    if l.len() < 2 {
        return Err(format!("invalid segment record: {}", l.join(" ")));
    }
    let s = l[1].to_string();
    let u = Node {
        id: l[0].parse::<u64>().map_err(|e| e.to_string())?,
    };
    seq.insert(u.id, s);
    g.add_node(u);
    Ok(())
}

/* ignoring overlap field */
fn readlink(g: &mut UnGraphMap<Node, Edge>, l: &[&str]) -> Result<(), String> {
    if l.len() < 4 {
        return Err(format!("invalid link record: {}", l.join(" ")));
    }
    let u = Node {
        id: l[0].parse::<u64>().map_err(|e| e.to_string())?,
    };
    let v = Node {
        id: l[2].parse::<u64>().map_err(|e| e.to_string())?,
    };
    if !g.contains_node(u) || !g.contains_node(v) {
        return Err(format!("unknown segment in link record: {}", l.join(" ")));
    }
    let w = match (l[1], l[3]) {
        ("+", "+") => Ok(FwFw),
        ("-", "-") => Ok(InvInv),
        ("+", "-") => Ok(FwInv),
        ("-", "+") => Ok(InvFw),
        _ => Err(format!("invalid link type: {}", l.join(" "))),
    }?;
    g.add_edge(u, v, w);
    Ok(())
}

/* ignoring overlap field */
fn readpath(
    g: &mut UnGraphMap<Node, Edge>,
    l: &[&str],
    path: &mut FxHashMap<String, Vec<Step>>,
) -> Result<(), String> {
    if l.len() < 2 {
        return Err(format!("invalid path record: {}", l.join(" ")));
    }

    /* FIXME: what in the fuck */
    let mut v = Vec::new();
    let re = Regex::new(r"^([0-9]+)(\+|-)$").unwrap();
    for i in l[1].split(',').collect::<Vec<&str>>() {
        match re.captures(i) {
            Some(m) => {
                if m.len() != 2 {
                    return Err(format!("invalid node in path record: {}", l[0]));
                }
                let id = m
                    .get(0)
                    .ok_or(format!("invalid node in path record: {}", l[0]))?
                    .as_str()
                    .parse::<u64>()
                    .map_err(|e| e.to_string())?;
                let rev = m
                    .get(1)
                    .ok_or(format!("invalid node in path record: {}", l[0]))?
                    .as_str()
                    == "-";
                let s = Step { id: id, rev: rev };
            }
            _ => return Err(format!("invalid node in path record: {}", l[0])),
        }
    }
    path.insert(l[0].to_string(), v);
    Ok(())
}

fn verify(g: &UnGraphMap<Node, Edge>) -> Result<&UnGraphMap<Node, Edge>, String> {
    // unused nodes warning!
    // unused edges? info!
    // unknown edge
    Ok(g)
}

pub fn readgfa(file: &String) -> Result<(UnGraphMap<Node, Edge>, FxHashMap<u64, String>), String> {
    let mut g = UnGraphMap::new();
    let mut seq: FxHashMap<u64, String> = FxHashMap::default();
    let mut path: FxHashMap<String, Vec<Step>> = FxHashMap::default();

    let bf: Box<dyn BufRead> = if *file == "".to_string() {
        Box::new(BufReader::new(std::io::stdin()))
    } else {
        Box::new(BufReader::new(File::open(file).map_err(|e| e.to_string())?))
    };
    bf.lines().try_for_each(|i| {
        i.map_err(|e| e.to_string()).and_then(|i| {
            let l = i.split_whitespace().collect::<Vec<_>>();
            match l[0] {
                "S" => readnode(&mut g, &l[1..], &mut seq),
                "L" => readlink(&mut g, &l[1..]),
                "P" => readpath(&mut g, &l[1..], &mut path),
                _ => Ok(()),
            }
        })
    })?;
    verify(&g)?;
    Ok((g, seq))
}
