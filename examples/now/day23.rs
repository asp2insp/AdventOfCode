use aoc::utils::Grid;

fn find_longest_path_to_ends(g: &Grid<()>, start: Point) -> Vec<(Point, usize)> {
	let mut visited = HashSet::new();
	let mut queue = VecDeque::new();
	queue.push_back((start, 0));
	let mut ends = Vec::new();
	while let Some((p, d)) = queue.pop_front() {
		if !visited.
	}
}



pub fn part1(input: String) -> String {
	let mut g = Grid::new(&input, ());
	g.wall_char = '#';
	let (l, bt, r, top) = g.get_bounds();
	let start = g.find_in_range('.', l..=r, top..=top).unwrap();
	let end = g.find_in_range('.', l..=r, 0..=0).unwrap();
}


pub fn part2(input: String) -> String {
	"part2".to_string()
}
