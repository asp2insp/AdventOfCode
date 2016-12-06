use std::str;
use chomp::*;
use chomp::ascii::{skip_whitespace,is_alpha,decimal};
use std::collections::HashMap;

fn alpha_string(i: Input<u8>) -> U8Result<String> {
	parse!{i;
		let s = take_while(is_alpha);
		ret str::from_utf8(s).unwrap().to_string()
	}
}

fn maybe_comma(i: Input<u8>) -> U8Result<u8> {
	option(i, |i| token(i, b','), b',')
}

fn characteristic(i: Input<u8>) -> U8Result<(String, usize)> {
	parse!{i;
					   skip_whitespace();
		let n        = alpha_string();
					   token(b':');
					   skip_whitespace();
		let d: usize = decimal();
					   maybe_comma();
					   skip_whitespace();
		ret (n, d)
	}
}

fn sue(i: Input<u8>) -> U8Result<Sue> {
	parse!{i;
						 string(b"Sue ");
		let n: usize   = decimal();
						 token(b':');
		let chars      = many1(characteristic);
						 skip_whitespace();
		ret Sue::new(n, chars)
	}
}

fn all_sues(i: Input<u8>) -> U8Result<Vec<Sue>> {
	parse!{i;
		let v = many1(sue);
		ret v
	}
}

#[derive(Clone, Debug)]
struct Sue {
	num: usize,
	chars: HashMap<String, usize>,
}

impl Sue {
	fn new(n: usize, chars: Vec<(String, usize)>) -> Sue {
		let mut map: HashMap<String, usize> = HashMap::new();
		for pair in chars {
			map.insert(pair.0.clone(), pair.1);
		}
		Sue {
			num: n,
			chars: map,
		}
	}

	fn matches(&self, chars: &Vec<(String, usize)>) -> bool {
		chars.iter()
			.fold(true, |b, pair| {
				b && *self.chars.get(&pair.0).unwrap_or(&pair.1) == pair.1
			})
	}

	fn matches_range(&self, chars: &Vec<(String, usize)>) -> bool {
		chars.iter()
			.fold(true, |b, pair| {
				b && match &pair.0.as_ref() {
					&"cats" | &"trees" => self.chars
						.get(&pair.0)
						.map(|v| *v > pair.1)
						.unwrap_or(true),
					&"pomeranians" | &"goldfish" => self.chars
						.get(&pair.0)
						.map(|v| *v < pair.1)
						.unwrap_or(true),
					_ => self.chars
						.get(&pair.0)
						.map(|v| *v == pair.1)
						.unwrap_or(true),
				}
			})
	}
}

pub fn part1(input: String) -> String {
	let sues = parse_only(all_sues, input.as_bytes()).unwrap();
	let desired: Vec<(String, usize)> = vec![
		("children", 3),
		("cats", 7),
		("samoyeds", 2),
		("pomeranians", 3),
		("akitas", 0),
		("vizslas", 0),
		("goldfish", 5),
		("trees", 3),
		("cars", 2),
		("perfumes", 1),
	].iter().map(|p| (p.0.to_string(), p.1)).collect();
	let res = sues.iter()
		.fold(123usize, |res, sue| {
			if sue.matches(&desired) {
				sue.num
			} else {
				res
			}
		});
	format!("{}", res)
}


pub fn part2(input: String) -> String {
	let sues = parse_only(all_sues, input.as_bytes()).unwrap();
	let desired: Vec<(String, usize)> = vec![
		("children", 3),
		("cats", 7),
		("samoyeds", 2),
		("pomeranians", 3),
		("akitas", 0),
		("vizslas", 0),
		("goldfish", 5),
		("trees", 3),
		("cars", 2),
		("perfumes", 1),
	].iter().map(|p| (p.0.to_string(), p.1)).collect();
	let res = sues.iter()
		.fold(123usize, |res, sue| {
			if sue.matches_range(&desired) {
				sue.num
			} else {
				res
			}
		});
	format!("{}", res)
}
