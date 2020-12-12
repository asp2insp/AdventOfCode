use itertools::*;

#[derive(Clone, Eq, PartialEq)]
struct Map {
	pub tiles: Vec<Vec<char>>,
}

impl std::fmt::Display for Map {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		for l in 0..self.tiles.len() {
			for c in 0..self.tiles[l].len() {
				write!(f, "{}", self.tiles[l][c]);
			}
			write!(f, "\n");
		}
		Ok(())
	}
}

impl std::fmt::Debug for Map {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self)
	}
}

fn opt_decrement(n: usize) -> Option<usize> {
	if n > 0 {
		Some(n - 1)
	} else {
		None
	}
}

impl Map {
	fn width(&self) -> usize {
		self.tiles[0].len()
	}

	fn height(&self) -> usize {
		self.tiles.len()
	}

	fn adjacencies(&self, x: usize, y: usize) -> Vec<(usize, usize, char)> {
		self.visible_predicate(x, y, |_| true)
	}

	// Returns a vec of the first item in each direction that satisfies the predicate
	fn visible_predicate(
		&self,
		x: usize,
		y: usize,
		pred: impl Fn(char) -> bool,
	) -> Vec<(usize, usize, char)> {
		const DIRS: &[(isize, isize)] = &[
			(-1, -1),
			(0, -1),
			(1, -1),
			(-1, 0),
			(1, 0),
			(-1, 1),
			(0, 1),
			(1, 1),
		];
		let mut res = vec![];
		for &(dx, dy) in DIRS {
			let mut xi = x as isize;
			let mut yi = y as isize;
			loop {
				if dx < 0 && xi == 0
					|| dy < 0 && yi == 0 || dx > 0 && xi as usize == self.width() - 1
					|| dy > 0 && yi as usize == self.height() - 1
				{
					break;
				}
				xi += dx;
				yi += dy;
				let c = self.tiles[yi as usize][xi as usize];
				if pred(c) {
					res.push((xi as usize, yi as usize, c));
					break;
				}
			}
		}
		res
	}
}

impl From<String> for Map {
	fn from(s: String) -> Self {
		Map {
			tiles: s
				.trim()
				.lines()
				.map(|l| l.trim().chars().collect())
				.collect::<Vec<_>>(),
		}
	}
}

fn run_step(map: &Map) -> Map {
	let mut next = map.clone();
	for x in 0..map.width() {
		for y in 0..map.height() {
			let count = map
				.adjacencies(x, y)
				.into_iter()
				.filter(|(_, _, c)| c == &'#')
				.count();

			match (map.tiles[y][x], count) {
				('L', 0) => next.tiles[y][x] = '#',
				('#', c) if c >= 4 => next.tiles[y][x] = 'L',
				_ => {}
			};
		}
	}
	next
}

pub fn part1(input: String) -> String {
	let map: Map = input.into();
	let mut curr = map;
	loop {
		let next = run_step(&curr);
		if next == curr {
			break;
		} else {
			curr = next;
		}
	}
	curr.tiles
		.iter()
		.flat_map(|r| r.iter())
		.filter(|c| **c == '#')
		.count()
		.to_string()
}

fn run_step2(map: &Map) -> Map {
	let mut next = map.clone();
	for x in 0..map.width() {
		for y in 0..map.height() {
			let count = map
				.visible_predicate(x, y, |c| c == '#' || c == 'L')
				.into_iter()
				.filter(|(_, _, c)| c == &'#')
				.count();

			match (map.tiles[y][x], count) {
				('L', 0) => next.tiles[y][x] = '#',
				('#', c) if c >= 5 => next.tiles[y][x] = 'L',
				_ => {}
			};
		}
	}
	next
}

pub fn part2(input: String) -> String {
	let map: Map = input.into();
	let mut curr = map;
	loop {
		let next = run_step2(&curr);
		if next == curr {
			break;
		} else {
			curr = next;
		}
	}
	curr.tiles
		.iter()
		.flat_map(|r| r.iter())
		.filter(|c| **c == '#')
		.count()
		.to_string()
}

const TEST_MAP: &str = r"
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

const TEST_STEP: &str = r"
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
";

const TEST_STEP_2: &str = r"
#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##
";

#[test]
fn test_adj() {
	let start: Map = TEST_MAP.to_owned().into();
	assert_eq!(10, start.width());
	assert_eq!(10, start.height());
	let expected: Map = TEST_STEP.to_owned().into();
	assert_eq!(expected, run_step(&start));
	let last: Map = TEST_STEP_2.to_owned().into();
	assert_eq!(last, run_step(&expected));
}
