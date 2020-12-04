use regex::*;

struct Reaction<'a> {
	inputs: Vec<(usize, &'a str)>,
	output: (usize, &'a str),
}

fn parse_input(input: &str) -> Vec<Reaction> {
	let re = Regex::new(r#"((\d+ [A-Z]+,?)+) => (?P<out>\d+ [A-Z]+)"#).unwrap();
	let ret = input.lines()
		.for_each(|l| {
			println!("{:?}\n", re.captures(l))
		});

	println!("{:?}", ret);
	unimplemented!();
}

pub fn part1(input: String) -> String {
	let reactions = parse_input(&input);
	"part1".to_string()
}


pub fn part2(input: String) -> String {
	"part2".to_string()
}
