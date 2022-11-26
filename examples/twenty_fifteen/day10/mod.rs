use itertools::Itertools;

fn look_and_say(s: String) -> String {
	let subs: Vec<String> = s.chars()
		.group_by(|c| c.clone())
		.map(|kg| format!("{}{}", kg.1.len(), kg.0))
		.collect();
	subs.iter()
		.flat_map(|s| s.chars())
		.collect()
}

pub fn part1(input: String) -> String {
	let mut s = input.clone();
	for _ in 0..40 {
		s = look_and_say(s);
	}
	format!("{}", s.len())
}


pub fn part2(input: String) -> String {
	let mut s = input.clone();
	for _ in 0..50 {
		s = look_and_say(s);
	}
	format!("{}", s.len())
}
