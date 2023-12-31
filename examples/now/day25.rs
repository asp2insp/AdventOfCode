use itertools::Itertools;
use rand::prelude::*;
use std::collections::{HashMap, HashSet};

fn parse(s: &str) -> HashMap<String, HashSet<String>> {
    let mut m: HashMap<String, HashSet<String>> = s
        .lines()
        .map(|l| {
            let (n, r) = l.split(':').collect_tuple().unwrap();
            let conns = r
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect::<HashSet<_>>();
            (n.to_string(), conns)
        })
        .collect();
    for (k, v) in m.clone().into_iter() {
        for c in v {
            m.entry(c).or_default().insert(k.clone());
        }
    }
    m
}

#[derive(Debug, Clone)]
struct Adjacency {
    from: String,
    to: String,
    orig_from: String,
    orig_to: String,
}

impl Adjacency {
    fn new(from: String, to: String) -> Self {
        Self {
            from: from.clone(),
            to: to.clone(),
            orig_from: from,
            orig_to: to,
        }
    }

    fn matches(&self, other: &Self) -> bool {
        (self.from == other.from && self.to == other.to)
            || (self.from == other.to && self.to == other.from)
    }
}

fn map_to_adj_list(m: &HashMap<String, HashSet<String>>) -> Vec<Adjacency> {
    m.iter()
        .flat_map(|(k, v)| v.iter().map(|c| Adjacency::new(k.clone(), c.clone())))
        .collect()
}

fn kargers_to_n(graph: Vec<Adjacency>, n: usize) -> Vec<Adjacency> {
    let mut rng = rand::thread_rng();
    loop {
        let mut graph = graph.clone();
		// continue until we're only at 2 nodes
        while graph.iter().flat_map(|a| [&a.from, &a.to]).unique().count() > 2 {
            let i = rng.gen_range(0..graph.len());
            let adj = graph.remove(i);
            let from = adj.from.clone();
            let to = adj.to.clone();
            // Remove the reverse edge
            graph.retain(|a| a.from != to || a.to != from);
            graph = graph
                .into_iter()
                .map(|mut a| {
                    if a.from == to {
                        a.from = from.clone();
                    }
                    if a.to == to {
                        a.to = from.clone();
                    }
                    a
                })
                .collect();
			graph.retain(|a| a.from != a.to);
        }
		// println!("Found a cut with {} edges {:?}", graph.len(), graph);
        if graph.len() == n * 2 {
            break graph;
        }
    }
    .into_iter()
    .map(|a| Adjacency::new(a.orig_from, a.orig_to))
    .collect()
}

fn subgraph_size(graph: &[Adjacency], start: &str) -> usize {
    let mut seen = HashSet::new();
    let mut to_visit = vec![start.to_string()];
    while let Some(node) = to_visit.pop() {
        if seen.contains(&node) {
            continue;
        }
        seen.insert(node.clone());
        to_visit.extend(
            graph
                .iter()
                .filter(|a| a.from == node)
                .map(|a| a.to.clone()),
        );
    }
    seen.len()
}

pub fn part1(input: String) -> String {
    let map = parse(&input);
    let mut adj_list = map_to_adj_list(&map);
	// for (k, v) in &map {
	// 	for c in v {
	// 		println!("{} -- {}", k, c);
	// 	}
	// }
    // let cut = kargers_to_n(adj_list, 3);
	// Found via Graphviz
	let cut = [
		Adjacency::new("qmr".to_string(), "ptj".to_string()),
		Adjacency::new("lsv".to_string(), "lxt".to_string()),
		Adjacency::new("dhn".to_string(), "xvh".to_string()),
	];
	let size_before = adj_list.len();
	adj_list.retain(|a| !cut.iter().any(|c| a.matches(c)));
	println!("Removed {} edges", size_before - adj_list.len());
    let lsize = subgraph_size(&adj_list, &cut[0].from);
    let rsize = subgraph_size(&adj_list, &cut[0].to);
	format!("{} * {} = {}", lsize, rsize, lsize * rsize)
}

pub fn part2(input: String) -> String {
    "part2".to_string()
}
