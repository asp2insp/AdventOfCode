struct Rule {
	max: usize,
	min: usize,
	l: char,
}

fn parse(s: &str) -> (Rule, &str) {
	let parts = s.split(" ").collect::<Vec<_>>();
	let nums = parts[0].split("-").collect::<Vec<_>>();
	(
		Rule {
			min: nums[0].parse::<usize>().unwrap(),
			max: nums[1].parse::<usize>().unwrap(),
			l: parts[1].chars().nth(0).unwrap(),
		},
		parts[2],
	)
}

fn is_valid_one((rule, pass): &(Rule, &str)) -> bool {
	let n = pass.chars().filter(|c| *c == rule.l).count();
	n >= rule.min && n <= rule.max
}

pub fn part1(input: String) -> String {
	input
		.lines()
		.map(parse)
		.filter(is_valid_one)
		.count()
		.to_string()
}

fn is_valid_two((rule, pass): &(Rule, &str)) -> bool {
	let n = pass
		.chars()
		.enumerate()
		.filter(|(i, c)| *c == rule.l && (*i+1 == rule.min || *i+1 == rule.max))
		.count();
	n == 1
}

pub fn part2(input: String) -> String {
	input
		.lines()
		.map(parse)
		.filter(is_valid_two)
		.count()
		.to_string()
}
