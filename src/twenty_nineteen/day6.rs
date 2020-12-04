use rayon::prelude::*;
use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet, VecDeque};

fn parse_item(item: &str) -> (&str, &str) {
	let mut caps = item.split(")");
	let inner = caps.next().unwrap();
	let outer = caps.next().unwrap();
	(outer, inner)
}

fn find_orbiting<'a>(inner: &str, orbits: &'a [(&str, &str)]) -> Vec<&'a str> {
	orbits
		.iter()
		.filter(|(outer, i)| *i == inner)
		.map(|(outer, _)| *outer)
		.collect()
}

fn find_orbits<'a>(outer: &str, orbits: &'a [(&str, &str)]) -> Vec<&'a str> {
	orbits
		.iter()
		.filter(|(o, i)| *o == outer)
		.map(|(o, i)| *i)
		.collect()
}

fn count_transitive_single<'a>(start: &'a str, orbits: &'a [(&'a str, &'a str)]) -> usize {
	let mut visited = HashSet::new();
	let mut q = VecDeque::new();
	q.push_back(start);
	while !q.is_empty() {
		let next = q.pop_front().unwrap();
		if visited.contains(next) {
			continue;
		}
		visited.insert(next);
		q.extend(find_orbits(next, orbits).into_iter());
	}
	visited.len() - 1 // Don't count the planet itself
}

fn count_transitive_all<'a>(orbits: &'a [(&'a str, &'a str)]) -> HashMap<&'a str, usize> {
	let uniq: HashSet<&'a str> = orbits
		.iter()
		.flat_map(|tup| std::iter::once(tup.0).chain(std::iter::once(tup.1)))
		.collect();
	uniq.into_par_iter()
		.map(|k| (k, count_transitive_single(k, orbits)))
		.collect()
}

pub fn part1(input: String) -> String {
	let orbits = input.lines().map(parse_item).collect::<Vec<_>>();
	let totals = count_transitive_all(&orbits);
	format!("{}", totals.values().sum::<usize>())
}

fn shortest_path_len<'a>(from: &'a str, to: &'a str, orbits: &'a [(&'a str, &'a str)]) -> usize {
	let mut visited = HashSet::new();
	let mut q = VecDeque::new();
	q.push_back((from, 0));
	while !q.is_empty() {
		let (next, curr_dist) = q.pop_front().unwrap();
		if next == to {
			return curr_dist
		}
		if visited.contains(next) {
			continue;
		}
		visited.insert(next);
		q.extend(
			find_orbits(next, orbits)
				.into_iter()
				.chain(find_orbiting(next, orbits).into_iter())
				.map(|n| (n, curr_dist + 1)),
		);
	}
	panic!("No route found")
}

pub fn part2(input: String) -> String {
	let orbits = input.lines().map(parse_item).collect::<Vec<_>>();
	let from = find_orbits("YOU", &orbits)[0];
	let to = find_orbits("SAN", &orbits)[0];
	format!("{}", shortest_path_len(from, to, &orbits))
}

#[test]
fn test_simple() {
	let input = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"#;
	let orbits = input.lines().map(parse_item).collect::<Vec<_>>();
	let totals = count_transitive_all(&orbits);
	println!("{:#?}", totals);
	assert_eq!(42, totals.values().sum::<usize>());
}

// Dead code

// fn count_transitive_orbits<'a>(orbits: &'a [(&'a str, &'a str)]) -> HashMap<&'a str, usize> {
// 	let mut done: HashSet<&str> = HashSet::new();
// 	let mut todo = vec!["COM"];
// 	let mut totals: HashMap<&str, usize> = HashMap::new();
// 	while !todo.is_empty() {
// 		println!("TODO: {:?}", todo);
// 		done.extend(todo.iter());
// 		let mut next = vec![];
// 		for n in todo {
// 			let current = totals.get(&n).cloned().unwrap_or(0);
// 			for orb in find_orbiting(n, &orbits) {
// 				*totals.entry(orb).or_insert(0) += current + 1;
// 				next.push(orb);
// 			}
// 		}
// 		todo = next;
// 	}
// 	totals
// }
