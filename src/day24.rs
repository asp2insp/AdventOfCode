const Z: u8 = 'z' as u8;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Operand {
	Var(char),
	Lit(isize),
}

impl std::fmt::Display for Operand {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Var(c) => write!(f, "{}", c),
			Lit(n) => write!(f, "{}", n),
		}
	}
}

impl From<Option<&str>> for Operand {
	fn from(s: Option<&str>) -> Self {
		match s.unwrap().chars().next().unwrap() {
			c if c >= 'w' && c <= 'z' => Var(c),
			_ => Lit(parse!(s.unwrap(), isize)),
		}
	}
}


#[derive(Debug)]
enum Instruction {
	Inp(Operand),
	Add(Operand, Operand),
	Mul(Operand, Operand),
	Div(Operand, Operand),
	Mod(Operand, Operand),
	Eql(Operand, Operand),
}

use std::ops::Add;

use Instruction::*;
use Operand::*;
use itertools::Itertools;

impl From<&str> for Instruction {
	fn from(s: &str) -> Self {
		let mut parts = s.trim().split_whitespace();
		match parts.next().unwrap() {
			"inp" => Inp(parts.next().into()),
			"add" => Add(parts.next().into(), parts.next().into()),
			"mul" => Mul(parts.next().into(), parts.next().into()),
			"div" => Div(parts.next().into(), parts.next().into()),
			"mod" => Mod(parts.next().into(), parts.next().into()),
			"eql" => Eql(parts.next().into(), parts.next().into()),
			_ => unreachable!(),
		}
	}
}

struct Computer {
	memory: [isize; 4],
	prog: Vec<Instruction>,
}

impl Computer {
	fn new(is: Vec<Instruction>) -> Computer {
		Computer {
			memory: [0; 4],
			prog: is,
		}
	}

	fn read(&self, op: Operand) -> isize {
		match op {
			Lit(v) => v,
			Var(a) => self.memory[(Z - a as u8) as usize],
		}
	}

	fn store(&mut self, val: isize, dest: Operand) {
		match dest {
			Var(a) => self.memory[(Z -a as u8) as usize] = val,
			_ => unimplemented!(),
		}
	}

	// fn run(&mut self, tape: impl IntoIterator<Item=isize>) {
	// 	for instr in &self.prog {
	// 		match instr {
	// 			Inp(op) 
	// 		}
	// 	}
	// }

	fn to_c(&self) -> String {
		let mut s = String::new();
		s += "#include <stdio.h>\n\nint main(int* input) {\n";
		s += "int w, x, y, z = 0;";
		let mut shift = 0;
		for i in &self.prog {
			s += &format!("\t{};\n", match i {
				Inp(Var(a)) => {
					shift += 1;
					format!("{} = input[{}] - '0'", a, shift-1)
					// format!("{} = getchar() - 0; putchar({})", a, a)
				},
				Add(Var(a), op) => format!("{} = {} + {}", a, a, op),
				Mul(Var(a), op) => format!("{} = {} * {}", a, a, op),
				Div(Var(a), op) => format!("{} = {} / {}", a, a, op),
				Mod(Var(a), op) => format!("{} = {} % {}", a, a, op),
				Eql(Var(a), op) => format!("{} = {} == {} ? 1 : 0", a, a, op),
				_ => unreachable!(),
			});
		}
		// s += "\tif (z) {puts(\"No\");} else {puts(\"Yes\");}";
		s += "\treturn z;\n}";
		s
	}
}

pub fn part1(input: String) -> String {
	let instrs = input.lines().map(Instruction::from).collect_vec();
	let mut c = Computer::new(instrs);
	println!("{}", c.to_c());
	"".to_string()
}


pub fn part2(input: String) -> String {
	"part2".to_string()
}

struct Block {
	z_div: isize,
	x_add: isize,
	y_add: isize,
}

impl Block {
	fn run_block(&self, in_digit: isize, mut carry: isize) -> isize {
		let test = if carry % 26 + self.x_add == in_digit {0} else {1};
		carry /= self.z_div;

		carry *= 25 * test + 1;

		carry += (in_digit + self.y_add) * test;

		carry
	}
}

