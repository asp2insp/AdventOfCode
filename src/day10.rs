use itertools::*;
use num::Integer;
use std::collections::HashMap;

fn parse_grid(input: &str) -> Vec<Vec<char>> {
	input.lines().map(|l| l.trim().chars().collect()).collect()
}

fn asteroids(grid: &Vec<Vec<char>>) -> impl Iterator<Item = (usize, usize)> + '_ {
	grid.iter()
		.enumerate()
		.flat_map(|(y, r)| std::iter::repeat(y).zip(r.iter().enumerate()))
		.filter(|(y, (x, c))| **c == '#')
		.map(|(y, (x, c))| (y, x))
}

fn calc_score(y: usize, x: usize, grid: &Vec<Vec<char>>) -> usize {
	asteroids(&grid)
		.filter(|coords| *coords != (y, x))
		.map(|(cy, cx)| {
			let rise = cy as isize - y as isize;
			let run = cx as isize - x as isize;
			let denom = rise.gcd(&run);
			(rise / denom, run / denom)
		})
		.unique()
		.count()
}

pub fn part1(input: String) -> String {
	let grid = parse_grid(&input);
	let ans = asteroids(&grid)
		.map(|(y, x)| (y, x, calc_score(y, x, &grid)))
		.max_by_key(|(cy, cx, s)| *s)
		.unwrap();
	format!("{:?}", ans)
}

fn count_by_angle(
	y: usize,
	x: usize,
	grid: &Vec<Vec<char>>,
) -> HashMap<(isize, isize), Vec<(usize, usize)>> {
	let mut order = asteroids(&grid)
		.filter(|coords| *coords != (y, x))
		.map(|(cy, cx)| {
			let rise = cy as isize - y as isize;
			let run = cx as isize - x as isize;
			let denom = rise.gcd(&run);
			let (rise, run) = (rise / denom, run / denom);
			let (rise, run) = match (rise, run) {
				(0, x) if x >= 0 => (0, 1),
				(0, x) if x < 0 => (0, -1),
				(y, 0) if y >= 0 => (1, 0),
				(y, 0) if y < 0 => (-1, 0),
				(y, x) => (y, x),
			};
			((rise, run), (cy, cx))
		})
		.fold(HashMap::new(), |mut hm, (angle, c)| {
			hm.entry(angle).or_insert(Vec::new()).push(c);
			hm
		});
	order.values_mut().for_each(|v| {
		v.sort_by_key(|c2| dist(y, x, c2.1, c2.0));
	});
	order
}

fn dist(x: usize, y: usize, x2: usize, y2: usize) -> usize {
	hypotenuse(x2 as isize - x as isize, y2 as isize - y as isize)
}

fn hypotenuse(rise: isize, run: isize) -> usize {
	(((rise.pow(2) + run.pow(2)) as f32).sqrt() * 100.0) as usize
}

fn slope_to_angle((rise, run): (isize, isize)) -> f32 {
	let theta = (-rise as f32).atan2(run as f32); // Negative rise since y axis points down for us
	let mut shifted = -(theta - std::f32::consts::FRAC_PI_2); // Shift so we point up and reverse
	if shifted < 0.0 {
		shifted += std::f32::consts::PI * 2.0;
	}
	shifted
}

fn destroy_order((cy, cx): (usize, usize), grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
	let mut counts = count_by_angle(cy, cx, &grid);
	let mut order = counts.keys().cloned().collect::<Vec<_>>();
	order.sort_by(|slope1, slope2| slope_to_angle(*slope1).partial_cmp(&slope_to_angle(*slope2)).unwrap());
	let mut destroyed = vec![];
	let mut n = 0;
	while counts.values().map(Vec::len).any(|l| l > 0) {
		let a = order[n % order.len()];
		if counts[&a].len() > 0 {
			let ret = counts.get_mut(&a).unwrap().remove(0);
			destroyed.push(ret);
			// println!("Destroyed {:?} at rise/run {:?}, angle: ({})", ret, a, slope_to_angle(a));
		}
		n += 1;
	}
	destroyed
}

pub fn part2(input: String) -> String {
	let grid = parse_grid(&input);
	let (cy, cx) = (18, 20);
	let destroyed = destroy_order((cy, cx), &grid);
	format!("{}", destroyed[199].1 * 100 + destroyed[199].0)
}


#[test]
fn test_small() {
let input = r#".#....#####...#..
			   ##...##.#####..##
			   ##...#...#.#####.
			   ..#.....X...###..
			   ..#.#.....#....##"#;

	let (cy, cx) = (3, 8);
	let grid = parse_grid(&input);
	let destroyed = destroy_order((cy, cx), &grid);
	println!("{:?}", destroyed);
}