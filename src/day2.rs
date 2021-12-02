
pub fn part1(input: String) -> String {
	let mut depth: usize = 0;
	let mut pos: usize = 0;
	for l in input.lines() {
		let d = l.split_whitespace().collect::<Vec<_>>();
		match d[0] {
			"forward" => pos += parse!(d[1], usize),
			"down" => depth += parse!(d[1], usize),
			"up" => depth -= parse!(d[1], usize),
			_ => unimplemented!(),
		};
	}
	format!("{}", depth * pos)
}


pub fn part2(input: String) -> String {
	let mut depth: isize = 0;
	let mut pos: isize = 0;
	let mut aim: isize = 0;
	for l in input.lines() {
		let d = l.split_whitespace().collect::<Vec<_>>();
		match d[0] {
			"forward" => {
				let num = parse!(d[1], isize);
				pos += num;
				depth += aim * num; 
			},
			"down" => aim += parse!(d[1], isize),
			"up" => aim -= parse!(d[1], isize),
			_ => unimplemented!(),
		};
	}
	format!("{}", depth * pos)
}
