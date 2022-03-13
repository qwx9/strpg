use crate::{Edge::*, *};
use log::{debug, info, trace, warn};
use petgraph::graphmap::UnGraphMap;
use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/* GFAv1 only, with numerical segment id's */

fn readnode(
    g: &mut UnGraphMap<Node, Edge>,
    l: &[&str],
    seq: &mut FxHashMap<u64, String>,
    h: &mut FxHashSet<Node>,
) -> Result<(), String> {
    trace!("readnode: new node {}", l[0]);
    if l.len() < 2 {
        return Err(format!("invalid segment record: {}", l.join(" ")));
    }
    let s = l[1].to_string();
    let u = Node {
        id: l[0].parse::<u64>().map_err(|e| e.to_string())?,
    };
    if g.contains_node(u) {
        warn!("readnode: duplicate node {}", u.id);
    }
    if s != "*" {
        trace!("readnode: inserting new sequence {} for node {}", s, u.id);
        seq.insert(u.id, s);
    }
    h.insert(u);
    g.add_node(u);
    Ok(())
}

/* ignoring overlap field */
fn readlink(
    g: &mut UnGraphMap<Node, Edge>,
    l: &[&str],
    h: &mut FxHashSet<Node>,
    e: &mut FxHashSet<String>,
) -> Result<(), String> {
    trace!("readlink: parsing new record {}", l[0]);
    if l.len() < 4 {
        return Err(format!("invalid new link: {}", l.join(" ")));
    }
    let u = Node {
        id: l[0].parse::<u64>().map_err(|e| e.to_string())?,
    };
    let v = Node {
        id: l[2].parse::<u64>().map_err(|e| e.to_string())?,
    };
    if !g.contains_node(u) || !g.contains_node(v) {
        return Err(format!("unknown node in link record: {}", l.join(" ")));
    }
    let w = match (l[1], l[3]) {
        ("+", "+") => Ok(FwFw),
        ("-", "-") => Ok(InvInv),
        ("+", "-") => Ok(FwInv),
        ("-", "+") => Ok(InvFw),
        _ => Err(format!("invalid link type: {}", l.join(" "))),
    }?;
    if g.contains_edge(u, v) {
        warn!("readnode: duplicate link {},{}", u.id, v.id);
    }
    h.remove(&u);
    h.remove(&v);
    e.insert(format!("{}{}{}{}", l[0], l[1], l[2], l[3]));
    debug!("inserted edge {}{}{}{}", l[0], l[1], l[2], l[3]);
    g.add_edge(u, v, w);
    Ok(())
}

/* ignoring overlap field */
fn readpath(
    g: &mut UnGraphMap<Node, Edge>,
    l: &[&str],
    path: &mut FxHashMap<String, Vec<Step>>,
    e: &mut FxHashSet<String>,
) -> Result<(), String> {
    trace!("readpath: parsing new record {}", l[0]);
    if l.len() < 2 {
        return Err(format!("invalid path record: {}", l[0]));
    }

    let name = l[0].to_string();
    let mut v = Vec::new();
    let re = Regex::new(r"^([0-9]+)(\+|-)$").unwrap();
    let mut prev: (u64, bool) = (0, false);
    for i in l[1].split(',').collect::<Vec<&str>>() {
        match re.captures(i) {
            Some(m) => {
                if m.len() != 3 {
                    return Err(format!("invalid node {} format in path record {}", i, l[0]));
                }
                let id = m
                    .get(1)
                    .ok_or(format!("invalid node {} name in path record {}", i, l[0]))?
                    .as_str()
                    .parse::<u64>()
                    .map_err(|e| e.to_string())?;
                if !g.contains_node(Node { id: id }) {
                    return Err(format!("unknown node {} in path record {}", id, l[0]));
                }
                if !v.is_empty() && !g.contains_edge(Node { id: id }, Node { id: prev.0 }) {
                    return Err(format!(
                        "unknown edge {},{} in path record {}",
                        id, prev.0, l[0]
                    ));
                }
                let rev = m
                    .get(2)
                    .ok_or(format!(
                        "invalid node {} direction in path record {}",
                        i, l[0]
                    ))?
                    .as_str()
                    == "-";
                if !v.is_empty() {
                    e.remove(
                        format!(
                            "{}{}{}{}",
                            prev.0,
                            if prev.1 { "-" } else { "+" },
                            id,
                            if rev { "-" } else { "+" }
                        )
                        .as_str(),
                    );
                }
                v.push(Step { id: id, rev: rev });
                prev = (id, rev);
            }
            _ => return Err(format!("invalid node {} format in path record {}", i, l[0])),
        }
    }
    if path.contains_key(&name) {
        warn!("path: duplicate path {},", name);
    }
    path.insert(name, v);
    Ok(())
}

fn summary(
    g: &UnGraphMap<Node, Edge>,
    seq: &FxHashMap<u64, String>,
    path: &FxHashMap<String, Vec<Step>>,
    h: &FxHashSet<Node>,
    e: &FxHashSet<String>,
) -> Result<(), String> {
    seq.iter()
        .for_each(|(k, v)| debug!("non-empty sequence for node {}: {}", k, v));
    path.iter().for_each(|(k, v)| {
        debug!(
            "path {}: {}",
            k,
            v.iter()
                .map(|x| format!("{}{}", if x.rev { "<" } else { ">" }, x.id))
                .collect::<Vec<String>>()
                .join("")
        )
    });
    if !h.is_empty() {
        warn!(
            "unused nodes: {}",
            h.iter()
                .map(|x| x.id.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
    }
    if !e.is_empty() {
        warn!(
            "unused edges: {}",
            e.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
    }
    Ok(())
}

pub fn readgfa(
    file: &String,
) -> Result<
    (
        UnGraphMap<Node, Edge>,
        FxHashMap<u64, String>,
        FxHashMap<String, Vec<Step>>,
    ),
    String,
> {
    let mut g = UnGraphMap::new();
    let mut seq: FxHashMap<u64, String> = FxHashMap::default();
    let mut path: FxHashMap<String, Vec<Step>> = FxHashMap::default();
    let mut h: FxHashSet<Node> = FxHashSet::default();
    let mut e: FxHashSet<String> = FxHashSet::default();

    info!("loading gfa file {}", file);
    let bf: Box<dyn BufRead> = if *file == "".to_string() {
        Box::new(BufReader::new(std::io::stdin()))
    } else {
        Box::new(BufReader::new(File::open(file).map_err(|e| e.to_string())?))
    };
    bf.lines().try_for_each(|i| {
        i.map_err(|e| e.to_string()).and_then(|i| {
            let l = i.split_whitespace().collect::<Vec<_>>();
            match l[0] {
                "S" => readnode(&mut g, &l[1..], &mut seq, &mut h),
                "L" => readlink(&mut g, &l[1..], &mut h, &mut e),
                "P" => readpath(&mut g, &l[1..], &mut path, &mut e),
                _ => Ok(()),
            }
        })
    })?;
    info!("done parsing input file");
    summary(&g, &seq, &path, &h, &e)?;
    Ok((g, seq, path))
}
