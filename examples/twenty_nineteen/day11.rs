use itertools::*;
use std::collections::{BTreeMap, VecDeque};

fn parse_program(input: String) -> Vec<isize> {
	input.split(",").flat_map(str::parse::<isize>).collect()
}

#[derive(PartialEq, Debug)]
enum ProgYield {
	Output(isize),
	Halt,
}

impl ProgYield {
	fn unwrap(self) -> isize {
		match self {
			ProgYield::Output(i) => i,
			ProgYield::Halt => panic!("unwrap called on a halt instruction"),
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

const UP: (isize, isize) = (0, 1);
const DOWN: (isize, isize) = (0, -1);
const LEFT: (isize, isize) = (-1, 0);
const RIGHT: (isize, isize) = (1, 0);
const TURN_LEFT: isize = 0;
const TURN_RIGHT: isize = 1;

fn paint_hull(c: &mut Computer) -> BTreeMap<(isize, isize), isize> {
	let mut hull: BTreeMap<(isize, isize), isize> = BTreeMap::new();
	let mut x = 1000isize;
	let mut y = 1000isize;
	let mut dir = (0isize, 1isize);
	while let ProgYield::Output(paint) = c.run_and_return_output() {
		hull.insert((x, y), paint);
		dir = match (dir, c.run_and_return_output().unwrap()) {
			(UP, TURN_LEFT) => LEFT,
			(UP, TURN_RIGHT) => RIGHT,
			(DOWN, TURN_LEFT) => RIGHT,
			(DOWN, TURN_RIGHT) => LEFT,
			(RIGHT, TURN_RIGHT) => DOWN,
			(RIGHT, TURN_LEFT) => UP,
			(LEFT, TURN_LEFT) => DOWN,
			(LEFT, TURN_RIGHT) => UP,
			_ => unreachable!(),
		};
		x += dir.0;
		y += dir.1;
		c.input(*hull.get(&(x, y)).unwrap_or(&0));
	}
	hull
}

pub fn part1(input: String) -> String {
	let mut c = Computer::new(parse_program(input));
	c.input(0);
	let hull = paint_hull(&mut c);
	hull.values().count().to_string()
}

pub fn part2(input: String) -> String {
	let mut c = Computer::new(parse_program(input));
	c.input(1);
	let hull = paint_hull(&mut c);
	let (min_x, min_y, max_x, max_y) =
		hull.keys()
			.fold((isize::max_value(), isize::max_value(), 0, 0), |curr, n| {
				(
					curr.0.min(n.0),
					curr.1.min(n.1),
					curr.2.max(n.0),
					curr.3.max(n.1),
				)
			});
	let mut output = vec![vec![0isize; (max_x + 1 - min_x) as usize]; (max_y + 1 - min_y) as usize];
	for ((x, y), paint) in hull.iter() {
		output[(y - min_y) as usize][(x - min_x) as usize] = *paint;
	}
	let ans = output
		.into_iter()
		.rev()
		.map(|l| {
			l.into_iter()
				.map(|n| if n == 0 { " " } else { "█" })
				.collect::<String>()
		})
		.join("\n");
	println!("{}", ans);
	"^^".to_string()
}
