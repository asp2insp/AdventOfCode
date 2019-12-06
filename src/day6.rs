use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};

fn parse_item(item: &str) -> (&str, &str) {
	let mut caps = item.split(")");
	let inner = caps.next().unwrap();
	let outer = caps.next().unwrap();
	(outer, inner)
}

fn find_orbiting<'a>(inner: &str, orbits: &'a[(&str, &str)]) -> Vec<&'a str> {
	orbits.iter()
		.filter(|(outer, i)| *i == inner)
		.map(|(outer, _)| *outer)
		.collect()
}

fn count_transitive_orbits<'a>(orbits: &'a [(&'a str, &'a str)]) -> HashMap<&'a str, usize> {
	let mut done: HashSet<&str> = HashSet::new();
	let mut todo = vec!["COM"];
	let mut totals: HashMap<&str, usize> = HashMap::new();
	while !todo.is_empty() {
		println!("TODO: {:?}", todo);
		done.extend(todo.iter());
		let mut next = vec![];
		for n in todo {
			let current = totals.get(&n).cloned().unwrap_or(1);
			for orb in find_orbiting(n, &orbits) {
				*totals.entry(orb).or_insert(0) += current;
				next.push(orb);
			}
		}
		todo = next;
	}
	totals
}

pub fn part1(input: String) -> String {
	let orbits = input.lines()
		.map(parse_item)
		.collect::<Vec<_>>();
	let totals = count_transitive_orbits(&orbits);
	format!("{}", totals.values().sum::<usize>())
}


pub fn part2(input: String) -> String {
	"part2".to_string()
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
	let orbits = input.lines()
		.map(parse_item)
		.collect::<Vec<_>>();
	let totals = count_transitive_orbits(&orbits);
	println!("{:#?}", totals);
	assert_eq!(42, totals.values().sum::<usize>());
}