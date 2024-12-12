use aoc::{makeset, utils::*};

pub fn part1(input: String) -> String {
	use Direction::*;
	let g = Grid::new(&input, ());
	let mut seen = makeset!();
	let mut total_price = 0;
	for (p, c, _) in g.iter() {
		if seen.contains(&p) {
			continue;
		}
		let region = g.flood_search_by_pred(p, |_, d| g.read_pt(&d) == c);
		let borders = g.borders_of_contiguous_area(&region);
		let border_len = borders.into_iter()
			.flat_map(|p| [N, E, S, W].map(|d| g.drive(p, d)))
			.filter(|np| np.is_none_or(|nnp| g.read_pt(&nnp) != c))
			.count();
		total_price += region.len() * border_len;
		seen.extend(region);
	}
	total_price.to_string()
}


pub fn part2(input: String) -> String {
	"part2".to_string()
}
