use aoc::utils::*;
use itertools::Itertools;


fn parse(s: &str) -> Grid<bool> {
	let nums = gimme_usize_nums(s);
	let pts = nums.into_iter().map(|v| {
		v.chunks(2).map(|c| Point::new(c[0] as isize, c[1] as isize)).collect_vec()
	}).collect_vec();
	let minx = pts.iter().flat_map(|v| v.iter()).min_by_key(|p| p.x).unwrap().x - 1;
	let maxx = pts.iter().flat_map(|v| v.iter()).max_by_key(|p| p.x).unwrap().x + 1;
	let miny = 0;
	let maxy = pts.iter().flat_map(|v| v.iter()).max_by_key(|p| p.y).unwrap().y + 2;
	let invert = maxy - miny;

	let mut g = Grid::new_with_bounds(minx, miny, maxx, maxy, |_| (' ', false));

	for scan in pts {
		for (mut start, mut end) in scan.into_iter().tuple_windows() {
			start.y = invert - start.y;
			end.y = invert - end.y;
			if start.x == end.x {
				(start.y.min(end.y)..=start.y.max(end.y)).for_each(|y| {
					g.set(Point::new(start.x, y), '#', false);
				})
			} else {
				(start.x.min(end.x)..=start.x.max(end.x)).for_each(|x| {
					g.set(Point::new(x, start.y), '#', false);
				})
			}
		}
	}

	g
}

pub fn part1(input: String) -> String {
	let mut g = parse(&input).with_wall('#');
	g.set(Point::new(500, g.top_bound), '+', false);
	println!("{}", g.to_string());
	let mut grains = 0;
	loop {
		let mut p = Point::new(500, g.top_bound);

		loop {
			if p.y <= g.bottom_bound {
				println!("{}", g.to_string());
				return grains.to_string()
			} else if let Some(np) = g.drive(p, Direction::S) {
				// Prefer falling straight
				p = np;
			} else if let Some(np) = g.drive2(p, Direction::S, Direction::W) {
				// Fall diagonally left
				p = np;
			} else if let Some(np) = g.drive2(p, Direction::S, Direction::E) {
				// fall diagonally right
				p = np;
			} else {
				// Blocked, so sand comes to rest
				g.add_wall(p, false);
				g.set(p, 'o', false);
				break
			}
			g.set(p, '~', false);
		}
		grains += 1;
	}
}


pub fn part2(input: String) -> String {
	let mut g = parse(&input);
	g.wall_char = '#';
	g.left_bound = -10000;
	g.right_bound = 10000;
	let source = Point::new(500, g.top_bound);
	for x in g.left_bound..=g.right_bound {
		g.add_wall(Point::new(x, 0), false);
	}
	// println!("{}", g.to_string());
	let mut grains = 0;
	loop {
		let mut p = source;

		loop {
			if let Some(np) = g.drive(p, Direction::S) {
				// Prefer falling straight
				p = np;
			} else if let Some(np) = g.drive2(p, Direction::S, Direction::W) {
				// Fall diagonally left
				p = np;
			} else if let Some(np) = g.drive2(p, Direction::S, Direction::E) {
				// fall diagonally right
				p = np;
			} else {
				// Blocked, so sand comes to rest
				g.set(p, '#', false);
				if p == source {
					// println!("{}", g.to_string());
					return (grains + 1).to_string();
				}
				break
			}
		}
		grains += 1;
	}
}


#[test]
fn test1() {
	let s = r#"498,4 -> 498,6 -> 496,6
	503,4 -> 502,4 -> 502,9 -> 494,9"#.to_owned();
	assert_eq!("24", part1(s));
}


#[test]
fn test2() {
	let s = r#"498,4 -> 498,6 -> 496,6
	503,4 -> 502,4 -> 502,9 -> 494,9"#.to_owned();
	assert_eq!("93", part2(s));
}