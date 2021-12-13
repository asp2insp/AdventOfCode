use itertools::*;
use std::collections::HashSet;

fn fold(coords: impl Iterator<Item=(usize, usize)>, (xy, line): (&str, usize)) -> HashSet<(usize, usize)> {
	let mut ret = makeset![];
	for c in coords {
		match (xy, c.0 < line, c.1 < line) {
			("x", true, _) => {
				ret.insert(c);
			},
			("x", false, _) => {
				ret.insert((line * 2 - c.0, c.1));
			},
			("y", _, true) => {
				ret.insert(c);
			},
			("y", _, false) => {
				ret.insert((c.0, line * 2 - c.1));
			},
			_ => unreachable!(),
		};
	}
	ret
}

fn parse(s: &str) -> (Vec<(usize, usize)>, Vec<(&str, usize)>) {
	let mut coords = s.lines().take_while(|l| !l.trim().is_empty())
		.map(|l| {
			let mut sp = l.split(",");
			(parse!(sp.next().unwrap(), usize), parse!(sp.next().unwrap(), usize))
		})
		.collect_vec();
	let folds = s.lines().skip_while(|l| !l.starts_with("fold"))
		.map(|l| l.split_whitespace().collect_vec()[2])
		.map(|f| {
			let mut sp = f.split("=");
			(sp.next().unwrap(), parse!(sp.next().unwrap(), usize))
		})
		.collect_vec();
	(coords, folds)
}

pub fn part1(input: String) -> String {
	let (coords, folds) = parse(&input);
	fold(coords.into_iter(), folds[0]).len().to_string()
}


pub fn part2(input: String) -> String {
	let (coords, folds) = parse(&input);
	let mut set = coords.into_iter().collect::<HashSet<_>>();
	let set = folds.into_iter().fold(set, |set, f| fold(set.into_iter(), f));
	let max_y = *set.iter().map(|(x, y)| y).max().unwrap();
	let max_x = *set.iter().map(|(x, y)| x).max().unwrap();
	let res = (0..=max_y).map(|line| (0..=max_x).map(|x| if set.contains(&(x, line)) {"#"} else {" "}).collect::<String>()).join("\n");
	println!("{}", res);
	res
}
