use permutohedron::LexicalPermutation;
use rayon::prelude::*;

fn parse_program(input: String) -> Vec<isize> {
	input.split(",").flat_map(str::parse::<isize>).collect()
}

macro_rules! make_input [
	($($e:expr),*) => {
		Box::new(vec![$($e),*].into_iter())
	};
];

#[derive(PartialEq, Debug)]
enum ProgYield {
	Output(isize),
	Halt,
}

struct Computer {
	program: Vec<isize>,
	iptr: usize,
	input: Box<dyn Iterator<Item = isize>>,
	relative_base: isize,
}

impl Computer {
	fn new(prog: &[isize], input: Box<dyn Iterator<Item = isize>>) -> Computer {
		Computer {
			program: prog.to_vec(),
			iptr: 0,
			input: input,
			relative_base: 0,
		}
	}

	fn check_dest(&mut self, offset: usize, param_modes: isize) -> usize {
		let mode = param_modes / 10_isize.pow(offset as u32 - 1);
		let dest = (self.program[self.iptr + offset] + if mode == 2 { self.relative_base } else { 0 }) as usize;
		if dest > self.program.len() {
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
					self.program[dest] = self.input.next().unwrap();
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

fn run_to_tape(program: &[isize]) -> String {
	let mut computer = Computer::new(&program, make_input![1]);
	let mut res = String::new();
	while let ProgYield::Output(o) = computer.run_and_return_output() {
		res.push_str(&format!("{},", o));
	}
	res
}

pub fn part1(input: String) -> String {
	let mut program = parse_program(input);
	run_to_tape(&program)
}

pub fn part2(input: String) -> String {
	let mut program = parse_program(input);
	let mut computer = Computer::new(&program, make_input![2]);
	let mut res = String::new();
	while let ProgYield::Output(o) = computer.run_and_return_output() {
		res.push_str(&format!("{},", o));
	}
	res
}

#[test]
fn test_one() {
	let mut prog = [104, 1125899906842624, 99];
	let mut computer = Computer::new(&prog, make_input![1]);
	assert_eq!(
		ProgYield::Output(1125899906842624),
		computer.run_and_return_output()
	);
}

#[test]
fn test_big() {
	let mut computer = Computer::new(&[1102, 34915192, 34915192, 7, 4, 7, 99, 0], make_input![1]);
	assert_eq!(
		ProgYield::Output(1219070632396864),
		computer.run_and_return_output()
	);
}

#[test]
fn test_quine() {
	assert_eq!(
		"109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99,",
		run_to_tape(&[109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99])
	);
}
