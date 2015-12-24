use std::collections::HashMap;

pub fn part1(input: String) -> String {
	let mut coord = (0, 0);
	let positions = input.chars()
	.enumerate()
	.map(char_to_move)
	.map(|m| {
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

fn char_to_move(i_and_c: (usize, char)) -> (i32, i32) {
	match i_and_c.1 {
		'<' => (-1, 0),
		'>' => (1, 0),
		'^' => (0, 1),
		'v' => (0, -1),
		_ => (0, 0),
	}
}

pub fn part2(input: String) -> String {
	let mut coord1 = (0, 0);
	let mut coord2 = (0, 0);
	let positions1 = input.chars()
	.enumerate()
	.filter(|pair| pair.0 % 2 == 0)
	.map(char_to_move).map(|m| {
		coord1 = (coord1.0 + m.0, coord1.1 + m.1);
		coord1
	});
	let positions2 = input.chars()
	.enumerate()
	.filter(|pair| pair.0 % 2 == 1)
	.map(char_to_move).map(|m| {
		coord2 = (coord2.0 + m.0, coord2.1 + m.1);
		coord2
	});
	let num_houses = Some((0,0)).into_iter()
		.chain(positions1)
		.chain(positions2)
		.fold(HashMap::<String, i32>::new(), |mut m, pos| {
			*m.entry(format!("({},{})", pos.0, pos.1)).or_insert(0) += 1;
			m
		})
		.len();
	format!("{}", num_houses)
}
