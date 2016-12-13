use chomp::prelude::{U8Input, SimpleResult, parse_only, token, string, many1};
use chomp::ascii::{decimal, skip_whitespace};

use self::Instruction::*;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

struct Screen {
	pixels: [[bool; WIDTH]; HEIGHT],
}

impl Screen {
	fn new() -> Screen {
		Screen {
			pixels: [[false; WIDTH]; HEIGHT],
		}
	}

	fn print(&self) {
		for y in 0..HEIGHT {
			let s: String = self.pixels[y].iter().map(|p| if *p {'\u{2588}'} else {' '}).collect();
			println!("{}", s);
		}
		println!(" ");
	}

	fn rotate_row(mut self, y: usize, n: usize) -> Screen {
		for _ in 0..n {
			let last = self.pixels[y][WIDTH-1];
			for x in (1..WIDTH).rev() {
				self.pixels[y][x] = self.pixels[y][x-1];
			}
			self.pixels[y][0] = last;
		}
		self
	}

	fn rotate_col(mut self, x: usize, n: usize) -> Screen {
		for _ in 0..n {
			let last = self.pixels[HEIGHT-1][x];
			for y in (1..HEIGHT).rev() {
				self.pixels[y][x] = self.pixels[y-1][x];
			}
			self.pixels[0][x] = last;
		}
		self
	}

	fn light_up(mut self, x_n: usize, y_n: usize) -> Screen {
		for y in 0..y_n {
			for x in 0..x_n {
				self.pixels[y][x] = true;
			}
		}
		self
	}

	fn count_lit(&self) -> usize {
		self.pixels.iter()
			.flat_map(|r| r.iter())
			.filter(|p| **p)
			.count()
	}

	fn run(self, i: Instruction) -> Screen {
		match i {
			Rect(x_n, y_n) => self.light_up(x_n, y_n),
			RotateRow(y, n) => self.rotate_row(y, n),
			RotateCol(x, n) => self.rotate_col(x, n),
		}
	}
}

enum Instruction {
	Rect(usize, usize),
	RotateRow(usize, usize),
	RotateCol(usize, usize),
}

fn rect<I: U8Input>(i: I) -> SimpleResult<I, Instruction> {
	parse!{i;
				string(b"rect ");
		let x = decimal();
				token(b'x');
		let y = decimal();
		ret Rect(x, y)
	}
}

fn rotate_row<I: U8Input>(i: I) -> SimpleResult<I, Instruction> {
	parse!{i;
				string(b"rotate row y=");
		let y = decimal();
				string(b" by ");
		let n = decimal();
		ret RotateRow(y, n)
	}
}

fn rotate_col<I: U8Input>(i: I) -> SimpleResult<I, Instruction> {
	parse!{i;
				string(b"rotate column x=");
		let x = decimal();
				string(b" by ");
		let n = decimal();
		ret RotateCol(x, n)
	}
}

fn instr<I: U8Input>(i: I) -> SimpleResult<I, Instruction> {
    parse!{i;
		let instruction = rect() <|> rotate_row() <|> rotate_col();
			skip_whitespace();
		ret instruction
	}
}

fn all_instrs<I: U8Input>(i: I) -> SimpleResult<I, Vec<Instruction>> {
    parse!{i;
		let instructions = many1(instr);
		ret instructions
	}
}

pub fn part1(input: String) -> String {
	let instructions = parse_only(all_instrs, input.as_bytes()).unwrap();
	let count = instructions.into_iter()
		.fold(Screen::new(), |s, i| {
			//s.print();
			s.run(i)
		})
		.count_lit();
	format!("{}", count)
}


pub fn part2(input: String) -> String {
	let instructions = parse_only(all_instrs, input.as_bytes()).unwrap();
	let count = instructions.into_iter()
		.fold(Screen::new(), |s, i| {
			//s.print();
			s.run(i)
		})
		.print();
	"^^^".to_owned()
}
