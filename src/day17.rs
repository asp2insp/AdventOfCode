use std::collections::{HashSet,HashMap};

fn parse(s: &str) -> HashMap<(isize, isize, isize), char> {
	s.lines().enumerate()
		.flat_map(|(y, l)| l.chars().enumerate().map( move |(x,c)| ((x as isize, y as isize, 0isize), c)))
		.collect()
}

fn adjacencies((x, y, z): &(isize, isize, isize)) -> Vec<(isize, isize, isize)> {
	let mut res = Vec::with_capacity(26);
	for xoff in &[-1, 0, 1] {
		for yoff in &[-1, 0, 1] {
			for zoff in &[-1, 0, 1] {
				if *xoff == 0 && *yoff == 0 && *zoff == 0 {
					continue
				}
				res.push((x + xoff, y + yoff, z + zoff));
			}
		}
	}
	res
}

fn run_cycle(map: &HashMap<(isize, isize, isize), char>) -> HashMap<(isize, isize, isize), char> {
	let new_coords: HashSet<(isize, isize, isize)> = map.keys().flat_map(|p| adjacencies(p).into_iter()).chain(map.keys().cloned()).collect();
	let mut new_map = HashMap::new();
	for (pnew, c) in new_coords.iter().map(|p| (p, map.get(p).unwrap_or(&'.'))) {
		let num = adjacencies(&pnew).into_iter().filter(|p| map.get(p).map(|c| *c == '#').unwrap_or(false)).count();
		if *c == '.' && num == 3 {
			new_map.insert(*pnew, '#');
		} else if *c == '#' && (num == 2 || num == 3) {
			new_map.insert(*pnew, '#');
		}
	}
	new_map
}

pub fn part1(input: String) -> String {
	let mut map = parse(&input);
	for _ in 0..6 {
		map = run_cycle(&map);
	}
	map.values().filter(|c| **c == '#').count().to_string()
}


fn parse2(s: &str) -> HashMap<(isize, isize, isize, isize), char> {
	s.lines().enumerate()
		.flat_map(|(y, l)| l.chars().enumerate().map( move |(x,c)| ((x as isize, y as isize, 0isize, 0isize), c)))
		.collect()
}

fn adjacencies2((x, y, z, w): &(isize, isize, isize, isize)) -> Vec<(isize, isize, isize, isize)> {
	let mut res = Vec::with_capacity(26);
	for xoff in &[-1, 0, 1] {
		for yoff in &[-1, 0, 1] {
			for zoff in &[-1, 0, 1] {
				for woff in &[-1, 0, 1] {
					if *xoff == 0 && *yoff == 0 && *zoff == 0 && *woff == 0 {
						continue
					}
					res.push((x + xoff, y + yoff, z + zoff, w + woff));
				}
			}
		}
	}
	res
}

fn run_cycle2(map: &HashMap<(isize, isize, isize, isize), char>) -> HashMap<(isize, isize, isize, isize), char> {
	let new_coords: HashSet<(isize, isize, isize, isize)> = map.keys().flat_map(|p| adjacencies2(p).into_iter()).chain(map.keys().cloned()).collect();
	let mut new_map = HashMap::new();
	for (pnew, c) in new_coords.iter().map(|p| (p, map.get(p).unwrap_or(&'.'))) {
		let num = adjacencies2(&pnew).into_iter().filter(|p| map.get(p).map(|c| *c == '#').unwrap_or(false)).count();
		if *c == '.' && num == 3 {
			new_map.insert(*pnew, '#');
		} else if *c == '#' && (num == 2 || num == 3) {
			new_map.insert(*pnew, '#');
		}
	}
	new_map
}

pub fn part2(input: String) -> String {
	let mut map = parse2(&input);
	for _ in 0..6 {
		map = run_cycle2(&map);
	}
	map.values().filter(|c| **c == '#').count().to_string()
}


const TEST: &'static str = r".#.
..#
###";

#[test]
fn test() {
	let mut map = parse(&TEST);
	for _ in 0..6 {
		map = run_cycle(&map);
		// println!("{:?}", map);
	}
	let final_count = map.values().filter(|c| **c == '#').count();
	assert_eq!(112, final_count);
}