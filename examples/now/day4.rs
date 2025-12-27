use crate::utils::*;

fn get_removable(g: &Grid<()>) -> Vec<Point> {
	g.find_all('@')
        .into_iter()
        .filter(|p| {
            g.neighbors_with_diagonals(*p)
                .filter(|n| g.read(n.x, n.y) == '@')
				.count() < 4
		})
		.collect()
}

pub fn part1(input: String) -> String {
    let g = Grid::new(&input, ());
    get_removable(&g)
        .len()
        .to_string()
}

pub fn part2(input: String) -> String {
	let mut g = Grid::new(&input, ());
	let mut r = get_removable(&g);
	let mut count = 0;
	while !r.is_empty() {
		count += r.len();
		r.into_iter().for_each(|p| g.set(p, '.', ()));
		r = get_removable(&g);
	}
    count.to_string()
}
