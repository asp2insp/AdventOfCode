use crate::utils::*;
use itertools::*;

fn parse(s: &str) -> (Vec<char>, Grid<bool>) {
	let mut iter = s.split("\n\n");
	let decode = iter.next().unwrap().trim().chars().collect_vec();
	let g = Grid::new_with(iter.next().unwrap().trim(), |c| c == '#');
	(decode, g)
}

fn enhance(g: &Grid<bool>, dec: &[char], default: char) -> Grid<bool> {
	let margin = 3;
	let (l, b, r, t) = g.calc_bounds();
	let g2 = Grid::new_with_bounds(l-margin, b-margin, r+margin, t+margin, |p| {
		let score: Vec<u8> = g.three_by_three(p, default).map(|c| match c {
			'#' => 1,
			'.' => 0,
			_ => unreachable!(),
		}).collect_vec();
		let idx = from_bits(&score);
		#[cfg(test)]
		println!("{:?} => {:?} == {} => {}", p, score, idx, dec[idx]);
		match dec[idx] {
			'#' => ('#', true),
			'.' => ('.', false),
			_ => unreachable!(),
		}
	});
	#[cfg(test)]
	println!("{}", g2.to_string());
	g2
}

pub fn part1(input: String) -> String {
	let (dec, mut g) = parse(&input);
	enhance(&enhance(&g, &dec, '.'), &dec, '#').iter_range(None, None).filter(|(_, _, &on)| on).count().to_string()
}


pub fn part2(input: String) -> String {
	let (dec, mut g) = parse(&input);
	let mut default = '.';
	for _ in 0..50 {
		g = enhance(&g, &dec, default);
		default = toggle(default, '.', '#');
	}
	g.iter_range(None, None).filter(|(_, _, &on)| on).count().to_string()
}


#[test]
fn test_example() {
	let s = r"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

	let (dec, g) = parse(s);
	let g22 = enhance(&enhance(&g, &dec, '.'), &dec, '.');
	// println!("{}", g22.to_string());
	assert_eq!(35, g22.iter_range(None, None).filter(|(_, _, &on)| on).count());
}