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

struct Bloom64 {
	bits: Vec<u64>,
	n: u64,
	count: usize,
}

impl Bloom64 {
	fn with_cardinality(n: usize) -> Bloom64 {
		Bloom64 {
			bits: vec![0; n],
			n: 64 * n as u64,
			count: 0,
		}
	}

	fn contains(&self, hash_a: u64, hash_b: u64, hash_c: u64) -> bool {
		let ia = ((hash_a % self.n) / 64) as usize;
		let ra = (hash_a % self.n) % 64;
		let a = self.bits[ia] & 1 << ra != 0;

		let ib = ((hash_b % self.n) / 64) as usize;
		let rb = (hash_b % self.n) % 64;
		let b = self.bits[ib] & 1 << rb != 0;

		let ic = ((hash_c % self.n) / 64) as usize;
		let rc = (hash_c % self.n) % 64;
		let c = self.bits[ic] & 1 << rc != 0;

		// Combos
		let iab = (((hash_a ^ hash_b) % self.n) / 64) as usize;
		let rab = ((hash_a ^ hash_b) % self.n) % 64;
		let ab = self.bits[iab] & 1 << rab != 0;

		let ibc = (((hash_b ^ hash_c) % self.n) / 64) as usize;
		let rbc = ((hash_b ^ hash_c) % self.n) % 64;
		let bc = self.bits[ibc] & 1 << rbc != 0;

		let ica = (((hash_a ^ hash_c) % self.n) / 64) as usize;
		let rca = ((hash_a ^ hash_c) % self.n) % 64;
		let ca = self.bits[ica] & 1 << rca != 0;
		a && b && c && ab && bc && ca
	}

	fn insert(&mut self, hash_a: u64, hash_b: u64, hash_c: u64) {
		if self.contains(hash_a, hash_b, hash_c) {
			return
		}
		self.count += 1;

		let ia = ((hash_a % self.n) / 64) as usize;
		let ra = (hash_a % self.n) % 64;
		self.bits[ia] |= 1 << ra;

		let ib = ((hash_b % self.n) / 64) as usize;
		let rb = (hash_b % self.n) % 64;
		self.bits[ib] |= 1 << rb;

		let ic = ((hash_c % self.n) / 64) as usize;
		let rc = (hash_c % self.n) % 64;
		self.bits[ic] |= 1 << rc;

		// Combos
		let iab = (((hash_a ^ hash_b) % self.n) / 64) as usize;
		let rab = ((hash_a ^ hash_b) % self.n) % 64;
		self.bits[iab] |= 1 << rab;

		let ibc = (((hash_c ^ hash_b) % self.n) / 64) as usize;
		let rbc = ((hash_c ^ hash_b) % self.n) % 64;
		self.bits[ibc] |= 1 << rbc;

		let ica = (((hash_a ^ hash_c) % self.n) / 64) as usize;
		let rca = ((hash_a ^ hash_c) % self.n) % 64;
		self.bits[ica] |= 1 << rca;
	}

	fn len(&self) -> usize {
		self.count
	}
}

#[inline]
fn push_clone(v: &Vec<usize>, a: usize) -> Vec<usize> {
	let mut v2 = v.clone();
	v2.push(a);
	v2
}

#[inline]
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

fn hash(p: usize, tup: &(&Vec<usize>, &Vec<usize>, &Vec<usize>)) -> u64 {
	let mut v = vec![tup.0, tup.1, tup.2];
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

fn hash_2(p: usize, tup: &(&Vec<usize>, &Vec<usize>, &Vec<usize>)) -> u64 {
	let mut v = vec![tup.0, tup.1, tup.2];
	v.sort();
	let mut hash = 0u64;
	hash = (p as u64) + (hash << 6) + (hash << 16) - hash;

	for coll in v {
		hash = 263 + (hash << 6) + (hash << 16) - hash;
		for i in coll {
			hash = (*i as u64) + (hash << 6) + (hash << 16) - hash;
		}
		hash = 997 + (hash << 6) + (hash << 16) - hash;
	}
	hash
}

fn hash_3(p: usize, tup: &(&Vec<usize>, &Vec<usize>, &Vec<usize>)) -> u64 {
	let mut v = vec![tup.0, tup.1, tup.2];
	v.sort();
	let mut hash = 0u64;
	hash = (p as u64) + hash * 131;

	for coll in v {
		hash = 263 + hash * 131;
		for i in coll {
			hash = (*i as u64) + hash * 131;
		}
		hash = 997 + hash * 131;
	}
	hash
}

fn equal_parts_3(p: &[usize], prog: (&Vec<usize>, &Vec<usize>, &Vec<usize>), memo: &mut Bloom64, memo2: &mut HashSet<u64>)
						    -> Vec<(Vec<usize>, Vec<usize>, Vec<usize>)> {
	if p.len() == 0 {
		let sa = sum(&prog.0);
		let sb = sum(&prog.1);
		let sc = sum(&prog.2);
		return if sa == sb && sa == sc { // Only return if this is a valid solution
			vec![(prog.0.clone(), prog.1.clone(), prog.2.clone())]
		} else {
			vec![]
		}
	}

	let d = max_diff(sum(&prog.0), sum(&prog.1), sum(&prog.2));
	if d > sum(p) {
		return vec![]  // if we'll never catch up our smallest to our biggest, abort
	}

	let key  = hash(p[0], &prog); // Compute our key, check our memo
	let key2 = hash_2(p[0], &prog);
	let key3 = hash_3(p[0], &prog);
	if memo.contains(key, key2, key3) && memo2.contains(&key3) {
		return vec![]
	}

	// Otherwise, we'll actually proceed with the computation
	memo.insert(key, key2, key3);
	memo2.insert(key3);
	let mut a = equal_parts_3(&p[1..], (&push_clone(&prog.0, p[0]), prog.1, prog.2), memo, memo2);
	let mut b = equal_parts_3(&p[1..], (prog.0, &push_clone(&prog.1, p[0]), prog.2), memo, memo2);
	let mut c = equal_parts_3(&p[1..], (prog.0, prog.1, &push_clone(&prog.2, p[0])), memo, memo2);
	if b.len() > 0 {
		a.append(&mut b);
	}
	if c.len() > 0 {
		a.append(&mut c);
	}
	a
}

pub fn part1(input: String) -> String {
	let packages = parse_only(all_lines, input.as_bytes()).unwrap();
	let mut set: Bloom64 = Bloom64::with_cardinality(50000017);
	let mut set2: HashSet<u64> = HashSet::with_capacity(50000017);
	let n = equal_parts_3(&packages[..], (&vec![], &vec![], &vec![]), &mut set, &mut set2).len();
	println!("Cardinalities. Bloom {}, HT: {}, (-{})", set.len(), set2.len(), set2.len() - set.len());
	format!("{:?}", n)
}


pub fn part2(input: String) -> String {
	"part2".to_string()
}
