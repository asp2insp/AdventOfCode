use crate::utils::*;
use std::collections::HashMap;
use itertools::*;

pub fn part1(input: String) -> String {
	let g = Grid::new(&input, ());
	g.iter_range(None, None)
		.filter(|(x, y, c, _)| g.neighbors(*x, *y).all(|(x2, y2)| g.get(x2, y2).unwrap().0 > *c))
		.map(|(_, _, c, _)| c.to_digit(10).unwrap())
		.map(|d| d+1)
		.sum::<u32>()
		.to_string()
}


pub fn part2(input: String) -> String {
	let g = Grid::new(&input, ());
	g.iter_range(None, None)
		.filter(|(x, y, c, _)| g.neighbors(*x, *y).all(|(x2, y2)| g.get(x2, y2).unwrap().0 > *c))
		.map(|(x, y, _, _)| (x, y))
		.map(|(x, y)| g.flood_search_by_pred(x, y, |fx, fy, tx, ty| {
			let to = g.get(tx, ty).unwrap().0;
			let from = g.get(fx, fy).unwrap().0;
			to != '9' && to > from
		}).len())
		.sorted()
		.rev()
		.take(3)
		.product1::<usize>()
		.unwrap()
		.to_string()
}

#[test]
fn test() {
	let input = r"2199943210
3987894921
9856789892
8767896789
9899965678";

	assert_eq!("1134", part2(input.to_string()));
}