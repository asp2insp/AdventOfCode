use std::str;
use chomp::*;
use chomp::ascii::{skip_whitespace,decimal,is_alpha};
use permutohedron::LexicalPermutation;
use std::collections::{HashMap,HashSet};

fn gain_or_lose(i: Input<u8>) -> U8Result<i64> {
	or(i,
		|i| parse!{i;
			string(b"gain");
			ret 1i64
		},
		|i| parse!{i;
			string(b"lose");
			ret -1i64
		})
}

fn alpha_string(i: Input<u8>) -> U8Result<String> {
	parse!{i;
		let s = take_while(is_alpha);
		ret str::from_utf8(s).unwrap().to_string()
	}
}

fn happy(i: Input<u8>) -> U8Result<(String, String, i64)> {
	parse!{i;
		let n1 = alpha_string();
				 skip_whitespace();
				 string(b"would");
				 skip_whitespace();
		let pm = gain_or_lose();
				 skip_whitespace();
		let hu: i64 = decimal();
				 skip_whitespace();
				 string(b"happiness units by sitting next to");
				 skip_whitespace();
		let n2 = alpha_string();
				 token(b'.');
				 skip_whitespace();
		ret (n1, n2, hu*pm)
	}
}

fn all_happy(i: Input<u8>) -> U8Result<Vec<(String, String, i64)>> {
	parse!{i;
		let v = many1(happy);
		ret v
	}
}

fn abs(i: &i64) -> u64 {
	if *i < 0 {
		(i * -1i64) as u64
	} else {
		*i as u64
	}
}

fn calc_happy(ord: &[&String], m: &HashMap<String, i64>) -> (i64, u64) {
	let mut total_happy = 0i64;
	let mut change_happy = 0u64;
	// Calculate everyone but the first and last
	for i in 1..(ord.len()-1) {
		let left_key = format!("{}{}", ord[i], ord[i-1]).to_string();
		let right_key = format!("{}{}", ord[i], ord[i+1]).to_string();
		total_happy += m.get(&left_key).unwrap() + m.get(&right_key).unwrap();
		change_happy += abs(m.get(&left_key).unwrap()) + abs(m.get(&right_key).unwrap());
	}
	// Special case the first
	let left_key = format!("{}{}", ord[0], ord[ord.len()-1]).to_string();
	let right_key = format!("{}{}", ord[0], ord[1]).to_string();
	total_happy += m.get(&left_key).unwrap() + m.get(&right_key).unwrap();
	change_happy += abs(m.get(&left_key).unwrap()) + abs(m.get(&right_key).unwrap());
	// Special case the last
	let left_key = format!("{}{}", ord[ord.len()-1], ord[ord.len()-2]).to_string();
	let right_key = format!("{}{}", ord[ord.len()-1], ord[0]).to_string();
	total_happy += m.get(&left_key).unwrap() + m.get(&right_key).unwrap();
	change_happy += abs(m.get(&left_key).unwrap()) + abs(m.get(&right_key).unwrap());

	(total_happy, change_happy)
}


pub fn part1(input: String) -> String {
	let mut m: HashMap<String, i64> = HashMap::new();
	let mut names: HashSet<String> = HashSet::new();
	for tup in parse_only(all_happy, input.as_bytes()).unwrap() {
		m.insert(format!("{}{}", tup.0, tup.1).to_string(), tup.2);
		names.insert(tup.0.clone());
		names.insert(tup.1.clone());
	}

	// Get the lowest permutation
	let mut start = names.iter().collect::<Vec<&String>>().into_boxed_slice();
	while !start.prev_permutation() {}

	let mut best = (0, 0);
	loop {
		let cand = calc_happy(&start, &m);
		if cand.0 > best.0 {
			best = cand;
		}
		if !start.next_permutation() {
			break;
		}
	}
	format!("{:?}", best.0)
}


pub fn part2(input: String) -> String {
	let mut m: HashMap<String, i64> = HashMap::new();
	let mut names: HashSet<String> = HashSet::new();
	names.insert("me".to_string());
	let me = "me".to_string();
	for tup in parse_only(all_happy, input.as_bytes()).unwrap() {
		m.insert(format!("{}{}", tup.0, tup.1).to_string(), tup.2);

		m.insert(format!("{}{}", tup.0, me).to_string(), 0);
		m.insert(format!("{}{}", me, tup.0).to_string(), 0);
		m.insert(format!("{}{}", tup.1, me).to_string(), 0);
		m.insert(format!("{}{}", me, tup.1).to_string(), 0);

		names.insert(tup.0.clone());
		names.insert(tup.1.clone());
	}

	// Get the lowest permutation
	let mut start = names.iter().collect::<Vec<&String>>().into_boxed_slice();
	while !start.prev_permutation() {}

	let mut best = (0, 0);
	loop {
		let cand = calc_happy(&start, &m);
		if cand.0 > best.0 {
			best = cand;
		}
		if !start.next_permutation() {
			break;
		}
	}
	format!("{:?}", best.0)
}
