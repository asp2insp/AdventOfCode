use itertools::Itertools;

// A B C
// X Y Z
// R P S

pub fn part1(input: String) -> String {
	input.lines()
		.map(|l| match l {
			"A X" | "B Y" | "C Z" => 3,
			"A Z" | "B X" | "C Y" => 0,
			"A Y" | "B Z" | "C X" => 6,
			_ => panic!("Unknown line {}", l),
		} + match l.chars().skip(2).next().unwrap() {
			'X' => 1,
			'Y' => 2,
			'Z' => 3,
			a => panic!("Unkown play {}", a),
		})
		.sum::<isize>()
		.to_string()
}


// R P S
// A B C
// X Y Z
// L D W

pub fn part2(input: String) -> String {
	input.lines()
		.map(|l| match l {
			"A Y" | "B X" | "C Z" => 1,
			"A Z" | "B Y" | "C X" => 2,
			"A X" | "B Z" | "C Y" => 3,
			_ => panic!("Unknown line {}", l),
		} + match l.chars().skip(2).next().unwrap() {
			'X' => 0,
			'Y' => 3,
			'Z' => 6,
			a => panic!("Unkown play {}", a),
		})
		.sum::<isize>()
		.to_string()
}
