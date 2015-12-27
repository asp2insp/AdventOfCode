use std::str;
use std::collections::{HashMap,HashSet};
use chomp::*;
use chomp::ascii::{is_alpha,decimal,skip_whitespace};

struct Edge {
	from: String,
	to: String,
	dist: usize,
}

struct Node {
	edges: Vec<(String, usize)>,
	name: String,
}

impl Node{
	fn new(n: &String) -> Node {
		Node {
			edges: Vec::new(),
			name: n.clone(),
		}
	}
}

impl Clone for Node {
	fn clone(&self) -> Node {
		Node {
			name: self.name.clone(),
			edges: self.edges.clone(),
		}
	}
}

macro_rules! S8 {
	($buf:ident) => (
		str::from_utf8($buf).unwrap_or("S8 Error").to_string()
    )
}

fn edge(i: Input<u8>) -> U8Result<Edge> {
	// AlphaCentauri to Snowdin = 66
	parse!{i;
		let from = take_while(is_alpha);
				   string(b" to ");
		let to   = take_while(is_alpha);
				   string(b" = ");
		let dist = decimal();
				   skip_whitespace();
		ret Edge {
			from: S8!(from),
			to: S8!(to),
			dist: dist,
		}
	}
}

fn all_edges(i: Input<u8>) -> U8Result<Vec<Edge>> {
    parse!{i;
        let r = many1(edge);
		ret r
    }
}

fn make_graph(input: String) -> HashMap<String, Node> {
	let edges = parse_only(all_edges, input.as_bytes()).unwrap_or(vec![]);
	let mut nodes: HashMap<String, Node> = HashMap::new();
	for e in &edges {
		if !nodes.contains_key(&e.from) {
			nodes.insert(e.from.clone(), Node::new(&e.from));
		}
		if !nodes.contains_key(&e.to) {
			nodes.insert(e.to.clone(), Node::new(&e.to));
		}
	}
	for e in &edges {
		let to_ref = nodes.get(&e.to).unwrap().name.clone();
		let edit = nodes.get_mut(&e.from).unwrap();
		edit.edges.push((to_ref, e.dist));
	}
	// Add the reverse edge
	for e in &edges {
		let to_ref = nodes.get(&e.from).unwrap().name.clone();
		let edit = nodes.get_mut(&e.to).unwrap();
		edit.edges.push((to_ref, e.dist));
	}
	nodes
}

#[derive(Debug)]
struct Step {
	n: String,
	state: HashSet<String>,
	path: Vec<String>,
	target: usize,
}

#[derive(Debug)]
enum Tsp {
	Intermediate(Step, usize),
	Result(usize),
}

impl  Step {
	fn next(&self, g: &HashMap<String, Node>) -> Vec<Tsp> {
		g.get(&self.n)
			.unwrap()
			.edges
			.iter()
			.map(|p| {
				let name = p.0.clone();
				if self.state.contains(&name) {
					None // Don't visit any city twice
				} else {
					let mut new_state = self.state.clone();
					new_state.insert(name.clone());
					let mut path = self.path.clone();
					path.push(name);
					if new_state.len() < self.target {
						Some(Tsp::Intermediate(Step {
							n: p.0.clone(),
							state: new_state,
							path: path,
							target: self.target,
						}, p.1))
					} else {
						Some(Tsp::Result(p.1))
					}
				}
			})
			.filter(|o| o.is_some())
			.map(|o| o.unwrap())
			.collect()
	}
}

fn evaluate_tsp(s: &Step, g: &HashMap<String, Node>) -> usize {
	s.next(g).iter()
		.map(|tsp| {
			match tsp {
				&Tsp::Result(s) => s,
				&Tsp::Intermediate(ref step, len) => evaluate_tsp(&step, g) + len,
			}
		})
		.fold(4096usize, |min, u| {
			if u < min {
				u
			} else {
				min
			}
		})
}

fn tsp(g: &HashMap<String, Node>) -> String {
	format!("{}",
		g.keys().map(|k| {
			let mut start_state: HashSet<String> = HashSet::new();
			start_state.insert(k.clone());
			let start_step = Step {
				n: k.clone(),
				state: start_state,
				path: vec![k.clone()],
				target: g.len(),
			};
			evaluate_tsp(&start_step, g)
		})
		.fold(4096usize, |min, u| {
			if u < min {
				u
			} else {
				min
			}
		})
	)
}

fn evaluate_tsp_longest(s: &Step, g: &HashMap<String, Node>) -> usize {
	s.next(g).iter()
		.map(|tsp| {
			match tsp {
				&Tsp::Result(s) => s,
				&Tsp::Intermediate(ref step, len) => evaluate_tsp_longest(&step, g) + len,
			}
		})
		.fold(0, |max, u| {
			if u > max {
				u
			} else {
				max
			}
		})
}

fn tsp_longest(g: &HashMap<String, Node>) -> String {
	format!("{}",
		g.keys().map(|k| {
			let mut start_state: HashSet<String> = HashSet::new();
			start_state.insert(k.clone());
			let start_step = Step {
				n: k.clone(),
				state: start_state,
				path: vec![k.clone()],
				target: g.len(),
			};
			evaluate_tsp_longest(&start_step, g)
		})
		.fold(0, |max, u| {
			if u > max {
				u
			} else {
				max
			}
		})
	)
}

pub fn part1(input: String) -> String {
	let g = make_graph(input);
	tsp(&g)
}


pub fn part2(input: String) -> String {
	let g = make_graph(input);
	tsp_longest(&g)
}
