pub fn part1(input: String) -> String {
	let mut depth: usize = 0;
	let mut pos: usize = 0;
	for l in input.lines() {
		let d = l.split_whitespace().collect::<Vec<_>>();
		match d[0] {
			"forward" => pos += d[1].parse::<usize>().unwrap(),
			"down" => depth += d[1].parse::<usize>().unwrap(),
			"up" => depth -= d[1].parse::<usize>().unwrap(),
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
				let num = d[1].parse::<isize>().unwrap();
				pos += num;
				depth += aim * num; 
			},
			"down" => aim += d[1].parse::<isize>().unwrap(),
			"up" => aim -= d[1].parse::<isize>().unwrap(),
			_ => unimplemented!(),
		};
	}
	format!("{}", depth * pos)
}
