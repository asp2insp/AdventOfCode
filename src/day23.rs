use crate::utils::*;
use itertools::Itertools;
use std::collections::HashMap;

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
	g.bfs_generic(makeset![pf], Some(&|p| g.neighbors(p).filter(|n| g.read(n.x, n.y) == '.').map(|n| (n, 1)).collect_vec()), None).get(&pt).map(|tup| tup.0)
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

fn is_avail(g: &G, x: isize, compat: char) -> bool {
	let slot1 = g.read(x, 1);
	let slot2 = g.read(x, 2);
	(slot1 == '.' || slot1 == compat) && (slot2 == '.' || slot2 == compat)
}

fn is_done(g: &G) -> bool {
	let mut done = true;
	for hall in [(A, 'A'), (B, 'B'), (C, 'C'), (D, 'D')] {
		done &= g.read(hall.0, 1) == hall.1 && g.read(hall.0, 2) == hall.1;
	}
	done
}

fn candidates(g: &G) -> impl Iterator<Item=(Point, Point)> {
	g.iter_range(None, None)
		.filter(|&(_, c, _)| c != '#' && c != '.')
		.flat_map(|(pf, c, _)| 
			std::iter::repeat((pf, c))
				.zip(g.iter_range(None, None)
						.filter(|&(_, c, _)| c == '.')
						.map(|(pt, cc, _)| (pt, cc))
						.filter(|(pt, _)| pt.y != 3 || (pt.x != A && pt.x != B && pt.x != C && pt.x != D))
				)
			)
		.filter(|((pf, cf), (pt, ct))| match (pf.x, pf.y, cf) {
			// Things in their home
			(A, 1, 'A') => false,
			(A, 2, 'A') => g.read(A, 1) != 'A',
			(B, 1, 'B') => false,
			(B, 2, 'B') => g.read(B, 1) != 'B',
			(C, 1, 'C') => false,
			(C, 2, 'C') => g.read(C, 1) != 'C',
			(D, 1, 'D') => false,
			(D, 2, 'D') => g.read(D, 1) != 'D',

			// Things that can't go in their home
			(_, 3, 'A') => is_avail(g, A, 'A'),
			(_, 3, 'B') => is_avail(g, B, 'B'),
			(_, 3, 'C') => is_avail(g, C, 'C'),
			(_, 3, 'D') => is_avail(g, D, 'D'),
			_ => true,
		})
		.filter(|((pf, cf), (pt, ct))| match (cf, pt.x) {
			('A', A) if pt.y < 3 => true,
			('A', _) if pt.y < 3 => false,
			('B', B) if pt.y < 3 => true,
			('B', _) if pt.y <3 => false,
			('C', C) if pt.y < 3 => true,
			('C', _) if pt.y < 3 => false,
			('D', D) if pt.y < 3 => true,
			('D', _) if pt.y < 3 => false,
			(_, _) => (pt.y == 3) ^ (pf.y == 3),
		})
		.map(|((pf, _), (pt, _))| (pf, pt))
		.collect_vec()
		.into_iter()
}

fn min_cost(g: G, hoard: &mut HashMap<G, isize>) -> isize {
	if hoard.contains_key(&g) {
		return hoard[&g]
	}
	if is_done(&g) {
		return 0
	}
	let mut minval = isize::MAX;
	for (pf, pt) in candidates(&g) {
		if let Some(c) = cost(&g, &(pf, pt)) {
			let mut ng = g.clone();
			println!("{}", ng.to_string());
			ng.swap(pf, pt);
			minval = minval.min(c.saturating_add(min_cost(ng, hoard)));
		}
	}
	hoard.insert(g, minval);
	minval
}

pub fn part1(input: String) -> String {
	let mut start = Grid::new(&input, ());
	start.wall_char = '#';
	min_cost(start, &mut HashMap::new()).to_string()
}


pub fn part2(input: String) -> String {
	"part2".to_string()
}
