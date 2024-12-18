use aoc::makeset;
use aoc::utils::*;
use itertools::Itertools;

pub fn part1(input: String) -> String {
    let nums = gimme_nums(&input)
        .into_iter()
        .map(|l| Point::new(l[0], l[1]))
        .collect_vec();
    let g: Grid<()> = Grid::new_with_bounds(0, 0, 70, 70, |p| {
        if nums[0..1024].contains(&p) {
            ('#', ())
        } else {
            ('.', ())
        }
    }).with_wall('#');
	// g.dfs_path(Point::new(0,0), Point::new(70,70), None).0.to_string()
	g.bfs_generic(makeset!(Point::new(0,0)), None, None)
		.get(&Point::new(70,70))
		.unwrap()
		.0
		.to_string()
}

pub fn part2(input: String) -> String {
    let nums = gimme_nums(&input)
        .into_iter()
        .map(|l| Point::new(l[0], l[1]))
        .collect_vec();
    let mut g: Grid<()> = Grid::new_with_bounds(0, 0, 70, 70, |p| {
        if nums[0..1024].contains(&p) {
            ('#', ())
        } else {
            ('.', ())
        }
    }).with_wall('#');
	let mut n = 1024;
	while g.bfs_generic(makeset!(Point::new(0,0)), None, None).contains_key(&Point::new(70,70)) {
		g.add_wall(nums[n], ());
		n += 1;
	}
	nums[n-1].to_debug_string()
}
