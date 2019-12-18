use defaultmap::DefaultHashMap;
use itertools::*;
use std::collections::{VecDeque, HashSet};
use std::thread;

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

fn parse_program(input: String) -> Vec<isize> {
	input.split(",").flat_map(str::parse::<isize>).collect()
}

macro_rules! make_input [
	($($e:expr),*) => {
		vec![$($e),*].into_iter().collect::<VecDeque>()
	};
];

#[derive(PartialEq, Debug)]
enum ProgYield {
	Output(isize),
	Input,
	Halt,
}

impl ProgYield {
	fn unwrap(self) -> isize {
		match self {
			ProgYield::Output(i) => i,
			_ => panic!("unwrap called on a halt/input instruction"),
		}
	}
}

struct Computer {
	program: Vec<isize>,
	iptr: usize,
	input: VecDeque<isize>,
	relative_base: isize,
}

impl Computer {
	fn new(prog: Vec<isize>) -> Computer {
		Computer {
			program: prog,
			iptr: 0,
			input: VecDeque::new(),
			relative_base: 0,
		}
	}

	fn input(&mut self, item: isize) {
		self.input.push_back(item);
	}

	fn set(&mut self, loc: usize, item: isize) {
		self.program[loc] = item;
	}

	fn check_dest(&mut self, offset: usize, param_modes: isize) -> usize {
		let mode = param_modes / 10_isize.pow(offset as u32 - 1);
		let dest = (self.program[self.iptr + offset]
			+ if mode == 2 { self.relative_base } else { 0 }) as usize;
		if dest >= self.program.len() {
			self.program.resize(dest + 100, 0);
		}
		dest
	}

	fn read_param(&self, loc: usize, mode: isize) -> isize {
		let prog_value = self.program[loc];
		match mode {
			// Position mode
			0 => {
				if prog_value as usize > self.program.len() {
					0
				} else {
					self.program[prog_value as usize]
				}
			}
			// Immediate mode
			1 => prog_value,
			// Relative mode
			2 => {
				let addr = (self.relative_base + prog_value) as usize;
				if addr > self.program.len() {
					0
				} else {
					self.program[addr]
				}
			}
			_ => unreachable!("Uknown param mode"),
		}
	}

	fn get_params(&self, n: usize, mut param_modes: isize) -> Vec<isize> {
		let mut ret = Vec::new();
		for i in 1..=n {
			let mode = param_modes % 10;
			ret.push(self.read_param(self.iptr + i, mode));
			param_modes = param_modes / 10;
		}
		ret
	}

	fn run_and_return_output(&mut self) -> ProgYield {
		loop {
			let opcode = self.program[self.iptr] % 100;
			let param_modes = self.program[self.iptr] / 100;
			match opcode {
				1 => {
					// Add 1, 2, store 3
					let params = self.get_params(2, param_modes);
					let dest = self.check_dest(3, param_modes);
					self.program[dest] = params[0] + params[1];
					self.iptr += 4;
				}
				2 => {
					// Mult 1, 2, store 3
					let params = self.get_params(2, param_modes);
					let dest = self.check_dest(3, param_modes);
					self.program[dest] = params[0] * params[1];
					self.iptr += 4;
				}
				3 => {
					// Input and store 1
					let dest = self.check_dest(1, param_modes);
					if self.input.is_empty() {
						return ProgYield::Input;
					}
					self.program[dest] = self.input.pop_front().unwrap();
					self.iptr += 2;
				}
				4 => {
					// Output 1
					let params = self.get_params(1, param_modes);
					self.iptr += 2;
					return ProgYield::Output(params[0]);
				}
				5 => {
					// JNZ 1 to 2
					let params = self.get_params(2, param_modes);
					if params[0] != 0 {
						self.iptr = params[1] as usize;
					} else {
						self.iptr += 3;
					}
				}
				6 => {
					// JEZ 1 to 2
					let params = self.get_params(2, param_modes);
					if params[0] == 0 {
						self.iptr = params[1] as usize;
					} else {
						self.iptr += 3;
					}
				}
				7 => {
					// 1 LT 2, store 3
					let params = self.get_params(2, param_modes);
					let dest = self.check_dest(3, param_modes);
					self.program[dest] = if params[0] < params[1] { 1 } else { 0 };
					self.iptr += 4;
				}
				8 => {
					// 1 EQ 2, store 3
					let params = self.get_params(2, param_modes);
					let dest = self.check_dest(3, param_modes);
					self.program[dest] = if params[0] == params[1] { 1 } else { 0 };
					self.iptr += 4;
				}
				9 => {
					// Relbase adjust
					let params = self.get_params(1, param_modes);
					self.relative_base += params[0];
					self.iptr += 2;
				}
				99 => return ProgYield::Halt,
				other => unreachable!("Opcode {} unknown", other),
			}
		}
	}
}

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
