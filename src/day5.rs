use std::collections::HashSet;

fn find_row(id: &str) -> usize {
	let mut low = 0;
	let mut high = 127;
	for c in id.chars().take(7) {
		match c {
			'B' => low = (low + high) / 2,
			'F' => high = (low + high) / 2,
			_ => unimplemented!(),
		};
	}
	return high;
}

fn find_col(id: &str) -> usize {
	let mut low = 0;
	let mut high = 7;
	for c in id.chars().skip(7).take(3) {
		match c {
			'R' => low = (low + high) / 2,
			'L' => high = (low + high) / 2,
			_ => unimplemented!(),
		};
	}
	return high;
}

fn id(row: usize, col: usize) -> usize {
	// println!("{} {} {}", row, col, row*8+col);
	row * 8 + col
}

pub fn part1(input: String) -> String {
	input.lines()
		.map(|l| id(find_row(l), find_col(l)))
		.max()
		.unwrap()
		.to_string()
}


pub fn part2(input: String) -> String {
	let seats: HashSet<_> = input.lines().map(|l| id(find_row(l), find_col(l))).collect();
	for seat in &seats {
		if seats.contains(&(seat+2)) && !seats.contains(&(seat+1)) {
			return (seat+1).to_string()
		}
	}
	"not_found".to_owned()
}
