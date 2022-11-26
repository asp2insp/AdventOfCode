use crate::utils::*;
use itertools::Itertools;
use std::collections::{HashMap};
use std::collections::BinaryHeap;
use fnv::FnvHashMap;

// #############
// #...........#
// ###A#D#A#C###
//   #C#D#B#B#
//   #########
//  1 3 5 7 9

const A: isize = 3;
const B: isize = 5;
const C: isize = 7;
const D: isize = 9;

type G = Grid<()>;

fn dist(pf: Point, pt: Point, g: &G) -> Option<isize> {
	g.bfs_generic(makeset![pf], Some(&|p| g.neighbors(p).filter(|n| g.get(*n).unwrap().0 == '.').map(|n| (n, 1)).collect_vec()), None).get(&pt).map(|tup| tup.0)
}

fn cost(g: &G, &(pf, pt): &(Point, Point)) -> Option<isize> {
	let factor = match g.read(pf.x, pf.y) {
		'A' => 1,
		'B' => 10,
		'C' => 100,
		'D' => 1000,
		_ => unreachable!(),
	};
	dist(pf, pt, g).map(|c| c * factor)
}

fn is_avail(g: &G, x: isize, compat: char, hall_level: isize) -> bool {
	(1..hall_level).all(|y| g.read(x, y) == '.' || g.read(x, y) == compat)
}

fn is_done(g: &G, hall_level: isize) -> bool {
	let mut done = true;
	for hall in [(A, 'A'), (B, 'B'), (C, 'C'), (D, 'D')] {
		for y in 1..hall_level {
			done &= g.read(hall.0, y) == hall.1;
			if !done {
				return false
			}
		}
	}
	done
}

fn candidates(g: &G, hl: isize) -> impl Iterator<Item=(Point, Point)> {
	g.iter_range(None, None)
		.filter(|&(_, c, _)| c != '#' && c != '.') // From has to be a letter
		.filter(|&(pf, cf, _)| match (pf.x, cf) {
			// Don't move things in their home unless they're trapping someone else
			(A, 'A') => (1..pf.y).any(|y| g.read(A, y) != 'A'),
			(B, 'B') => (1..pf.y).any(|y| g.read(B, y) != 'B'),
			(C, 'C') => (1..pf.y).any(|y| g.read(C, y) != 'C'),
			(D, 'D') => (1..pf.y).any(|y| g.read(D, y) != 'D'),
			_ => true
		})
		.flat_map(|(pf, c, _)| 
			std::iter::repeat((pf, c))
				.zip(g.iter_range(None, None)
						// To is an empty space
						.filter(|&(_, c, _)| c == '.')
						.map(|(pt, cc, _)| (pt, cc))
						// Don't go to the top of the hall if the bottom is empty
						.filter(|&(pt, _)| {
							pt.y == hl || !(1..pt.y).any(|y| g.read(pt.x, y) == '.')
						})
						// Don't go to the space on top of a home hall
						.filter(|&(pt, _)| {
							pt.y != hl || ![A, B, C, D].contains(&pt.x)
						})
				)
			)
		.filter(|((pf, cf), (pt, ct))| {
			// Don't move hall -> hall
			!(pt.y == hl && pf.y == hl)
		})
		.filter(|((pf, cf), (pt, ct))| match (pt.y, cf) {
			// Don't enter your home unless your home is available
			(y, 'A') if y < hl => is_avail(g, A, 'A', hl),
			(y, 'B') if y < hl => is_avail(g, B, 'B', hl),
			(y, 'C') if y < hl => is_avail(g, C, 'C', hl),
			(y, 'D') if y < hl => is_avail(g, D, 'D', hl),
			_ => true,
		})
		.filter(|((pf, cf), (pt, ct))| match (cf, pt.x) {
			// Don't go into home that's not yours
			('A', x) if pt.y < hl => x == A,
			('B', x) if pt.y < hl => x == B,
			('C', x) if pt.y < hl => x == C,
			('D', x) if pt.y < hl => x == D,
			_ => true,
		})
		.map(|((pf, _), (pt, _))| (pf, pt))
		.collect_vec()
		.into_iter()
}

#[derive(PartialEq, Eq)]
struct State(isize, G);

impl Ord for State {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.0.cmp(&other.0).reverse()
	}
}

impl PartialOrd for State {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(&other))
	}
}

fn min_cost(g: G, hl: isize) -> isize {
	let mut q: BinaryHeap<State> = BinaryHeap::new();
	let mut hoard: FnvHashMap<G, (isize, G)> = FnvHashMap::with_capacity_and_hasher(250_000, Default::default());
	q.push(State(0, g));
	while let Some(State(c, g)) = q.pop() {
		if is_done(&g, hl) {
			println!("Done with {} unique states. Cost {}", hoard.len(), c);
			let mut hist = g;
			#[cfg(test)]
			while let Some((hc, hg)) = hoard.get(&hist) {
				println!("Hist: {}\n{}", hc, hg.to_string());
				hist = hg.clone();
			}
			return c;
		} else {
			for (pf, pt) in candidates(&g, hl) {
				if let Some(nc) = cost(&g, &(pf, pt)) {
					let mut ng = g.clone();
					ng.swap(pf, pt);
					// println!("{}", ng);
					let new_total_cost = c.saturating_add(nc);
					if new_total_cost < hoard.get(&ng).map(|t| t.0).unwrap_or(isize::MAX) {
						hoard.insert(ng.clone(), (new_total_cost, g.clone()));
						q.push(State(new_total_cost, ng));
					}
				}
			}
		}
	}
	unreachable!()
}

pub fn part1(input: String) -> String {
	let mut start = Grid::new(&input, ());
	start.wall_char = '#';
	min_cost(start, 3).to_string()
}


pub fn part2(input: String) -> String {
	let mut s = input.lines().collect_vec();
	s.insert(3, "###D#B#A#C###");
	s.insert(3, "###D#C#B#A###");
	let mut g = Grid::new(&s.into_iter().join("\n"), ());
	g.wall_char = '#';
	// println!("{}", g.to_string());
	min_cost(g, 5).to_string()
}


#[test]
fn test_cand() {
	let init = 
  r"#############
	#.B........D#
	###A#.#C#.###
	###A#B#C#D###
	#############";
	let mut g = Grid::new(init, ());
	g.wall_char = '#';
	assert_eq!(false, is_done(&g, 3));
	assert_eq!(makeset![
		(Point::from((11, 3)), Point::from((9, 2))),
		(Point::from((2, 3)), Point::from((5, 2))),
	], candidates(&g, 3).collect());
	g.swap(Point::from((2, 3)), Point::from((5, 2)));
	assert_eq!(makeset![
		(Point::from((11, 3)), Point::from((9, 2))),
	], candidates(&g, 3).collect());
	assert_eq!(Some(3000), cost(&g, &(Point::from((11, 3)), Point::from((9, 2)))));
}

#[test]
fn test_done() {
	let g = Grid::new(
	"#############
		#...........#
		###A#B#C#D###
		###A#B#C#D###
		#############", ());
	assert_eq!(true, is_done(&g, 3));
	assert_eq!(0, min_cost(g, 3));
}

#[test]
fn test_one() {
	let g = Grid::new(
	 "#############
		#..........D#
		###A#B#C#.###
		###A#B#C#D###
		#############", ());
	assert_eq!(false, is_done(&g, 3));
	assert_eq!(3000, min_cost(g, 3));
}

#[test]
fn test_example_two() {
	let mut g = Grid::new(
		r"#...........#
		###B#C#B#D###
		###D#C#B#A###
		###D#B#A#C###
		###A#D#C#A###
		#############", 
		());
	g.wall_char = '#';
	assert_eq!(44169, min_cost(g, 5));
}