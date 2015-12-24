use std::str;
use chomp::*;
use chomp::ascii::{skip_whitespace,is_digit};

struct Instruction {
	instr: String,
	l: usize,
	t: usize,
	r: usize,
	b: usize,
}

fn make_instruction(i: &[u8], l: &[u8], t: &[u8], r: &[u8], b: &[u8]) -> Instruction {
	Instruction {
		instr: str::from_utf8(i).unwrap().trim().to_string(),
		l: parse_bytes(l),
		t: parse_bytes(t),
		r: parse_bytes(r),
		b: parse_bytes(b),
	}
}

fn parse_bytes(b: &[u8]) -> usize {
	str::from_utf8(b).unwrap().parse::<usize>().unwrap()
}

fn instruction(i: Input<u8>) -> U8Result<Instruction> {
    parse!{i;
        let instruction = take_till(is_digit);
        let l = take_while(is_digit);
                token(b',');
		let t = take_while(is_digit);
                string(b" through ");
		let r = take_while(is_digit);
                token(b',');
        let b = take_while(is_digit);
                skip_whitespace();
        ret make_instruction(instruction, l, t, r, b)
    }
}

fn all_instructions(i: Input<u8>) -> U8Result<Vec<Instruction>> {
    parse!{i;
        let r = many1(instruction);
		ret r
    }
}

fn affected_cells(i: &Instruction) -> Vec<(usize, usize)> {
	let mut r: Vec<(usize, usize)> = Vec::new();
	for x in i.l..(i.r+1) {
		for y in i.t..(i.b+1) {
			r.push((x, y));
		}
	}
	r
}

struct Grid {
	grid: [[bool; 1000]; 1000]
}

impl Grid {
	fn new() -> Grid {
		Grid {
			grid: [[false; 1000]; 1000],
		}
	}

	fn apply(&mut self, i: &Instruction) {
		for cell in affected_cells(i) {
			match i.instr.as_ref() {
				"turn off" => self.grid[cell.0][cell.1] = false,
				"turn on" => self.grid[cell.0][cell.1] = true,
				"toggle" => self.grid[cell.0][cell.1] = !self.grid[cell.0][cell.1],
				_ => {},
			}
		}
	}

	fn count(&self) -> String {
		let mut count = 0;
		for r in self.grid.iter() {
			for c in r.iter() {
				if *c {
					count += 1;
				}
			}
		}
		format!("{}", count)
	}
}

pub fn part1(input: String) -> String {
	parse_only(all_instructions, input.as_bytes())
		.unwrap()
		.iter()
		.fold(Grid::new(), |mut g, i| {
			g.apply(i);
			g
		})
		.count()
}

struct Grid2 {
	grid: Vec<Vec<u32>>
}

fn decr_with_floor(i: u32) -> u32 {
	match i {
		0 => 0,
		_ => i-1,
	}
}

impl Grid2 {
	fn new() -> Grid2 {
		Grid2 {
			grid: vec![vec![0u32; 1000]; 1000]
		}
	}

	fn apply(&mut self, i: &Instruction) {
		for cell in affected_cells(i) {
			let val = self.grid[cell.0][cell.1];
			match i.instr.as_ref() {
				"turn off" => self.grid[cell.0][cell.1] = decr_with_floor(val),
				"turn on" => self.grid[cell.0][cell.1] += 1,
				"toggle" => self.grid[cell.0][cell.1] += 2,
				_ => {},
			}
		}
	}

	fn count(&self) -> String {
		let mut sum = 0u32;
		for r in &self.grid {
			for c in r {
				sum += *c;
			}
		}
		format!("{}", sum)
	}
}

pub fn part2(input: String) -> String {
	parse_only(all_instructions, input.as_bytes())
		.unwrap()
		.iter()
		.fold(Grid2::new(), |mut g, i| {
			g.apply(i);
			g
		})
		.count()
}
