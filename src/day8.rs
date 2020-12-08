use std::collections::HashSet;

use Inst::*;

#[derive(Clone, Copy)]
enum Inst {
	Acc(isize),
	Jmp(isize),
	Nop(isize),
}

impl From<&str> for Inst {
	fn from(s: &str) -> Inst {
		let parts = s.split_whitespace().collect::<Vec<_>>();
		let num = parts[1].parse().unwrap();
		match parts[0] {
			"acc" => Acc(num),
			"jmp" => Jmp(num),
			"nop" => Nop(num),
			_ => unimplemented!(),
		}
	}
}

fn parse(s: &str) -> Vec<Inst> {
	s.lines()
		.map(Inst::from)
		.collect()
}

pub fn part1(input: String) -> String {
	let mut accumulator = 0;
	let mut iptr: isize = 0;
	let program = parse(&input);
	let mut seen = HashSet::new();
	loop {
		if seen.contains(&iptr) {
			break;
		}
		seen.insert(iptr);
		iptr = match program[iptr as usize] {
			Acc(n) => {
				accumulator += n;
				iptr + 1
			},
			Jmp(n) => {
				iptr + n
			},
			Nop(n) => {
				iptr + 1
			},
		}
	}
	accumulator.to_string()
}

fn run_prog(program: Vec<Inst>) -> Option<isize> {
	let mut accumulator = 0;
	let mut iptr: isize = 0;
	let mut seen = HashSet::new();
	loop {
		if seen.contains(&iptr) {
			return None
		} else if iptr as usize >= program.len() {
			return Some(accumulator);
		}
		seen.insert(iptr);
		iptr = match program[iptr as usize] {
			Acc(n) => {
				accumulator += n;
				iptr + 1
			},
			Jmp(n) => {
				iptr + n
			},
			Nop(n) => {
				iptr + 1
			},
		}
	}
}

pub fn part2(input: String) -> String {
	let program = parse(&input);
	for i in 0..program.len() {
		let mut p = program.clone();
		match program[i] {
			Acc(n) => continue,
			Nop(n) => p[i] = Jmp(n),
			Jmp(n) => p[i] = Nop(n),
		};
		if let Some(res) = run_prog(p) {
			return res.to_string()
		}
	}
	"Not found".to_owned()
}
