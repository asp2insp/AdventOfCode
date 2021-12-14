use itertools::*;
use std::collections::HashMap;
use crate::utils::*;

fn parse(s: &str) -> (&str, HashMap<&str, &str>) {
	let mut l = s.lines();
	let poly = l.next().unwrap();
	(poly, l.skip(1).flat_map(|s| s.split(" -> ").collect_tuple::<(_,_)>()).collect::<HashMap<_,_>>())
}

fn run_step(s: String, t: &HashMap<&str, &str>) -> String {
	s.chars()
		.tuple_windows::<(_, _)>()
		.map(|(a,b)| {
			let c = t[format!("{}{}", a,b).as_str()];
			format!("{}{}", a, c)
		})
		.chain(s.chars().last().map(|c| format!("{}", c)).into_iter())
		.join("")
}

pub fn part1(input: String) -> String {
	let (poly, transitions) = parse(&input);
	let mut p = poly.to_string();
	for _ in 0..10 {
		// println!("{}", p);
		p = run_step(p, &transitions);
	}
	let counts = p.chars().counting_set();
	format!("{}", counts.values().max().unwrap() - counts.values().min().unwrap())
}


pub fn part2(input: String) -> String {
	let (poly, transitions) = parse(&input);
	let mut p = poly.to_string();
	for _ in 0..40 {
		// println!("{}", p);
		p = run_step(p, &transitions);
	}
	let counts = p.chars().counting_set();
	format!("{}", counts.values().max().unwrap() - counts.values().min().unwrap())
}
