use itertools::*;
use std::collections::{BinaryHeap, HashSet};
use rayon::prelude::*;

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

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Inp(Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
    Mod(Operand, Operand),
    Eql(Operand, Operand),
}

use std::ops::Add;

use itertools::Itertools;
use Instruction::*;
use Operand::*;

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
}

impl Computer {
    fn new() -> Computer {
        Computer {
            memory: [0; 4],
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
            Var(a) => self.memory[(Z - a as u8) as usize] = val,
            _ => unimplemented!(),
        }
    }

    fn run_one<'a>(&mut self, mut prog: impl Iterator<Item=&'a Instruction>, tape: &mut dyn Iterator<Item = isize>) {
		if let Some(instr) = prog.next() {
			match *instr {
				Inp(op) => self.store(tape.next().unwrap(), op),
				Add(l, r) => self.store(self.read(l) + self.read(r), l),
				Mul(l, r) => self.store(self.read(l) * self.read(r), l),
				Div(l, r) => self.store(self.read(l) / self.read(r), l),
				Mod(l, r) => self.store(self.read(l) % self.read(r), l),
				Eql(l, r) => self.store(if self.read(l) == self.read(r) { 1 } else { 0 }, l),
			};
		}
    }

	fn run_block<'a>(&mut self, mut prog: impl Iterator<Item=&'a Instruction>, next_digit: isize) -> isize {
		for _ in 0..18 {
			self.run_one(&mut prog, &mut std::iter::once(next_digit));
		}
		self.read(Var('z'))
	}

	fn run<'a>(&mut self, mut prog: impl Iterator<Item=&'a Instruction>, mut tape: &[isize]) -> isize {
		let mut ret = 0;
		for i in 0..14 {
			ret = self.run_block(&mut prog, tape[i]);
		}
		ret
	}
}

fn find_target(prog: &[Instruction], target: isize) -> Vec<(isize, isize)> {
	let mut ret = vec![];
	for carry in 0..1830 {
		for next_digit in 1..10 {
			let mut c = Computer::new();
			c.store(carry, Operand::Var('z'));
			let ncarry = c.run_block(prog.iter(), next_digit);
			if ncarry == target {
				ret.push((carry, next_digit));
			}
		}
	}
	ret
}

fn find_block_target(b: &Block, target: isize) -> Vec<(isize, isize)> {
	let mut ret = vec![];
	for carry in 0..1830 {
		for next_digit in 1..10 {
			let ncarry = b.run_block(next_digit, carry);
			if ncarry == target {
				ret.push((carry, next_digit));
			}
		}
	}
	ret
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct State {
	digits: isize,
	block: usize,
	carry_target: isize,
}

impl std::cmp::Ord for State {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.block.cmp(&other.block).reverse().then(self.digits.cmp(&other.digits))
	}
}

impl std::cmp::PartialOrd for State {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

fn check_last_digit(blocks: &[Block], last_carry: isize, last_digit: isize) {
	let mut q: BinaryHeap<State> = BinaryHeap::new();
	let mut minblock = 14;
	let mut maxdigit = 0;
	q.push(State {
		digits: last_digit,
		block: 12,
		carry_target: last_carry,
	});

	while let Some(n) = q.pop() {
		maxdigit = maxdigit.max(n.digits);
		if n.block < minblock {
			println!("Got to {}, qdepth {}, curr digit = {}", n.block, q.len(), maxdigit);
			minblock = n.block;
		}
		// if n.block < 2 {
		// 	println!("{}: {}  ({})", n.block, n.digits, q.len());
		// }
		for (carry_target, next_digit) in find_block_target(&blocks[n.block], n.carry_target) {
			let s = State {
				digits: n.digits + next_digit * 10isize.pow((13-n.block) as u32),
				block: n.block - 1,
				carry_target,
			};
			if s.block == 0 {
				for (first_z, next_digit) in find_block_target(&blocks[0], carry_target) {
					if first_z == 0 {
						println!("Possible Answer: {}{}", next_digit, s.digits);
					}
				}
				return
			}
			q.push(s);
		}
	}
}

pub fn part1(input: String) -> String {
    let instrs = input.lines().map(Instruction::from).collect_vec();
	let blocks = parse_blocks(&input);
	find_target(&instrs[13*18..], 0)
		.into_par_iter()
		.for_each(|(c, d)| check_last_digit(&blocks, c, d));
    "".to_string()
}

pub fn part2(input: String) -> String {
	let tape = vec![5,3,9,9,9,9,9,5,8,2,9,3,9,9];
	let instrs = input.lines().map(Instruction::from).collect_vec();
	let blocks = parse_blocks(&input);
	assert_eq!(run_blocks(&blocks, &tape), Computer::new().run(instrs.iter(), &tape));
	println!("53999995829399 => {:?}", run_blocks_stack(&blocks, &tape));
	let tape = vec![1,1,7,2,1,1,5,1,1,1,8,1,7,5];
	println!("11721151118175 => {:?}", run_blocks_stack(&blocks, &tape));
	// println!("");
	// let mut stack: Vec<(usize, Block)> = vec![];
	// for (i, b) in blocks.into_iter().enumerate() {
	// 	if b.z_div == 26 {
	// 		let m = stack.pop().unwrap();
	// 		println!("{:<2} {:<2} <=> {:<3} {:<2}", m.0, m.1.y_add,  b.x_add, i);
	// 	} else {
	// 		stack.push((i,b));
	// 	}
	// }
    "part2".to_string()
}


fn to_c(prog: impl IntoIterator<Item=Instruction>) -> String {
	let mut s = String::new();
	s += "#include <stdio.h>\n\nint main(int* input) {\n";
	s += "int w, x, y, z = 0;";
	let mut shift = 0;
	for i in prog {
		s += &format!(
			"\t{};\n",
			match i {
				Inp(Var(a)) => {
					shift += 1;
					format!("{} = input[{}] - '0'", a, shift - 1)
					// format!("{} = getchar() - 0; putchar({})", a, a)
				}
				Add(Var(a), op) => format!("{} = {} + {}", a, a, op),
				Mul(Var(a), op) => format!("{} = {} * {}", a, a, op),
				Div(Var(a), op) => format!("{} = {} / {}", a, a, op),
				Mod(Var(a), op) => format!("{} = {} % {}", a, a, op),
				Eql(Var(a), op) => format!("{} = {} == {} ? 1 : 0", a, a, op),
				_ => unreachable!(),
			}
		);
	}
	// s += "\tif (z) {puts(\"No\");} else {puts(\"Yes\");}";
	s += "\treturn z;\n}";
	s
}

fn run_blocks(b: &[Block], t: &[isize]) -> isize {
	b.iter().zip(t.iter()).fold(0, |carry, bt| bt.0.run_block(*bt.1, carry))
}

fn run_blocks_stack(b: &[Block], t: &[isize]) -> Vec<isize> {
	b.iter().zip(t.iter()).fold(vec![], |carry, bt| {
		let ret = bt.0.run_stack(*bt.1, carry.clone());
		println!("{:?} [{:?}] => {:?}", carry, bt, ret);
		ret
	})
}

fn parse_blocks(s: &str) -> Vec<Block> {
    s.split("inp w")
		.filter(|s| !s.is_empty())
		.map(|b| {
        let rel = b
            .lines()
			.enumerate()
			.filter(|(i, _)| [4, 5, 15].contains(i))
			.map(|(_, s)| s)
            .map(|l| parse!(
				l.split_whitespace().skip(2).next().unwrap(), 
				isize)
			)
            .collect_vec();
		Block {
			z_div: rel[0],
			x_add: rel[1],
			y_add: rel[2],
		}
    })
	.collect_vec()
}

#[derive(Debug)]
struct Block {
    z_div: isize,
    x_add: isize,
    y_add: isize,
}

impl Block {
    fn run_block(&self, in_digit: isize, mut carry: isize) -> isize {
        let x = carry % 26 + self.x_add; // stack.peek()
		if self.z_div == 26 {
			carry = carry / 26; // stack.pop()
		}
		if x != in_digit {
			carry = carry * 26 + in_digit + self.y_add; // stack.push()
		}
		carry
    }

	fn run_stack(&self, in_digit: isize, mut carry: Vec<isize>) -> Vec<isize> {
		let x = carry.last().unwrap_or(&0) + self.x_add; // stack.peek()
		if self.z_div == 26 {
			carry.pop(); // stack.pop()
		}
		if x != in_digit {
			carry.push(in_digit + self.y_add); // stack.push()
		}
		carry
	}
}
