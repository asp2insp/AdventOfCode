use itertools::*;
use std::collections::{HashMap, HashSet};
use Side::*;

struct Tile {
	data: [[char; 10]; 10],
	id: usize,
}

const NULL_TILE: Tile = Tile {
	data: [['X'; 10]; 10],
	id: 0,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Side {
	N,
	S,
	E,
	W,
}

impl Side {
	fn rot(&self, angle: usize) -> Side {
		let mut res = *self;
		for _ in 0..(angle / 90) {
			res = res.rot90();
		}
		res
	}

	fn rot90(&self) -> Side {
		match self {
			N => E,
			E => S,
			S => W,
			W => N,
		}
	}
}

impl Tile {
	fn edge(&self, s: Side, rf: &RotFlip) -> Vec<char> {
		let side = s.rot(rf.rot);
		let mut res: Vec<char> = match side {
			N => self.data[0].iter().cloned().collect(),
			S => self.data[9].iter().cloned().collect(),
			E => (0..10).map(|i| self.data[i][9]).collect(),
			W => (0..10).map(|i| self.data[i][0]).collect(),
		};
		if rf.flip_y && (s == E || s == W) {
			res.reverse();
		} else if rf.flip_x && (s == N || s == S) {
			res.reverse();
		}
		res
	}

	fn all_edges(&self) -> impl Iterator<Item = (Side, Vec<char>)> + '_ {
		[N, S, E, W]
			.iter()
			.map(move |s| (*s, self.edge(*s, &RotFlip::default())))
	}

	fn outer_sides(&self, others: &HashMap<usize, Tile>) -> Vec<Side> {
		self.all_edges()
			.filter_map(|(side, s)| if let Some(_) = find_matching_side(self.id, &s, others) {None} else {Some(side)})
			.collect()
	}
}

fn parse_tiles(s: &str) -> HashMap<usize, Tile> {
	let mut res = HashMap::new();
	let mut lines = s.lines();
	while let Some(l) = lines.next() {
		if l.is_empty() {
			continue;
		}
		let id = l
			.split_whitespace()
			.nth(1)
			.unwrap()
			.trim_matches(':')
			.parse::<usize>()
			.unwrap();
		let mut t = Tile {
			id: id,
			data: [[' '; 10]; 10],
		};
		for y in 0..10 {
			let mut c = lines.next().unwrap().chars();
			for x in 0..10 {
				t.data[y][x] = c.next().unwrap();
			}
		}
		res.insert(t.id, t);
	}
	res
}

fn find_matching_side(
	id: usize,
	side: &[char],
	all: &HashMap<usize, Tile>,
) -> Option<(usize, Side)> {
	for t in all.values() {
		if t.id == id {
			continue;
		}
		for (s, e) in t.all_edges() {
			if e == side || e.iter().eq(side.iter().rev()) {
				// println!("{}-{:?} matches {}/{:?}-{:?}", id, side, t.id, s, side);
				return Some((t.id, s));
			}
		}
	}
	None
}

fn find_corners(all: &HashMap<usize, Tile>) -> Vec<&Tile> {
	all.values()
		.filter(|t| t.outer_sides(all).len() == 2)
		.collect()
}

fn find_edges(all: &HashMap<usize, Tile>) -> Vec<&Tile> {
	all.values()
		.filter(|t| t.outer_sides(all).len() == 3)
		.collect()
}

#[derive(Default, Copy, Clone, Debug)]
struct RotFlip {
	rot: usize,
	flip_x: bool,
	flip_y: bool,
}

fn all_orientations() -> impl Iterator<Item = RotFlip> {
	[0, 90, 180, 270]
		.iter()
		.cartesian_product([true, false].iter())
		.cartesian_product([true, false].iter())
		.map(move |((r, x), y)| RotFlip {
			rot: *r,
			flip_x: *x,
			flip_y: *y,
		})
}

fn find_rotflip_for_constraints(t: &Tile, left: &[char], top: &[char]) -> Option<RotFlip> {
	all_orientations()
		.filter(|rf| {
			let left_edge = t.edge(W, rf);
			let top_edge = t.edge(N, rf);
			if !left.is_empty() && left != left_edge {
				false
			} else if !top.is_empty() && top != top_edge {
				false
			} else {
				true
			}
		})
		.next()
}

pub fn part1(input: String) -> String {
	let tiles = parse_tiles(&input);
	find_corners(&tiles)
		.into_iter()
		.map(|t| t.id)
		.product::<usize>()
		.to_string()
}

fn find_next<'a>(is: impl Iterator<Item=&'a Tile>, left: &[char], top: &[char]) -> (&'a Tile, RotFlip) {
	is.filter_map(|t| find_rotflip_for_constraints(t, left, top).map(|rf| (t, rf))).next().unwrap()
}

pub fn part2(input: String) -> String {
	let tiles = parse_tiles(&input);
	let mut corners = find_corners(&tiles);
	let mut board = [[(&NULL_TILE, RotFlip::default()); 12]; 12];
	let mut used = HashSet::new();
	// Top left corner
	let tl = corners.pop().unwrap();
	let outers = tl.outer_sides(&tiles);
	println!("{}, {:?}", tl.id, outers);
	let top_left_orient = find_rotflip_for_constraints(
		tl,
		&tl.edge(outers[0], &RotFlip::default()),
		&tl.edge(outers[1], &RotFlip::default()),
	)
	.unwrap();
	board[0][0] = (tl, top_left_orient);
	used.insert(tl.id);

	// Top Row Middle
	let edges = find_edges(&tiles);
	for x in 1..9 {
		println!("0, {}/{}", x, edges.len());
		board[0][x] = find_next(edges.iter().map(|i| *i), &board[0][x-1].0.edge(E, &board[0][x-1].1), &[]);
	}
	"hi".to_owned()
}
