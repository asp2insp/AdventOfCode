use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
	static ref RULE_RE: Regex =
		Regex::new(r"^(\w+ \w+) bags? contain (\d (\w+ \w+) bags?[,.])+$").unwrap();
	static ref COLOR_RE: Regex = Regex::new(r"(\w+ \w+) bag").unwrap();
	static ref EDGE_RE: Regex = Regex::new(r"(\d) (\w+ \w+) bag").unwrap();
}

fn reverse_mappings(input: &str) -> HashMap<String, HashSet<String>> {
	let mut res = HashMap::new();
	for line in input.lines() {
		let mut first: Option<String> = None;
		for cap in COLOR_RE.captures_iter(line) {
			if let Some(f) = &first {
				res.entry(cap[1].to_string())
					.or_insert(HashSet::new())
					.insert(f.clone());
			} else {
				first = Some(cap[1].to_string())
			}
		}
	}
	res
}

pub fn part1(input: String) -> String {
	let mappings = reverse_mappings(&input);
	// println!("{:#?}", mappings);
	let mut s: HashSet<String> = HashSet::new();
	s.insert("shiny gold".to_owned());
	let mut q = Vec::new();
	q.push("shiny gold");
	while let Some(n) = q.pop() {
		if let Some(edges) = mappings.get(n) {
			for e in edges {
				if !s.contains(e) {
					s.insert(e.clone());
					q.push(e);
				}
			}
		}
	}
	s.remove("shiny gold");
	s.len().to_string()
}

fn forward_mappings(input: &str) -> HashMap<String, HashSet<(usize, String)>> {
	let mut res = HashMap::new();
	for line in input.lines() {
		let lhs = COLOR_RE.captures_iter(line).next().unwrap()[1].to_string();
		for cap in EDGE_RE.captures_iter(line) {
			res.entry(lhs.clone())
					.or_insert(HashSet::new())
					.insert((cap[1].parse::<usize>().unwrap(), cap[2].to_string()));
		}
	}
	res
}

pub fn traverse(color: &str, n: usize, mappings: &HashMap<String, HashSet<(usize, String)>>) -> usize {
	let mut subsum = 1;
	if let Some(edges) = mappings.get(color) {
		for (n, edge) in edges {
			subsum += traverse(edge, *n, mappings);
		}
	}
	subsum * n
}

pub fn part2(input: String) -> String {
	let mappings = forward_mappings(&input);
	// println!("{:#?}", mappings);
	(traverse("shiny gold", 1, &mappings) - 1).to_string()
}


const SAMPLE: &str = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"#;

#[test]
fn test_sample() {
	assert_eq!("4", part1(SAMPLE.to_string()));
}

const SAMPLE2: &str = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
"#;

#[test]
fn test_sample2() {
	assert_eq!("1", part2("shiny gold bags contain 1 bright white bag".to_owned()));
	assert_eq!("5", part2("shiny gold bags contain 5 bright white bags".to_owned()));
	assert_eq!("7", part2("shiny gold bags contain 5 bright white bags, 2 dark gray bags.".to_owned()));
	assert_eq!("126", part2(SAMPLE2.to_string()));
}