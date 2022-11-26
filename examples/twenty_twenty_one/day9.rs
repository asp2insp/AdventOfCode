use crate::utils::*;
use std::collections::HashMap;
use itertools::*;

pub fn part1(input: String) -> String {
	let g = Grid::new(&input, ());
	g.iter_range(None, None)
		.filter(|(p, c, _)| g.neighbors(*p).all(|p| g.get(p).unwrap().0 > *c))
		.map(|(_, c, _)| c.to_digit(10).unwrap())
		.map(|d| d+1)
		.sum::<u32>()
		.to_string()
}


pub fn part2(input: String) -> String {
	let g = Grid::new(&input, ());
	g.iter_range(None, None)
		.filter(|(p, c, _)| g.neighbors(*p).all(|p2| g.get(p2).unwrap().0 > *c))
		.map(|(p, _, _)| (p))
		.map(|(p)| g.flood_search_by_pred(p, |fp, tp| {
			let to = g.get(tp).unwrap().0;
			let from = g.get(fp).unwrap().0;
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