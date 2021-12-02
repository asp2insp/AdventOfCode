use std::collections::HashSet;

use crate::computer::Inst::{self, *};
use crate::computer::{parse_program, Computer};

pub fn part1(input: String) -> String {
	let mut accumulator = 0;
	let mut iptr: isize = 0;
	let program = parse_program(&input);
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
			}
			Jmp(n) => iptr + n,
			Nop(n) => iptr + 1,
		}
	}
	accumulator.to_string()
}

pub fn part2(input: String) -> String {
	let program = parse_program(&input);
	for i in 0..program.len() {
		let mut p = program.clone();
		match program[i] {
			Acc(n) => continue,
			Nop(n) => p[i] = Jmp(n),
			Jmp(n) => p[i] = Nop(n),
		};
		let comp = Computer { program: p };
		if let Some(res) = comp.run() {
			return res.to_string();
		}
	}
	"Not found".to_owned()
}
