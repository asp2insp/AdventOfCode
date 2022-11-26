use crate::utils::*;
use itertools::*;

fn next_door(d: char, g: &Grid<()>, p: Point) -> Point {
	match d {
		'>' => g.drive_wrap(p, Direction::E),
		'v' => g.drive_wrap(p, Direction::S),
		_ => unreachable!(),
	}
}

fn move_all(g: &mut Grid<()>, d: char) {
	let swaps = g.iter_range(None, None)
	.filter(|(_, c, _)| *c == d)
	.filter_map(|(p, c, t)| {
		let n = next_door(d, g, p);
		if g.read_pt(&n) == '.' {
			Some((p, n))
		} else {
			None
		}
	})
	.collect_vec();
	for (f, t) in swaps {
		g.swap(f, t);
	}
}

pub fn part1(input: String) -> String {
	let mut g = Grid::new(&input, ());
	for i in 1.. {
		let curr = g.to_string();
		move_all(&mut g, '>');
		move_all(&mut g, 'v');
		if g.to_string() == curr {
			return i.to_string()
		}
	}
	unreachable!()
}


pub fn part2(input: String) -> String {
	"part2".to_string()
}


#[test]
fn test() {
	let s = 
	"v...>>.vv>
	.vv>>.vv..
	>>.>v>...v
	>>v>>.>.v.
	v>v.vv.v..
	>.>>..v...
	.vv..>.>v.
	v.v..>>v.v
	....v..v.>";

	assert_eq!("58", part1(s.to_string()));
}