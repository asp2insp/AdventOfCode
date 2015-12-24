use std::str;
use std::collections::HashMap;

pub fn part1(input: String) -> String {
	let mut coord = (0, 0);
	let positions = input.chars().map(|c| {
		match c {
			'<' => (-1, 0),
			'>' => (1, 0),
			'^' => (0, 1),
			'v' => (0, -1),
			_ => (0, 0),
		}
	}).map(|m| {
		coord = (coord.0 + m.0, coord.1 + m.1);
		coord
	});
	let num_houses = Some((0,0)).into_iter()
		.chain(positions)
		.fold(HashMap::<String, i32>::new(), |mut m, pos| {
			*m.entry(format!("({},{})", pos.0, pos.1)).or_insert(0) += 1;
			m
		})
		.len();
	format!("{}", num_houses)
}


pub fn part2(input: String) -> String {
	"".to_string()
}
