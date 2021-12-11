use crate::utils::*;
use std::{collections::{VecDeque, HashSet}, ops::Add};

pub fn part1(input: String) -> String {
	let mut g = Grid::new_with(&input, |c| c.to_digit(10).unwrap());
	let mut total_flashes = 0;
	for step in 0..100 {
		// First, all energy increases by 1
		g.for_each_mut(None, None, |(_, e)| *e += 1);
		// Next start flashes + chain reaction
		let mut starts = g.iter_range(None, None).filter_map(|(p, _, e)| if *e > 9 {Some(p)} else {None}).collect::<HashSet<Point>>();
		let mut q = starts.iter().cloned().collect::<VecDeque<_>>();
		while let Some(p) = q.pop_front() {
			for i in g.neighbors_with_diagonals(p) {
				if let Some((_, e)) = g.get_mut(i) {
					// If we're about to trigger a flash, add it
					if *e == 9 {
						starts.insert(i);
						q.push_back(i);
					}
					*e += 1;
				}
			}
		}
		total_flashes += starts.len();
		// All flashing octopi use up their energy
		starts.into_iter().for_each(|i| {
			if let Some((_, e)) = g.get_mut(i) {
				*e = 0;
			}
		});
	}

	total_flashes.to_string()
}


pub fn part2(input: String) -> String {
	let mut g = Grid::new_with(&input, |c| c.to_digit(10).unwrap());
	for step in 1.. {
		// First, all energy increases by 1
		g.for_each_mut(None, None, |(_, e)| *e += 1);
		// Next start flashes + chain reaction
		let mut starts = g.iter_range(None, None).filter_map(|(p, _, e)| if *e > 9 {Some(p)} else {None}).collect::<HashSet<Point>>();
		let mut q = starts.iter().cloned().collect::<VecDeque<_>>();
		while let Some(p) = q.pop_front() {
			for i in g.neighbors_with_diagonals(p) {
				if let Some((_, e)) = g.get_mut(i) {
					// If we're about to trigger a flash, add it
					if *e == 9 {
						starts.insert(i);
						q.push_back(i);
					}
					*e += 1;
				}
			}
		}
		if starts.len() == 100 {
			return step.to_string()
		}
		// All flashing octopi use up their energy
		starts.into_iter().for_each(|i| {
			if let Some((_, e)) = g.get_mut(i) {
				*e = 0;
			}
		});
	}
	unreachable!()
}

#[test]
fn test() {
	let s = r"5483143223
	2745854711
	5264556173
	6141336146
	6357385478
	4167524645
	2176841721
	6882881134
	4846848554
	5283751526";

	assert_eq!("1656", part1(s.to_string()));
	assert_eq!("195", part2(s.to_string()));
}