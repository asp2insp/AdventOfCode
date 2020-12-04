fn parse(s: &str) -> Vec<Vec<char>> {
	s.lines()
		.map(|l| l.chars().collect())
		.collect()
		
}

fn trees(slope: (usize, usize), map: &Vec<Vec<char>>) -> usize {
	let mut pos = (0, 0);
	let mut count = 0;
	while pos.0 < map.len() {
		if map[pos.0][pos.1 % map[0].len()] == '#' {
			count += 1;
		}
		pos = (pos.0 + slope.1, pos.1 + slope.0);
	}
	count
}

pub fn part1(input: String) -> String {
	let map = parse(&input);
	trees((3, 1), &map).to_string()
}


pub fn part2(input: String) -> String {
	let map = parse(&input);
	let slopes = [
		(1, 1),
		(3, 1),
		(5, 1),
		(7, 1),
		(1, 2),
	];
	slopes.iter()
		.map(|s| trees(*s, &map))
		.fold(1, |n, acc| n * acc)
		.to_string()
}
