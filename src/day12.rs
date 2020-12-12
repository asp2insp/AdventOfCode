
use Cmd::*;

enum Cmd {
	N(isize),
	S(isize),
	E(isize),
	W(isize),
	T(isize),
	F(isize),
}

impl Cmd {
	fn dir_to_cardinal(d: isize, i: isize) -> Cmd {
		match d % 360 {
			0 => N(i),
			90 | -270 => E(i),
			180 | - 180 => S(i),
			270 | -90 => W(i),
			_ => unimplemented!("Direction {} wasn't implemented", d),
		}
	}

	fn apply(&self, (x, y, d): (isize, isize, isize)) -> (isize, isize, isize) {
		match &self {
			N(i) => (x, y-i, d),
			S(i) => (x, y+i, d),
			E(i) => (x+i, y, d),
			W(i) => (x-i, y, d),
			T(i) => (x, y, (d+i) % 360),
			F(i) => Cmd::dir_to_cardinal(d, *i).apply((x,y,d)),
		}
	}

	fn rotate_around_origin(&self, (x, y): (isize, isize)) -> (isize, isize) {
		if let T(i) = self {
			match i {
				0 => (x, y),
				90 | -270 => (y, -x),
				180 | - 180 => (-x, -y),
				270 | -90 => (-y, x),
				_ => unimplemented!(),
			}
		} else {
			unimplemented!("Only rotation allowed")
		}
	}

	fn apply2(&self, (x, y): (isize, isize)) -> (isize, isize) {
		match &self {
			N(i) => (x, y+i),
			S(i) => (x, y-i),
			E(i) => (x+i, y),
			W(i) => (x-i, y),
			T(_) => self.rotate_around_origin((x, y)),
			F(i) => unimplemented!("F not allowed in apply2"),
		}
	}
}

fn parse_ln(s: &str) -> Cmd {
	let i = s.trim()[1..].parse::<isize>().unwrap();
	match s.trim().chars().next().unwrap() {
		'N' => N(i),
		'S' => S(i),
		'E' => E(i),
		'W' => W(i),
		'L' => T(-i),
		'R' => T(i),
		'F' => F(i),
		_ => unimplemented!(),
	}
}


pub fn part1(input: String) -> String {
	let (x, y, _) = input.lines()
		.map(parse_ln)
		.fold((0, 0, 90), |pos, c| c.apply(pos));
	(x.abs() + y.abs()).to_string()
}


pub fn part2(input: String) -> String {
	let mut coords = (0, 0);
	let mut waypoint = (10, 1);
	for c in input.lines().map(parse_ln) {
		if let F(i) = c {
			coords = (coords.0 + i * waypoint.0, coords.1 + i * waypoint.1);
		} else {
			waypoint = c.apply2(waypoint);
		}
	}
	(coords.0.abs() + coords.1.abs()).to_string()
}

#[test]
fn test_rotate() {
	assert_eq!((4, -10), T(90).rotate_around_origin((10, 4)));
	assert_eq!((4, 10), T(90).rotate_around_origin(T(-90).rotate_around_origin((4,10))));
	assert_eq!(T(90).rotate_around_origin((6, -12)), T(-270).rotate_around_origin((6, -12)));
}

#[test]
fn test_two() {
	let s = r"F10
	N3
	F7
	R90
	F11";
	assert_eq!("286", part2(s.to_owned()));
}