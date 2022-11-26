use std::collections::{HashMap, HashSet};

fn parse_item(start_point: (isize, isize), item: &str) -> Box<dyn Iterator<Item = (isize, isize)>> {
	let dir = item.chars().next().unwrap();
	let amount = item
		.chars()
		.skip(1)
		.collect::<String>()
		.parse::<isize>()
		.unwrap();
	match dir {
		'R' => Box::new((start_point.0..=start_point.0 + amount).map(move |x| (x, start_point.1))),
		'L' => Box::new(
			(start_point.0 - amount..=start_point.0)
				.rev()
				.map(move |x| (x, start_point.1)),
		),
		'U' => Box::new((start_point.1..=start_point.1 + amount).map(move |y| (start_point.0, y))),
		'D' => Box::new(
			(start_point.1 - amount..=start_point.1)
				.rev()
				.map(move |y| (start_point.0, y)),
		),
		_ => unreachable!(),
	}
}

fn parse_path(path: &str) -> (HashSet<(isize, isize)>, HashMap<(isize, isize), usize>) {
	let mut pt = (0, 0);
	let mut ret = (HashSet::new(), HashMap::new());
	ret.0.insert(pt);
	let mut dist = 0;
	for item in path.split(",") {
		for np in parse_item(pt, item) {
			pt = np;
			ret.0.insert(pt);
			if !ret.1.contains_key(&pt) {
				ret.1.insert(pt, dist);
			}
			dist += 1;
		}
		dist -= 1;
	}
	ret.0.remove(&(0, 0));
	ret
}

pub fn part1(input: String) -> String {
	let paths = input
		.lines()
		.map(parse_path)
		.map(|(p, _)| p)
		.collect::<Vec<_>>();
	let answer = paths[0]
		.intersection(&paths[1])
		.min_by_key(|pt| pt.0.abs() + pt.1.abs())
		.unwrap();
	format!("{:?}: {}", *answer, answer.0.abs() + answer.1.abs())
}

pub fn part2(input: String) -> String {
	let paths = input
		.lines()
		.map(parse_path)
		.collect::<Vec<_>>();
	paths[0].0
		.intersection(&paths[1].0)
		.map(|pt| paths[0].1.get(pt).unwrap() + paths[1].1.get(pt).unwrap())
		.min()
		.unwrap()
		.to_string()
}
