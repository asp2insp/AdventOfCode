use itertools::*;
use crate::utils::*;
use std::collections::VecDeque;

pub fn part1(input: String) -> String {
	let depth = 9171;
	let width = 7; 
	let height = 721;
	let erosion_mod = 20183;
	let mut dp = vec![vec![0usize; width+1]; height+1];
	(0..=width).cartesian_product(0..=height)
		.for_each(|(x, y)| {
			if x == 0 && y == 0 {
				dp[y][x] = (0 + depth) % erosion_mod;
			} else if x == 0 {
				dp[y][x] = (y * 48271 + depth) % erosion_mod;
			} else if y == 0 {
				dp[y][x] = (x * 16807 + depth) % erosion_mod;
			} else {
				let n = dp[y-1][x] * dp[y][x-1];
				dp[y][x] = (n + depth) % erosion_mod;
			}
		});
	dp[height][width] = depth % erosion_mod;
	// print_out(&dp);
	dp.into_iter()
		.flat_map(|it| it.into_iter())
		.map(|n| n % 3)
		.sum::<usize>()
		.to_string()
}

fn neighbors((x, y, e): (usize, usize, usize), width: usize, height: usize) -> impl Iterator<Item = (usize, usize, usize, usize)> {
	let mut res: Vec<(usize, usize, usize, usize)> = veci![
		(x-1, y, e, 1), if x > 0,
		(x, y-1, e, 1), if y > 0,
		(x+1, y, e, 1), if x < width,
		(x, y+1, e, 1), if y < height,
		(x, y, (e + 1) % 3, 7), if true,
		(x, y, (e + 2) % 3, 7), if true,
	];
	res.into_iter()
}

// 0 => c, t = n
// 1 => c, n = t
// 2 => t, n = c

fn print_out(board: &Vec<Vec<usize>>) {
	for l in board.iter() {
		println!("{:?}", l.iter().map(|n| match n%3 {0=>'.', 1=>'=', 2=>'|', _=>' '}).collect::<String>());
	}
}


pub fn part2(input: String) -> String {
	let depth = 9171;
	let width = 7; 
	let height = 721;
	let erosion_mod = 20183;
	let mut dp = vec![vec![usize::MAX; width+51]; height+51];
	(0..=width+50).cartesian_product(0..=height+50)
		.for_each(|(x, y)| {
			if x == 0 && y == 0 {
				dp[y][x] = (0 + depth) % erosion_mod;
			} else if x == 0 {
				dp[y][x] = (y * 48271 + depth) % erosion_mod;
			} else if y == 0 {
				dp[y][x] = (x * 16807 + depth) % erosion_mod;
			} else {
				let n = dp[y-1][x] * dp[y][x-1];
				dp[y][x] = (n + depth) % erosion_mod;
			}
		});
	dp[height][width] = depth % erosion_mod;
	// print_out(&dp);

	let mut seen = dict![(0, 0, 1) => (0, (0, 0, 1))];
	let mut q = VecDeque::new();
	q.push_back((0, 0, 1));
	while let Some(p) = q.pop_front() {
		for (nx, ny, nt, transition) in neighbors(p, width+50, height+50) {
			if dp[ny][nx] % 3 == nt {
				continue; // Impassible with current tool
			}
			let cost = seen.get(&p).map(|(c, _)| c).unwrap_or(&usize::MAX).saturating_add(transition);
			let n = (nx, ny, nt);
			if cost < *seen.get(&n).map(|(c, _)| c).unwrap_or(&usize::MAX) {
				let old = seen.insert(n, (cost, p));
				q.push_back(n);
				if n == (width, height, 1) {
					let mut it = n;
					while it != (0, 0, 1) {
						let (c, prev) = seen[&it];
						// println!("{} {:?}[{}] <- {:?}[{}]", c, prev, dp[prev.1][prev.0] % 3, it, dp[it.1][it.0] % 3);
						it = prev;
					}
				}
			}
		}
		// println!("{}/{}, {}", q.len(), seen.len(), seen.get(&(width, height, 1)).unwrap_or(&usize::MAX));
	}
	seen[&(width, height, 1)].0.to_string()
}