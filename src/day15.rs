use crate::utils::*;
use itertools::Itertools;
use std::collections::HashMap;

pub fn part1(input: String) -> String {
    let g = Grid::new(&input, ());
    let (l, b, r, t) = g.get_bounds();
    println!("{}", g.to_string());
    println!(
        "{:?}",
        g.bfs_generic(
            makeset!(Point::new(l, t)),
            Some(|p| g
                .neighbors(p)
                .map(|n| (n, g.get(p).unwrap().0.to_digit(10).unwrap() as isize))
				.collect_vec()),
            Some(|s: &HashMap<Point, _>| s.contains_key(&Point::new(r, b)))
        )
    );
    format!(
        "{:?}",
        g.dfs_path(
            Point::new(l, t),
            Point::new(r, b),
            Some(|p| g.get(p).unwrap().0.to_digit(10).unwrap() as isize),
        )
    )
}

pub fn part2(input: String) -> String {
    "part2".to_string()
}

#[test]
fn test() {
    let s = r"1163751742
	1381373672
	2136511328
	3694931569
	7463417111
	1319128137
	1359912421
	3125421639
	1293138521
	2311944581";

    assert_eq!("40", part1(s.to_string()));
}
