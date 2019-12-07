use permutohedron::LexicalPermutation;
use rayon::prelude::*;

fn parse_program(input: String) -> Vec<isize> {
	input.split(",").flat_map(str::parse::<isize>).collect()
}

fn get_params(n: usize, mut param_modes: isize, loc: usize, program: &[isize]) -> Vec<isize> {
	let mut ret = Vec::new();
	for i in 1..=n {
		let mode = param_modes % 10;
		let prog_value = program[loc + i];
		match mode {
			0 => ret.push(program[prog_value as usize]),
			1 => ret.push(prog_value),
			_ => unreachable!(),
		}
		param_modes = param_modes / 10;
	}
	ret
}

enum ProgYield {
	Output(isize, usize),
	Halt,
}

fn run_and_return_output(
	mut program: &mut [isize],
	mut iptr: usize,
	mut input: Box<dyn Iterator<Item = isize>>,
) -> ProgYield {
	loop {
		let opcode = program[iptr] % 100;
		let param_modes = program[iptr] / 100;
		match opcode {
			1 => {
				// Add 1, 2, store 3
				let params = get_params(2, param_modes, iptr, &program);
				let dest = program[iptr + 3] as usize;
				program[dest] = params[0] + params[1];
				iptr += 4;
			}
			2 => {
				// Mult 1, 2, store 3
				let params = get_params(2, param_modes, iptr, &program);
				let dest = program[iptr + 3] as usize;
				program[dest] = params[0] * params[1];
				iptr += 4;
			}
			3 => {
				// Input and store 1
				// let params = get_params(1, param_modes, iptr, &program);
				let dest = program[iptr + 1] as usize;
				program[dest] = input.next().unwrap();
				iptr += 2;
			}
			4 => {
				// Output 1
				let params = get_params(1, param_modes, iptr, &program);
				return ProgYield::Output(params[0], iptr + 2);
			}
			5 => {
				// JNZ 1 to 2
				let params = get_params(2, param_modes, iptr, &program);
				if params[0] != 0 {
					iptr = params[1] as usize;
				} else {
					iptr += 3;
				}
			}
			6 => {
				// JEZ 1 to 2
				let params = get_params(2, param_modes, iptr, &program);
				if params[0] == 0 {
					iptr = params[1] as usize;
				} else {
					iptr += 3;
				}
			}
			7 => {
				// 1 LT 2, store 3
				let params = get_params(2, param_modes, iptr, &program);
				let dest = program[iptr + 3] as usize;
				program[dest] = if params[0] < params[1] { 1 } else { 0 };
				iptr += 4;
			}
			8 => {
				// 1 EQ 2, store 3
				let params = get_params(2, param_modes, iptr, &program);
				let dest = program[iptr + 3] as usize;
				program[dest] = if params[0] == params[1] { 1 } else { 0 };
				iptr += 4;
			}
			99 => return ProgYield::Halt,
			other => unreachable!("Opcode {} unknown", other),
		}
	}
}

fn run_amp(mut program: &mut [isize], phase_setting: isize, in_val: isize) -> isize {
	if let ProgYield::Output(val, iptr) = run_and_return_output(
		program,
		0,
		Box::new(vec![phase_setting, in_val].into_iter()),
	) {
		val
	} else {
		panic!("Program halted without output")
	}
}

fn run_amps(program: &Vec<isize>, phase_settings: &[isize]) -> isize {
	let mut carry_val = 0;
	for setting in phase_settings {
		carry_val = run_amp(&mut program.clone(), *setting, carry_val);
	}
	carry_val
}

fn make_combos(mut settings: [isize; 5]) -> Vec<[isize; 5]> {
	let mut combos = vec![];
	loop {
		combos.push(settings.clone());
		if !settings.next_permutation() {
			break;
		}
	}
	combos
}

pub fn part1(input: String) -> String {
	let mut program = parse_program(input);
	let max_thrust = make_combos([0, 1, 2, 3, 4])
		.into_par_iter()
		.map(|s| run_amps(&program, &s))
		.max()
		.unwrap();
	format!("{:?}", max_thrust)
}

fn loop_amps(program: &Vec<isize>, phase_settings: &[isize]) -> isize {
	let mut carry_val = 0;
	let mut progs: Vec<Vec<isize>> = [program; 5].into_iter().map(|v| (*v).clone()).collect();
	let mut iptrs: [usize; 5] = [0; 5];
	let mut n = 0;
	let mut out = 0;
	let mut phase_setting_once: Vec<Option<isize>> =
		phase_settings.iter().map(|s| Some(*s)).collect();
	loop {
		let res = run_and_return_output(
			&mut progs[n],
			iptrs[n],
			Box::new(
				phase_setting_once[n]
					.take()
					.into_iter()
					.chain(std::iter::once(carry_val)),
			),
		);
		match res {
			ProgYield::Output(cv, iptr) => {
				iptrs[n] = iptr;
				carry_val = cv;
				if n == iptrs.len() - 1 {
					out = carry_val;
				}
			}
			ProgYield::Halt => {
				if n == iptrs.len() - 1 {
					break;
				}
			}
		};
		n = (n + 1) % iptrs.len();
	}
	out
}

pub fn part2(input: String) -> String {
	let mut program = parse_program(input);
	let max_thrust = make_combos([5, 6, 7, 8, 9])
		.into_par_iter()
		.map(|s| loop_amps(&program, &s))
		.max()
		.unwrap();
	format!("{:?}", max_thrust)
}

#[test]
fn test_one() {
	let mut prog = [
		3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
		1005, 28, 6, 99, 0, 0, 5,
	];
	let settings = [9, 8, 7, 6, 5];
	let val = loop_amps(&prog.to_vec(), &settings);
	assert_eq!(139629729, val);
}
