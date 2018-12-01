static BUTTONS: [[u8; 3]; 3] = [
	[1, 2, 3],
	[4, 5, 6],
	[7, 8, 9],
];

static REAL_BUTTONS: [[char; 5]; 5] = [
	['_', '_', '1', '_', '_'],
	['_', '2', '3', '4', '_'],
	['5', '6', '7', '8', '9'],
	['_', 'A', 'B', 'C', '_'],
	['_', '_', 'D', '_', '_'],
];

fn follow_direction(x: isize, y: isize, d: char) -> (isize, isize) {
	let (x, y) = match d {
		'U' => (x, y-1),
		'D' => (x, y+1),
		'L' => (x-1, y),
		'R' => (x+1, y),
		_ => unimplemented!(),
	};
	(x.min(2).max(0), y.min(2).max(0))
}

fn follow_direction_2(x: isize, y: isize, d: char) -> (isize, isize) {
	let (x_new, y_new) = match d {
		'U' => (x, y-1),
		'D' => (x, y+1),
		'L' => (x-1, y),
		'R' => (x+1, y),
		_ => unimplemented!(),
	};
	let x_new = x_new.min(4).max(0);
	let y_new = y_new.min(4).max(0);

	if REAL_BUTTONS[y_new as usize][x_new as usize] == '_' {
		(x, y)
	} else {
		(x_new, y_new)
	}
}

trait CompInt {
	fn max(self, other: Self) -> Self;
	fn min(self, other: Self) -> Self;
}

impl CompInt for isize {
	fn max(self, other: isize) -> isize {
		if self > other {
			self
		} else {
			other
		}
	}

	fn min(self, other: isize) -> isize {
		if self < other {
			self
		} else {
			other
		}
	}
}

pub fn part1(input: String) -> String {
	let mut x = 1;
	let mut y = 1;
	let mut ret = vec![];
	for line in input.lines() {
		for d in line.chars() {
			let (x_new, y_new) = follow_direction(x, y, d);
			x = x_new;
			y = y_new;
		}
		ret.push(BUTTONS[y as usize][x as usize]);
	}
	format!("{:?}", ret)
}


pub fn part2(input: String) -> String {
	let mut x = 0;
	let mut y = 2;
	let mut ret = vec![];
	for line in input.lines() {
		for d in line.chars() {
			let (x_new, y_new) = follow_direction_2(x, y, d);
			x = x_new;
			y = y_new;
		}
		ret.push(REAL_BUTTONS[y as usize][x as usize]);
	}
	format!("{}", ret.into_iter().collect::<String>())
}
