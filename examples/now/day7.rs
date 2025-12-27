use std::cell::Cell;
use std::collections::HashMap;

use aoc::makeset;
use aoc::utils::*;

pub fn part1(input: String) -> String {
    let g = Grid::new(&input, ());
    let splits = Cell::new(0usize);
    let start = g.find('S').unwrap();
    g.bfs_generic(
        makeset! {start},
        Some(&|p| {
            if let Some(('^', _)) = g.get(p) {
                splits.update(|i| i + 1);
                vec![
                    (g.drive(p, Direction::W).map(|p2| (p2, 1))),
                    (g.drive(p, Direction::E).map(|p2| (p2, 1))),
                ].into_iter().flat_map(|i| i).collect()
            } else {
                vec![
                    (g.drive(p, Direction::S).map(|p2| (p2, 1))),
                ].into_iter().flat_map(|i| i).collect()
            }
        }),
        None,
    );
    splits.get().to_string()
}

fn count_paths(start: Point, g: &Grid<()>, memo: &mut HashMap<Point, usize>) -> usize {
	if let Some(res) = memo.get(&start) {
		*res
	} else if start.y == g.bottom_bound {
		1
	} else if g.get(start).unwrap().0 == '^' {
		let res = count_paths(start.offset((-1, 0)), g, memo) + count_paths(start.offset((1, 0)), g, memo);
		memo.insert(start, res);
		res
	} else {
		count_paths(start.offset((0, -1)), g, memo)
	}
}

pub fn part2(input: String) -> String {
    let g = Grid::new(&input, ());
    let start = g.find('S').unwrap();
	let mut memo = HashMap::new();
	count_paths(start, &g, &mut memo).to_string()
}
