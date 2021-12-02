use itertools::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Seq(Vec<usize>);

impl From<&str> for Seq {
	fn from(s: &str) -> Seq {
		let v = s.split_whitespace()
					.map(|u| u.parse::<usize>().unwrap())
					.collect();
		Seq(v)
	}
}

#[derive(Debug)]
enum Rule {
	Then(Seq),
	Or(Seq, Seq),
	Base(u8),
}

fn parse_rule(line: &str) -> Option<(usize, Rule)> {
	let mut parts = line.split(':');
	let id = parts.next()?.parse::<usize>().ok()?;
	let body = parts.next()?;
	if body.contains('"') {
		if body.contains('a') {
			Some((id, Rule::Base(b'a')))
		} else {
			Some((id, Rule::Base(b'b')))
		}
	} else if body.contains('|') {
		let (a, b) = body
			.split('|')
			.map(Seq::from)
			.collect_tuple::<(_, _)>()?;
		Some((id, Rule::Or(a, b)))
	} else {
		Some((id, Rule::Then(Seq::from(body))))
	}
}

fn parse_all(input: &str) -> (HashMap<usize, Rule>, Vec<&str>) {
	let mut lines = input.lines();
	let map = input.lines().take_while(|l| !l.is_empty()).map(parse_rule).filter_map(|a| a).collect();
	let res = input.lines().skip_while(|l| !l.is_empty()).collect();
	(map, res)
}

fn is_seq_match<'a>(v: &Vec<usize>, s: &'a [u8], map: &HashMap<usize, Rule>, depth: usize) -> (bool, &'a [u8]) {
		let mut rem = s;
		for (i, r) in v.iter().enumerate() {
			let (g2, rest) = is_match(*r, rem, map, depth + i);
			if !g2 {
				return (false, rem)
			}
			rem = rest;
		}
		(true, rem)
}

fn is_match<'a>(r: usize, s: &'a [u8], map: &HashMap<usize, Rule>, depth: usize) -> (bool, &'a [u8]) {
	println!("{}{}: {}", " ".repeat(depth), r, String::from_utf8_lossy(s));
	if s.is_empty() {
		return (false, s)
	}
	match map.get(&r) {
		None => panic!("WTF dude {} doesn't exist", r),
		Some(Rule::Base(c)) => {
			if c == &s[0] {
				(true, &s[1..])
			} else {
				(false, s)
			}
		}
		Some(Rule::Then(Seq(v))) => is_seq_match(v, s, map, depth+1),
		Some(Rule::Or(left, right)) => {
			let (r_good, r_rem) = is_seq_match(&right.0, s, map, depth+1);
			if r_good {
				return (r_good, r_rem)
			}
			let (l_good, l_rem) = is_seq_match(&left.0, s, map, depth+1);
			if l_good {
				return (l_good, l_rem)
			}
			println!("{}  endif", " ".repeat(depth));
			return (false, s)
		}
	}
}

fn recur_match(rs: Vec<usize>, s: &[u8], map: &HashMap<usize, Rule>) -> bool {
	if s.is_empty() || rs.is_empty() {
		return s.is_empty() && rs.is_empty()
	}
	// println!("{:?} ~ {}", rs, String::from_utf8_lossy(s));
	match map.get(&rs[0]) {
		None => panic!("WTF dude {} doesn't exist", rs[0]),
		Some(Rule::Base(c)) => *c == s[0] && recur_match(rs[1..].to_owned(), &s[1..], map),
		Some(Rule::Then(Seq(v))) => {
			let mut v = v.clone();
			v.extend(rs.clone().into_iter().skip(1));
			recur_match(v, s, map)
		},
		Some(Rule::Or(left, right)) => {
			let mut vl = left.0.clone();
			vl.extend(rs.clone().into_iter().skip(1));
			let mut vr = right.0.clone();
			vr.extend(rs.clone().into_iter().skip(1));
			recur_match(vl, s, map) || recur_match(vr, s, map)
		}
	}
}

fn check(l: &str, rules: &HashMap<usize, Rule>) -> bool {
	let (good, rem) = is_match(0, l.as_bytes(), rules, 0);
	good && rem.is_empty()
}

pub fn part1(input: String) -> String {
	let (rules, lines) = parse_all(&input);
	lines.iter()
		.filter(|l| check(l, &rules))
		.count()
		.to_string()
}

pub fn part2(input: String) -> String {
	let (mut rules, lines) = parse_all(&input);
	// 8: 42 | 42 8
	rules.insert(8, Rule::Or(Seq(vec![42]), Seq(vec![42, 8])));
	// 11: 42 31 | 42 11 31
	rules.insert(11, Rule::Or(Seq(vec![42, 31]), Seq(vec![42, 11, 31])));
	// println!("{:#?}", rules);
	lines.iter()
		.filter(|l| recur_match(vec![0], l.as_bytes(), &rules))
		.count()
		.to_string()
}
