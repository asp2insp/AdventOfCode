use std::collections::{HashMap, VecDeque};
use itertools::Itertools;
use aoc::makeset;
use aoc::utils::{Grid, Point, Direction};
use aoc::utils::CloneWith;

// fn find_longest_path_to_ends(g: &Grid<()>, start: Point) -> Vec<(Point, usize)> {
// 	let mut visited = HashSet::new();
// 	let mut queue = VecDeque::new();
// 	queue.push_back((start, 0));
// 	let mut ends = Vec::new();
// 	while let Some((p, d)) = queue.pop_front() {
// 		if !visited.
// 	}
// }

fn pair_dists(pts: &[(Point, char)], g: &Grid<()>) -> HashMap<Point, HashMap<Point, isize>> {
	let mut ret = HashMap::new();
	for i in 0..pts.len() {
		for j in 0..pts.len() {
			if i == j {
				continue;
			}
			let reachable = g.bfs_generic(
				makeset!(pts[i].0), 
				Some(&|pt| 
					g.neighbors_with_directions(pt).filter(|(dir, n)| g.read_pt(n) == '.' || ( 
						*n == pts[j].0 &&
						match (g.read_pt(n), dir) {
							('>', Direction::W) => true,
							('<', Direction::E) => true,
							('^', Direction::N) => true,
							('v',  Direction::S) => true,
							_ => false,
						}
					)).map(|(_, n)| (n, 1)).collect_vec()), 
				Some(&|dists| dists.contains_key(&pts[j].0))
			);
			if let Some((dist, _)) = reachable.get(&pts[j].0) {
				ret.entry(pts[j].0).or_insert(HashMap::new()).insert(pts[i].0, *dist);
			}
		}
	}
	ret
}



pub fn part1(input: String) -> String {
	let mut g = Grid::new(&input, ());
	g.wall_char = '#';
	let (l, bt, r, top) = g.get_bounds();
	let start = g.find_in_range('.', l..=r, top..=top).unwrap();
	let end = g.find_in_range('.', l..=r, 0..=0).unwrap();
	let mut junctions = g.iter_chars().filter(|(_, c)| "<>v^".contains(*c)).collect_vec();
	junctions.push((start, '.'));
	junctions.push((end, '.'));
	let adj_list = pair_dists(&junctions, &g);
	assert!(adj_list.contains_key(&start));

	let mut s = String::new();
	for a in adj_list {
		for b in a.1 {
			s.push_str(&format!("\"{},{}\" -> \"{},{}\" [label={}]\n", a.0.x, a.0.y, b.0.x, b.0.y, b.1));
		}
	}
	println!("{}", s);
	s

	// let mut max_dist = 0;
	// let mut q = VecDeque::new();
	// q.push_back((start, 0, makeset!(start)));
	// while let Some((n, d, visited)) = q.pop_front() {
	// 	if n == end && d > max_dist {
	// 		max_dist = d;
	// 		continue;
	// 	}
	// 	for (adj, dist) in adj_list.get(&n).unwrap_or(&HashMap::new()) {
	// 		if !visited.contains(adj) {
	// 			q.push_back((*adj, d+dist, visited.clone_with(*adj)));
	// 		}
	// 	}
	// }
	// format!("{:?}", max_dist)
}


pub fn part2(input: String) -> String {
	"part2".to_string()
}
