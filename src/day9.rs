use std::collections::{HashSet, VecDeque};

fn is_valid(slice: &[usize]) -> bool {
	let map: HashSet<usize> = slice.iter().take(25).cloned().collect();
	for n in slice.iter().take(25) {
		if slice[25] <= *n {
			continue;
		}
		if map.contains(&(slice[25] - n)) {
			return true;
		}
	}
	return false;
}

fn find_first_invalid_num(nums: &[usize]) -> Option<usize> {
	for i in 25..nums.len() {
		if !is_valid(&nums[i - 25..=i]) {
			return Some(nums[i]);
		}
	}
	None
}

pub fn part1(input: String) -> String {
	let nums = input
		.lines()
		.map(|l| l.parse::<usize>().unwrap())
		.collect::<Vec<_>>();
	format!("{:?}", find_first_invalid_num(&nums))
}

pub fn part2(input: String) -> String {
	let nums = input
		.lines()
		.map(|l| l.parse::<usize>().unwrap())
		.collect::<Vec<_>>();
	let target = find_first_invalid_num(&nums).unwrap();
	let mut low = 0;
	let mut high = 1;
	loop {
		let current: usize = nums[low..=high].iter().sum();
		if current < target {
			high += 1;
		} else if current > target {
			low += 1;
		} else {
			return (nums[low..=high].iter().max().unwrap()
				+ nums[low..=high].iter().min().unwrap())
			.to_string();
		}
	}
}
