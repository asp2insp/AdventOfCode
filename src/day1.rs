use itertools::Itertools;

pub fn part1(input: String) -> String {
	let nums = input.lines().filter_map(|l| l.parse::<u32>().ok());
	let mut count = 0;
	for (a, b) in nums.tuple_windows() {
		if b > a {
			count += 1;
		}
	}
	format!("{}", count)
}


pub fn part2(input: String) -> String {
	let nums = input.lines().filter_map(|l| l.parse::<u32>().ok());
	let mut count = 0;
	let mut prev = 99999;
	for (a, b, c) in nums.tuple_windows() {
		let sum = a + b + c;
		if sum > prev {
			count += 1;
		}
		prev = sum;
	}
	format!("{}", count)
}