use chomp::*;
use chomp::ascii::{skip_whitespace,decimal};

fn num_line(i: Input<u8>) -> U8Result<usize> {
	parse!{i;
		let n: usize = decimal();
					   skip_whitespace();
		ret n
	}
}

fn all_nums(i: Input<u8>) -> U8Result<Vec<usize>> {
	parse!{i;
		let v = many1(num_line);
		ret v
	}
}

fn combs(target: usize, sizes: &[usize]) -> usize {
	if sizes.len() == 0 {
		if target == 0 {1} else {0}
	} else if sizes[0] > target {
		// skip it
		combs(target, &sizes[1..])
	} else {
		// use it or lose it
		combs(target-sizes[0], &sizes[1..]) + combs(target, &sizes[1..])
	}
}

fn min(a: usize, b: usize) -> usize {
	if a < b {
		a
	} else {
		b
	}
}

fn min_combo(target: usize, sizes: &[usize]) -> usize {
	if sizes.len() == 1 {
		if sizes[0] == target {
			1
		} else {
			4096
		}
	} else if sizes[0] > target {
		// skip it
		min_combo(target, &sizes[1..])
	} else {
		// use it or lose it
		min(
			1 + min_combo(target-sizes[0], &sizes[1..]),
			min_combo(target, &sizes[1..])
		)
	}
}

fn combs_limited(target: usize, sizes: &[usize], bins: usize) -> usize {
	if sizes.len() == 0 || bins == 0 {
		if target == 0 {1} else {0}
	} else if sizes[0] > target {
		// skip it
		combs_limited(target, &sizes[1..], bins)
	} else {
		// use it or lose it
		combs_limited(target-sizes[0], &sizes[1..], bins-1) + combs_limited(target, &sizes[1..], bins)
	}
}

pub fn part1(input: String) -> String {
	let nums = parse_only(all_nums, input.as_bytes()).unwrap();
	format!("{}", combs(150, &nums[..]))
}


pub fn part2(input: String) -> String {
	let nums = parse_only(all_nums, input.as_bytes()).unwrap();
	let min_bins = min_combo(150, &nums[..]);
	format!("{}", combs_limited(150, &nums[..], min_bins))
}
