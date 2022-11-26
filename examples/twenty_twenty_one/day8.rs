use std::collections::{HashMap, HashSet};

fn parse(s: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
	s.lines()
		.map(|l| {
			let mut it = l.split("|");
			let pat = it.next().unwrap().trim().split_whitespace().collect::<Vec<_>>();
			let out = it.next().unwrap().trim().split_whitespace().collect::<Vec<_>>();
			(pat, out)
		})
		.collect::<Vec<_>>()
}


pub fn part1(input: String) -> String {
	let lines = parse(&input);
	lines.into_iter()
		.flat_map(|l| l.1.into_iter())
		.filter(|s| match s.len() {
			2 | 3 | 4 | 7 => true,
			_ => false,
		})
		.count()
		.to_string()
}


fn map_to_digits(pats: &[&str]) -> HashMap<Vec<char>, usize> {
	let mut mapping = HashMap::new();
	for p in pats {
		let s = p.chars().collect::<HashSet<_>>();
		match p.len() {
			1 => unreachable!(),
			2 => 
				mapping.insert(1, s),
			
			3 => 
				mapping.insert(7, s),
			
			4 => 
				mapping.insert(4, s),
			7 => mapping.insert(8, s),
			_ => {None},
		};
	}

	for p in pats.iter().filter(|s| s.len() == 6) {
		// 0, 6, or 9
		let s = p.chars().collect::<HashSet<_>>();
		if s.intersection(&mapping[&4]).count() == 4 {
			mapping.insert(9, s);
		} else if s.intersection(&mapping[&7]).count() != 3 {
			mapping.insert(6, s);
		} else {
			mapping.insert(0, s);
		}
	}

	for p in pats.iter().filter(|s| s.len() == 5) {
		// 2, 3, or 5
		let s = p.chars().collect::<HashSet<_>>();
		if s.intersection(&mapping[&1]).count() > 1 {
			mapping.insert(3, s);
		} else if s.intersection(&mapping[&6]).count() == 5 {
			mapping.insert(5, s);
		} else {
			mapping.insert(2, s);
		}
	}
	let mut ret = HashMap::new();
	for (k,v) in mapping {
		let mut s = v.into_iter().collect::<Vec<_>>();
		s.sort();
		ret.insert(s, k);
	}
	ret
}


pub fn part2(input: String) -> String {
	let lines = parse(&input);
	let mut sum = 0;
	for (pats, out) in lines {
		let codex = map_to_digits(&pats);
		let mut partial = 0;
		for i in 0..4 {
			let mut key = out[i].chars().collect::<Vec<_>>();
			key.sort();
			partial *= 10;
			partial += codex[&key];
		}
		// println!("{}", partial);
		sum += partial;
	}
	sum.to_string()
}

