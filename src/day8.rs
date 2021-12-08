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

//  0000 
//  1  2 
//  1  2
//  3333
//  4  5
//  4  5
//  6666
fn pos_pos(s: &str) -> HashMap<char, HashSet<usize>> {
	let cs = s.chars().collect::<Vec<_>>();
	let mut map = HashMap::new();
	match cs.len() {
		1 => unreachable!(),
		2 => cs.into_iter().for_each(|c| { map.insert(c, makeset![2, 5]); }), // 1
		3 => cs.into_iter().for_each(|c| { map.insert(c, makeset![0, 2, 5]); }), // 7
		4 => cs.into_iter().for_each(|c| { map.insert(c, makeset![1, 2, 3, 5]); }), // 4
		5 => cs.into_iter().for_each(|c| { map.insert(c, makeset![0, 1, 2, 3, 4, 5, 6]); }), // 2, 3, 5
		6 => cs.into_iter().for_each(|c| { map.insert(c, makeset![0, 1, 2, 3, 4, 5 ,6]); }), // 0, 9, 6
		7 => cs.into_iter().for_each(|c| { map.insert(c, makeset![0, 1, 2, 3, 4, 5, 6]); }), // 8
		_ => unreachable!(),
	};
	map
}

fn pos_nums(s: &str) -> HashMap<char, HashSet<usize>> {
	let cs = s.chars().collect::<Vec<_>>();
	let mut map = HashMap::new();
	match cs.len() {
		1 => unreachable!(),
		2 => cs.into_iter().for_each(|c| { map.insert(c, makeset![1]); }), // 1
		3 => cs.into_iter().for_each(|c| { map.insert(c, makeset![7]); }), // 7
		4 => cs.into_iter().for_each(|c| { map.insert(c, makeset![4]); }), // 4
		5 => cs.into_iter().for_each(|c| { map.insert(c, makeset![2, 3, 5]); }), // 2, 3, 5
		6 => cs.into_iter().for_each(|c| { map.insert(c, makeset![0, 6, 9]); }), // 0, 9, 6
		7 => cs.into_iter().for_each(|c| { map.insert(c, makeset![8]); }), // 8
		_ => unreachable!(),
	};
	map
}

fn intersect(a: HashMap<char, HashSet<usize>>, mut b: HashMap<char, HashSet<usize>>) -> HashMap<char, HashSet<usize>> {
	let mut res = HashMap::new();
	let full =  makeset![0, 1, 2, 3, 4, 5, 6];
	for c in &['a', 'b', 'c', 'd', 'e', 'f', 'g'] {
		let l = a.get(c).unwrap_or(&full);
		let r = b.get(c).unwrap_or(&full);
		res.insert(*c, l.intersection(r).cloned().collect());
	}
	res
}

fn union(a: HashMap<char, HashSet<usize>>, mut b: HashMap<char, HashSet<usize>>) -> HashMap<char, HashSet<usize>> {
	let mut res = HashMap::new();
	let empty =  makeset![];
	for c in &['a', 'b', 'c', 'd', 'e', 'f', 'g'] {
		let l = a.get(c).unwrap_or(&empty);
		let r = b.get(c).unwrap_or(&empty);
		res.insert(*c, l.union(r).cloned().collect());
	}
	res
}

fn reduce(mut pps: Vec<HashMap<char, HashSet<usize>>>, mut pns: Vec<HashMap<char, HashSet<usize>>>) -> HashMap<char, usize> {
	let mut pos = pps.pop().unwrap();
	while let Some(next) = pps.pop() {
		pos = intersect(pos, next);
	}
	let mut nums = pns.pop().unwrap();
	while let Some(next) = pns.pop() {
		nums = union(nums, next);
	}
	
	// use 1 and 7 to find the top segment
	let ones = pos.values().find(|s| s.len() == 2).unwrap().clone();
	let (seven_c, seven) = pos.iter().find(|(_, s)| s.len() == 3).unwrap();
	let top_seg = (seven - &ones).into_iter().next().unwrap();
	let seven_c = *seven_c;
	drop(seven);
	
	// Remove top_seg from everyone else
	pos.iter_mut().filter(|(c, _)| **c != seven_c).for_each(|(_, s)| {s.remove(&top_seg);});
	// Remove ones segs from everyone else
	for s in pos.values_mut() {
		if s.len() > 2 {
			s.retain(|elem| !ones.contains(elem));
		}
	}

	// Next find (fours - ones)
	let fours = pos.values().find(|s| s.len() == 2 && s != &&ones).unwrap().clone();
	// remove (fours - ones) from everyone else
	for s in pos.values_mut() {
		if s.len() > 2 {
			s.retain(|elem| !fours.contains(elem));
		}
	}

	// Next, figure out which is the bottom seg by doing 9-4-7
	let mut four_one_seven = makeset![seven_c];
	pos.iter().filter(|(c, s)| s == &&fours).map(|(c, _)| c).for_each(|c| {four_one_seven.insert(*c);});
	pos.iter().filter(|(c, s)| s == &&ones).map(|(c, _)| c).for_each(|c| {four_one_seven.insert(*c);});
	let candidates = nums.iter().filter(|(_, s)| s.contains(&9)).map(|(c, _)| c).cloned().collect::<HashSet<_>>();
	let bot_seg = candidates.difference(&four_one_seven).next().unwrap();
	pos.get_mut(bot_seg).unwrap().retain(|e| *e == 6);
	// Remove bot from everyone else
	for (c,s) in pos.iter_mut() {
		if c != bot_seg {
			s.remove(&6);
		}
	}

	// Next figure out middle seg by contrasting 8 and 0
	let cand_zero = nums.iter().filter(|(_, s)| s.contains(&0)).map(|(c, _)| c).cloned().collect::<HashSet<_>>();
	let cand_eight = nums.iter().filter(|(_, s)| s.contains(&8)).map(|(c, _)| c).cloned().collect::<HashSet<_>>();


	println!("ez {:?} {:?}", cand_zero, cand_eight);

	println!("{:?}", pos);

	pos.into_iter().map(|(c, s)| (c, s.into_iter().next().unwrap())).collect()
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

