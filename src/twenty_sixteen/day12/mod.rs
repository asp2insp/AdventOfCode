use chomp::prelude::{U8Input, SimpleResult, parse_only, many1, any, string, token, take_while};
use chomp::ascii::{decimal, skip_whitespace, is_whitespace, is_alphanumeric, is_alpha, signed};
use std::collections::HashMap;
use chomp::types::Buffer;
use self::Instr::*;

#[derive(Debug)]
enum Instr {
	Cpy(u8, u8),
	Inc(u8),
	Dec(u8),
	Jnz(u8, isize),
}

impl Instr {
	fn apply(&self, regs: &mut HashMap<u8, isize>) -> isize {
		match self {
			&Inc(r) => {
				*regs.entry(r).or_insert(0) += 1;
				1
			},
			&Dec(r) => {
				*regs.entry(r).or_insert(0) -= 1;
				1
			},
			&Cpy(x, y) => {
				if is_alpha(x) {
					let v = *regs.get(&x).unwrap();
					regs.insert(y, v);
				} else {
					regs.insert(y, x as isize);
				}
				1
			},
			&Jnz(r, i) => {
				if is_alpha(r) {
					if *regs.get(&r).unwrap() != 0 { i } else { 1 }
				} else {
					if r != 0 { i } else { 1 }
				}
			},
		}
	}
}

fn copy<I: U8Input>(i: I) -> SimpleResult<I, Instr> {
    parse!{i;
        string(b"cpy ");
		let x = take_while(is_alphanumeric);
				token(b' ');
		let y = any();
				skip_whitespace();
		ret if is_alpha(x.to_vec()[0]) {
			Cpy(x.to_vec()[0], y)
		} else {
			Cpy(
				String::from_utf8(x.to_vec())
					.unwrap()
					.parse::<u8>()
					.unwrap(),
				y
			)
		}
	}
}

fn increment<I: U8Input>(i: I) -> SimpleResult<I, Instr> {
    parse!{i;
        string(b"inc ");
		let x = any();
				skip_whitespace();
		ret Inc(x)
	}
}

fn jump_not_zero<I: U8Input>(i: I) -> SimpleResult<I, Instr> {
    parse!{i;
        string(b"jnz ");
		let x = any();
				token(b' ');
		let y = signed(decimal);
				skip_whitespace();
		ret Jnz(x, y)
	}
}

fn decrement<I: U8Input>(i: I) -> SimpleResult<I, Instr> {
    parse!{i;
        string(b"dec ");
		let x = any();
				skip_whitespace();
		ret Dec(x)
	}
}

fn instr<I: U8Input>(i: I) -> SimpleResult<I, Instr> {
    parse!{i;
        let instruction = increment() <|> decrement() <|> jump_not_zero() <|> copy();
		ret instruction
	}
}

fn all_instrs<I: U8Input>(i: I) -> SimpleResult<I, Vec<Instr>> {
    parse!{i;
        let v = many1(instr);
		ret v
	}
}

fn run(program: Vec<Instr>, regs: &mut HashMap<u8, isize>) -> isize {
	let mut i = 0isize;
	while i < program.len() as isize {
		let prev = i;
		i += program[i as usize].apply(regs);
	}
	*regs.get(&b'a').unwrap()
}

pub fn part1(input: String) -> String {
	let mut regs = HashMap::with_capacity(3);
	regs.insert(b'a', 0);
	regs.insert(b'b', 0);
	regs.insert(b'c', 0);

	let instrs = parse_only(all_instrs, input.as_bytes()).unwrap();
	format!("{}", run(instrs, &mut regs))
}


pub fn part2(input: String) -> String {
	let mut regs = HashMap::with_capacity(3);
	regs.insert(b'a', 0);
	regs.insert(b'b', 0);
	regs.insert(b'c', 1);

	let instrs = parse_only(all_instrs, input.as_bytes()).unwrap();
	format!("{}", run(instrs, &mut regs))
}
