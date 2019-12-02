
fn parse_program(input: String) -> Vec<usize> {
	input.split(",")
		.flat_map(str::parse::<usize>)
		.collect()
}

fn run_and_return_output(mut program: Vec<usize>) -> usize {
	let mut iptr = 0;
	loop {
		let (op1, op2, dest) = (program[iptr+1], program[iptr+2], program[iptr+3]);
		match program[iptr] {
			1 => {
				program[dest] = program[op1] + program[op2];
			},
			2 => {
				program[dest] = program[op1] * program[op2];
			},
			99 => break,
			_ => unreachable!(),
		}
		iptr += 4;
	}
	program[0]
}

pub fn part1(input: String) -> String {
	let mut program = parse_program(input);
	// Set initital conditions from puzzle
	program[1] = 12;
	program[2] = 2;
	format!("{}", run_and_return_output(program))
}


pub fn part2(input: String) -> String {
	let program = parse_program(input);
	for noun in 0..=99 {
		for verb in 0..=99 {
			let mut p = program.clone();
			p[1] = noun;
			p[2] = verb;
			if run_and_return_output(p) == 19690720 {
				return format!("100 * {} + {} = {}", noun, verb, 100 * noun + verb)
			}
		}
	}
	unreachable!()
}
