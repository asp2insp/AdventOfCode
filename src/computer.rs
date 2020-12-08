use std::collections::HashSet;
use Inst::*;

pub struct Computer {
    pub program: Vec<Inst>,
}

impl Computer {
    pub fn run(&self) -> Option<isize> {
        let mut accumulator = 0;
        let mut iptr: isize = 0;
        let mut seen = HashSet::new();
        loop {
            if seen.contains(&iptr) {
                return None
            } else if iptr as usize >= self.program.len() {
                return Some(accumulator);
            }
            seen.insert(iptr);
            iptr = match self.program[iptr as usize] {
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
}

#[derive(Clone, Copy)]
pub enum Inst {
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

pub fn parse_program(s: &str) -> Vec<Inst> {
	s.lines()
		.map(Inst::from)
		.collect()
}