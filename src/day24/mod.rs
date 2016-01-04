use chomp::*;
use chomp::ascii::{skip_whitespace,decimal};
use std::collections::HashSet;

fn line(i: Input<u8>) -> U8Result<usize> {
	parse!{i;
		let d: usize = decimal();
					   skip_whitespace();
		ret d
	}
}

fn all_lines(i: Input<u8>) -> U8Result<Vec<usize>> {
	parse!{i;
		let v = many1(line);
		ret v
	}
}

fn push_clone(v: &Vec<usize>, a: usize) -> Vec<usize> {
	let mut v2 = v.clone();
	v2.push(a);
	v2
}

fn sum(v: &[usize]) -> usize {
	v.iter().fold(0, |s, a| s + a)
}

#[inline]
fn max_3(a: usize, b: usize, c: usize) -> usize {
	if a > b {
		if a > c {a} else {c}
	} else {
		if b > c {b} else {c}
	}
}

#[inline]
fn min_3(a: usize, b: usize, c: usize) -> usize {
	if a < b {
		if a < c {a} else {c}
	} else {
		if b < c {b} else {c}
	}
}

#[inline]
fn max_diff(a: usize, b: usize, c: usize) -> usize {
	max_3(a, b, c) - min_3(a, b, c)
}

fn sorted_rep(p: usize, tup: &(Vec<usize>, Vec<usize>, Vec<usize>)) -> u64 {
	let mut v = vec![&tup.0, &tup.1, &tup.2];
	v.sort();
	let mut hash = 1315423911u64;
	hash ^= (hash << 5) + (p as u64) + (hash >> 2);

	for coll in v {
		hash ^= (hash << 5) + 263 + (hash >> 2);
		for i in coll {
			hash ^= (hash << 5) + (*i as u64) + (hash >> 2);
		}
		hash ^= (hash << 5) + 997 + (hash >> 2);
	}
	hash
}

fn equal_parts_3(p: &[usize], prog: (Vec<usize>, Vec<usize>, Vec<usize>), memo: &mut HashSet<u64>)
						    -> Vec<(Vec<usize>, Vec<usize>, Vec<usize>)> {
	if p.len() == 0 {
		let sa = sum(&prog.0);
		let sb = sum(&prog.1);
		let sc = sum(&prog.2);
		return if sa == sb && sa == sc { // Only return if this is a valid solution
			vec![prog]
		} else {
			vec![]
		}
	}

	let d = max_diff(sum(&prog.0), sum(&prog.1), sum(&prog.2));
	if d > sum(p) {
		return vec![]  // if we'll never catch up our smallest to our biggest, abort
	}

	let key = sorted_rep(p[0], &prog); // Compute our key, check our memo
	if memo.contains(&key) {
		return vec![]
	}

	// Otherwise, we'll actually proceed with the computation
	memo.insert(key);
	let mut a = equal_parts_3(&p[1..], (push_clone(&prog.0, p[0]), prog.1.clone(), prog.2.clone()), memo);
	let mut b = equal_parts_3(&p[1..], (prog.0.clone(), push_clone(&prog.1, p[0]), prog.2.clone()), memo);
	let mut c = equal_parts_3(&p[1..], (prog.0, prog.1, push_clone(&prog.2, p[0])), memo);
	a.append(&mut b);
	a.append(&mut c);
	a
}

pub fn part1(input: String) -> String {
	let packages = parse_only(all_lines, input.as_bytes()).unwrap();
	format!("{:?}", equal_parts_3(&packages[..],
			(vec![], vec![], vec![]),
			&mut HashSet::with_capacity(12000000)).len())
}


pub fn part2(input: String) -> String {
	"part2".to_string()
}
