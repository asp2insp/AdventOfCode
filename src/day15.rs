use defaultmap::DefaultHashMap;
use itertools::*;
use std::collections::{VecDeque, HashSet};
use crate::intcode::*;

const NORTH: isize = 1;
const SOUTH: isize = 2;
const WEST: isize = 3;
const EAST: isize = 4;

static FORWARD_OFFSETS_X: [isize; 5] = [42, 0, 0, -1, 1];
static FORWARD_OFFSETS_Y: [isize; 5] = [42, -1, 1, 0, 0];

static RH_OFFSETS_X: [isize; 5] = [42, 1, -1, 0, 0];
static RH_OFFSETS_Y: [isize; 5] = [42, 0, 0, -1, 1];

static RH: [isize; 5] = [42, EAST, WEST, NORTH, SOUTH];
static LH: [isize; 5] = [42, WEST, EAST, SOUTH, NORTH];

struct RhBot {
	map: DefaultHashMap<(isize, isize), char>,
	x: isize,
	y: isize,
	dir: isize,
	computer: Computer,
}

impl RhBot {
	fn new(c: Computer) -> RhBot {
		RhBot {
			map: DefaultHashMap::new('?'),
			x: 0,
			y: 0,
			dir: EAST,
			computer: c,
		}
	}

	fn draw_map(&self, marks: &HashSet<(isize, isize)>) -> String {
		let (min_x, min_y, max_x, max_y) =
			self.map
				.keys()
				.fold((0, 0, 0, 0), |(min_x, min_y, max_x, max_y), (x, y)| {
					(min_x.min(*x), min_y.min(*y), max_x.max(*x), max_y.max(*y))
				});
		(min_y - 1..=max_y + 1)
			.map(|y| {
				(min_x - 1..=max_x + 1)
					.map(|x| {
						if marks.contains(&(x, y)) {
							'x'
						} else if (x, y) == (self.x, self.y) {
							self.draw_self()
						} else {
							*self.map.get(&(x, y))
						}
					})
					.collect::<String>()
			})
			.join("\n")
	}

	fn get_rh_rule_move(&self) -> isize {
		let rh = *self.map.get(&(self.x + RH_OFFSETS_X[self.dir as usize], self.y + RH_OFFSETS_Y[self.dir as usize]));
		let fw = *self.map.get(&(self.x + FORWARD_OFFSETS_X[self.dir as usize], self.y + FORWARD_OFFSETS_Y[self.dir as usize]));

		match (fw, rh) {
			// We hit a corner, go back right to find wall
			(_, r) if r != '#' => RH[self.dir as usize],
			// Wall ahead, turn left.
			('#', _) => LH[self.dir as usize],
			// Keep marching on with RH on wall
			(_, '#') => self.dir,
			// Empty space to right... keep soldering on
			(_, '?') => self.dir,
			other => panic!("Betcha didn't think of {:?}", other),
		}
	}

	fn draw_self(&self) -> char {
		match self.dir {
			NORTH => '^',
			SOUTH => 'v',
			EAST => '>',
			WEST => '<',
			_ => unreachable!(),
		}
	}

	fn next_step(&mut self) -> bool {
		self.dir = self.get_rh_rule_move();
		self.computer.input(self.dir);
		if let ProgYield::Output(t) = self.computer.run_and_return_output() {
			let ahead_xy = (self.x + FORWARD_OFFSETS_X[self.dir as usize], self.y + FORWARD_OFFSETS_Y[self.dir as usize]);
			match t {
				0 => {
					self.map.insert(ahead_xy, '#');
				},
				1 => {
					self.map.insert(ahead_xy, ' ');
					self.x = ahead_xy.0;
					self.y = ahead_xy.1;
				},
				2 => {
					self.map.insert(ahead_xy, '@');
					self.x = ahead_xy.0;
					self.y = ahead_xy.1;
					return true
				},
				_ => unreachable!(),
			};
		}
		return false;
	}
}

fn bfs((x, y): (isize, isize), target_x: isize, target_y: isize, marks: &mut HashSet<(isize, isize)>, bot: &RhBot) -> Option<usize> {
	marks.insert((x, y));
	if (x, y) == (target_x, target_y) {
		return Some(0);
	}
	// print!("\x1B[2J");
	// print!("{}", bot.draw_map(marks));
	// print!("{}", bot.draw_map(None));
	[NORTH, SOUTH, EAST, WEST].into_iter()
		.filter_map(|dir| {
			let ahead_xy = (x + FORWARD_OFFSETS_X[*dir as usize], y + FORWARD_OFFSETS_Y[*dir as usize]);
			if *bot.map.get(&ahead_xy) == '#' || marks.contains(&ahead_xy) {
				None
			} else {
				bfs(ahead_xy, target_x, target_y, marks, bot).map(|steps| steps + 1)
			}
		})
		.min()
}

pub fn part1(input: String) -> String {
	let mut bot = RhBot::new(Computer::new(parse_program(input)));
	for _ in 0..55_000 {
		bot.next_step();
	}
	// print!("{}", bot.draw_map(None));
	let (target_x, target_y) = *bot.map.iter().find(|(_, c)| **c == '@').map(|(coords, _)| coords).unwrap();
	bfs((0, 0), target_x, target_y, &mut HashSet::new(), &bot).unwrap().to_string()
}

pub fn part2(input: String) -> String {
	let mut bot = RhBot::new(Computer::new(parse_program(input)));
	for _ in 0..55_000 {
		bot.next_step();
	}
	let mut open_spaces = bot.map.iter().filter_map(|(coords, c)| if *c == ' ' { Some(*coords) } else { None }).collect::<HashSet<_>>();
	let mut front = HashSet::new();
	front.insert((-16, -20));
	let mut minutes = 0;
	while !open_spaces.is_empty() {
		front = front.into_iter()
			.flat_map(|(x, y)| [NORTH, SOUTH, EAST, WEST].into_iter()
				.map(move |dir| (x + FORWARD_OFFSETS_X[*dir as usize], y + FORWARD_OFFSETS_Y[*dir as usize])))
			.filter(|p| open_spaces.remove(p))
			.collect();
		minutes += 1;
	}
	minutes.to_string()
}
