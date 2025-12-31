use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use crate::utils::*;

fn parse<'a>(s: &'a str) -> HashMap<&'a str, Vec<&'a str>> {
	s.lines()
		.map(|l| {
			let mut parts = l.split(':');
			let k = parts.next().unwrap();
			let v = parts.next().unwrap().split_whitespace().collect_vec();
			(k, v)
		})
		.collect()
}

fn bfs_count_paths(map: &HashMap<&str, Vec<&str>>, start: &str, end: &str) -> usize {
	// Precompute a reachable set
	let mut reachable = HashSet::new();
	reachable.insert(end);
	let mut size = reachable.len();
	loop {
		for (&k, v) in map.iter() {
			if v.iter().any(|e| reachable.contains(e)) {
				reachable.insert(k);
			}
		}
		if size == reachable.len() {
			break
		}
		size = reachable.len()
	}
	println!("Found reachable set for {} of size {}", end, reachable.len());

	let mut q = VecDeque::new();
	let mut seen = HashSet::new();
	q.push_back(vec![start]);
	let mut out_count = 0;
	while let Some(path_so_far) = q.pop_front() {
		seen.insert(path_so_far.clone());
		for &next in map.get(path_so_far.last().unwrap()).unwrap() {
			if next == end {
				out_count += 1;
				continue
			}
			// Only allow reachable nodes
			if !reachable.contains(next) {
				continue
			}
			let next_path = path_so_far.clone_with(next);
			if seen.contains(&next_path) {
				continue
			}
			q.push_back(next_path);
		}
	}
	out_count
}

fn dfs_count_paths<'a>(map: &HashMap<&'a str, Vec<&'a str>>, state: (&'a str, bool, bool), end: &'a str, memo: &mut HashMap<(&'a str, bool, bool), usize>) -> usize {
	if state.0 == end {
		if state.1 && state.2 {
			return 1
		} else {
			return 0
		}
	}
	if let Some(v) = memo.get(&state) {
		return *v
	}
	let sat_dac = state.1 || state.0 == "dac";
	let sat_fft = state.2 || state.0 == "fft";
	let v = map.get(&state.0).unwrap_or(&vec![]).iter().map(|n| dfs_count_paths(map, (n, sat_dac, sat_fft), end, memo)).sum::<usize>();
	memo.insert(state, v);
	v
}

pub fn part1(input: String) -> String {
	let map = parse(&input);
	bfs_count_paths(&map, "you", "out").to_string()
}


pub fn part2(input: String) -> String {
	let map = parse(&input);
	// let starts = dfs_count_paths(&map, "svr", "dac", &mut HashMap::new()) + dfs_count_paths(&map, "svr", "fft", &mut HashMap::new());
	// let mids = dfs_count_paths(&map, "dac", "fft", &mut HashMap::new()) + dfs_count_paths(&map, "fft", "dac", &mut HashMap::new());
	// let ends = dfs_count_paths(&map, "fft", "out", &mut HashMap::new()) + dfs_count_paths(&map, "dac", "out", &mut HashMap::new());
	// (starts * mids * ends).to_string()

	dfs_count_paths(&map, ("svr", false, false), "out", &mut HashMap::new()).to_string()
}
