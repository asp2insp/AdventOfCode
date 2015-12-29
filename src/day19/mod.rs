use std::str;
use chomp::*;
use chomp::ascii::{skip_whitespace,is_uppercase,is_lowercase};

fn maybe_lower(i: Input<u8>) -> U8Result<String> {
	option(i, |i| parse!{i;
		let s = satisfy(is_lowercase);
		ret format!("{}", s as char)
	}, "".to_string())
}

fn atom(i: Input<u8>) -> U8Result<String> {
	parse!{i;
		let s1 = satisfy(is_uppercase);
		let s2 = maybe_lower();
		ret format!("{}{}", s1 as char, s2)
	}
}

fn rule(i: Input<u8>) -> U8Result<(String, Vec<String>)> {
	parse!{i;
		let from = atom();
				   string(b" => ");
		let to   = many1(atom);
				   skip_whitespace();
		ret (from, to)
	}
}

fn all_rules(i: Input<u8>) -> U8Result<Vec<(String, Vec<String>)>> {
	parse!{i;
		let v = many1(rule);
		ret v
	}
}

fn molecule(i: Input<u8>) -> U8Result<Vec<String>> {
	parse!{i;
		let v = many1(atom);
		ret v
	}
}

fn everything(i: Input<u8>) -> U8Result<(Vec<(String, Vec<String>)>, Vec<String>)> {
	parse!{i;
		let rules = all_rules();
					skip_whitespace();
		let mol   = molecule();
		ret (rules, mol)
	}
}

pub fn part1(input: String) -> String {
	format!("{:?}", parse_only(everything, input.as_bytes()).unwrap())
}


pub fn part2(input: String) -> String {
	"part2".to_string()
}
