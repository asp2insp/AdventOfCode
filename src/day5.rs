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

fn run_and_return_output(
	mut program: Vec<isize>,
	mut input: Box<dyn Iterator<Item = isize>>,
) -> Vec<isize> {
	let mut iptr = 0;
	let mut output = Vec::new();
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
				output.push(params[0]);
				iptr += 2;
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
			99 => break,
			other => unreachable!("Opcode {} unknown", other),
		}
	}
	output
}

pub fn part1(input: String) -> String {
	let mut program = parse_program(input);
	let prog_input = Box::new(vec![1].into_iter());
	format!("{:?}", run_and_return_output(program, prog_input))
}

pub fn part2(input: String) -> String {
	let mut program = parse_program(input);
	let prog_input = Box::new(vec![5].into_iter());
	format!("{:?}", run_and_return_output(program, prog_input))
}

#[test]
fn test_small() {
	let mut program = vec![
		3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
		1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
		1105, 1, 46, 98, 99,
	];

	assert_eq!(
		vec![999],
		run_and_return_output(program.clone(), Box::new(vec![7].into_iter()))
	);
	assert_eq!(
		vec![1000],
		run_and_return_output(program.clone(), Box::new(vec![8].into_iter()))
	);
	assert_eq!(
		vec![1001],
		run_and_return_output(program.clone(), Box::new(vec![9].into_iter()))
	);
}
