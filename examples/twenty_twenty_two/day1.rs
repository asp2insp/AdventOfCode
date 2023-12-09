use std::collections::BinaryHeap;

use aoc::parse;
use itertools::Itertools;

pub fn part1(input: String) -> String {
    let mut most = 0;
	let mut curr = 0;
	for l in input.lines() {
		if l.is_empty() {
			if curr > most {
				most = curr;
			}
			curr = 0;
		} else {
		curr += parse!(l, isize);
		}
	}
	most.to_string()
}

pub fn part2(input: String) -> String {
	let mut heap = BinaryHeap::new();
	let mut curr = 0;
    for l in input.lines() {
		if l.is_empty() {
			heap.push(curr);
			curr = 0;
		} else {
		curr += parse!(l, isize);
		}
	}
	(0..3).map(|_| heap.pop().unwrap()).sum::<isize>().to_string()
}
