use chomp::*;
use chomp::ascii::{skip_whitespace,decimal};

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

// struct Bloom64 {
// 	bits: Vec<u64>,
// 	bits2: Vec<u64>,
// 	n: u64,
// 	count: usize,
// }
//
// impl Bloom64 {
// 	fn with_cardinality(n: usize) -> Bloom64 {
// 		Bloom64 {
// 			bits: vec![0; n],
// 			bits2: vec![0; n],
// 			n: 64 * n as u64,
// 			count: 0,
// 		}
// 	}
//
// 	fn contains(&self, hash_a: u64, hash_b: u64, hash_c: u64) -> bool {
// 		let ia = ((hash_a % self.n) / 64) as usize;
// 		let ra = (hash_a % self.n) % 64;
// 		let a = self.bits[ia] & 1 << ra != 0;
//
// 		let ib = ((hash_b % self.n) / 64) as usize;
// 		let rb = (hash_b % self.n) % 64;
// 		let b = self.bits[ib] & 1 << rb != 0;
//
// 		let ic = ((hash_c % self.n) / 64) as usize;
// 		let rc = (hash_c % self.n) % 64;
// 		let c = self.bits[ic] & 1 << rc != 0;
//
// 		// Combos
// 		let iab = (((hash_a ^ hash_b) % self.n) / 64) as usize;
// 		let rab = ((hash_a ^ hash_b) % self.n) % 64;
// 		let ab = self.bits2[iab] & 1 << rab != 0;
//
// 		let ibc = (((hash_b ^ hash_c) % self.n) / 64) as usize;
// 		let rbc = ((hash_b ^ hash_c) % self.n) % 64;
// 		let bc = self.bits2[ibc] & 1 << rbc != 0;
//
// 		let ica = (((hash_a ^ hash_c) % self.n) / 64) as usize;
// 		let rca = ((hash_a ^ hash_c) % self.n) % 64;
// 		let ca = self.bits2[ica] & 1 << rca != 0;
//
// 		let iabc = (((hash_a ^ hash_b ^ hash_c) % self.n) / 64) as usize;
// 		let rabc = ((hash_a ^ hash_b ^ hash_c) % self.n) % 64;
// 		let abc = self.bits2[iabc] & 1 << rabc != 0;
// 		a && b && c && ab && bc && ca && abc
// 	}
//
// 	fn insert(&mut self, hash_a: u64, hash_b: u64, hash_c: u64) {
// 		if self.contains(hash_a, hash_b, hash_c) {
// 			return
// 		}
// 		self.count += 1;
//
// 		let ia = ((hash_a % self.n) / 64) as usize;
// 		let ra = (hash_a % self.n) % 64;
// 		self.bits[ia] |= 1 << ra;
//
// 		let ib = ((hash_b % self.n) / 64) as usize;
// 		let rb = (hash_b % self.n) % 64;
// 		self.bits[ib] |= 1 << rb;
//
// 		let ic = ((hash_c % self.n) / 64) as usize;
// 		let rc = (hash_c % self.n) % 64;
// 		self.bits[ic] |= 1 << rc;
//
// 		// Combos
// 		let iab = (((hash_a ^ hash_b) % self.n) / 64) as usize;
// 		let rab = ((hash_a ^ hash_b) % self.n) % 64;
// 		self.bits2[iab] |= 1 << rab;
//
// 		let ibc = (((hash_c ^ hash_b) % self.n) / 64) as usize;
// 		let rbc = ((hash_c ^ hash_b) % self.n) % 64;
// 		self.bits2[ibc] |= 1 << rbc;
//
// 		let ica = (((hash_a ^ hash_c) % self.n) / 64) as usize;
// 		let rca = ((hash_a ^ hash_c) % self.n) % 64;
// 		self.bits2[ica] |= 1 << rca;
//
// 		let iabc = (((hash_a ^ hash_b ^ hash_c) % self.n) / 64) as usize;
// 		let rabc = ((hash_a ^ hash_b ^ hash_c) % self.n) % 64;
// 		self.bits2[iabc] |= 1 << rabc;
// 	}
//
// 	fn len(&self) -> usize {
// 		self.count
// 	}
// }

// struct Bits32 {
// 	bits: Vec<u64>,
// 	count: usize,
// }
//
// impl Bits32 {
// 	fn new() -> Bits32 {
// 		Bits32 {
// 			bits: vec![0; 67_108_864],
// 			count: 0,
// 		}
// 	}
//
// 	fn contains(&self, hash: u32) -> bool {
// 		let ia = (hash / 64) as usize;
// 		let ra = hash % 64;
// 		let a = self.bits[ia] & 1 << ra != 0;
// 		a
// 	}
//
// 	fn insert(&mut self, hash: u32) {
// 		if self.contains(hash) {
// 			return
// 		}
// 		self.count += 1;
//
// 		let ia = (hash / 64) as usize;
// 		let ra = hash % 64;
// 		self.bits[ia] |= 1 << ra;
// 	}
//
// 	fn len(&self) -> usize {
// 		self.count
// 	}
// }
//
// #[inline]
// fn push_clone(v: &Vec<usize>, a: usize) -> Vec<usize> {
// 	let mut v2 = v.clone();
// 	v2.push(a);
// 	v2
// }
//
//
// #[inline]
// fn max(v: &Vec<usize>) -> usize {
// 	v.iter().fold(0, |m, a| if *a > m {*a} else {m})
// }
//
// #[inline]
// fn min(v: &Vec<usize>) -> usize {
// 	v.iter().fold(usize::max_value(), |m, a| if *a < m {*a} else {m})
// }
//
// #[inline]
// fn max_diff(vecs: &Vec<usize>) -> usize {
// 	max(vecs) - min(vecs)
// }
//
// fn hash(p: usize, tup: &Vec<Vec<usize>>) -> u32 {
// 	let mut v: Vec<Vec<usize>> = tup.clone();
// 	v.sort();
// 	let mut hash = 1315423911u32;
// 	hash ^= (hash << 5) + (p as u32) + (hash >> 2);
//
// 	for coll in v {
// 		hash ^= (hash << 5) + 263 + (hash >> 2);
// 		for i in coll {
// 			hash ^= (hash << 5) + (i as u32) + (hash >> 2);
// 		}
// 		hash ^= (hash << 5) + 997 + (hash >> 2);
// 	}
// 	hash
// }
//
// fn hash_2(p: usize, tup: &(&Vec<usize>, &Vec<usize>, &Vec<usize>)) -> u32 {
// 	let mut v = vec![tup.0, tup.1, tup.2];
// 	v.sort();
// 	let mut hash = 0u32;
// 	hash = (p as u64) + (hash << 6) + (hash << 16) - hash;
//
// 	for coll in v {
// 		hash = 263 + (hash << 6) + (hash << 16) - hash;
// 		for i in coll {
// 			hash = (*i as u64) + (hash << 6) + (hash << 16) - hash;
// 		}
// 		hash = 997 + (hash << 6) + (hash << 16) - hash;
// 	}
// 	hash
// }
//
// fn hash_3(p: usize, tup: &(&Vec<usize>, &Vec<usize>, &Vec<usize>)) -> u32 {
// 	let mut v = vec![tup.0, tup.1, tup.2];
// 	v.sort();
// 	let mut hash = 0u32;
// 	hash = (p as u64) + hash * 131;
//
// 	for coll in v {
// 		hash = 263 + hash * 131;
// 		for i in coll {
// 			hash = (*i as u64) + hash * 131;
// 		}
// 		hash = 997 + hash * 131;
// 	}
// 	hash
// }

// fn equal_parts_n(p: &[usize], prog: Vec<Vec<usize>>, memo: &mut Bits32, target: usize)
// 						    -> LinkedList<Vec<Vec<usize>>> {
// 	let c = if p.len() > 0 {p[0]} else {211};
// 	let key  = hash(c, &prog); // Compute our key, check our memo
// 	if memo.contains(key) {
// 		return LinkedList::new()
// 	}
// 	// Otherwise, we'll actually proceed with the computation
// 	memo.insert(key);
//
// 	let sums: Vec<usize> = prog.iter().map(|p| sum(p)).collect();
// 	if p.len() == 0 {
// 		let mut ll = LinkedList::new();
// 		if sums.iter().fold(true, |b, i| b && *i == target) { // Only return if this is a valid solution
// 			ll.push_back(vec![prog[0].clone(), prog[1].clone(), prog[2].clone()]);
// 		}
// 		return ll
// 	}
//
// 	if max_diff(&sums) > sum(p) {
// 		return LinkedList::new() // Once we build too far on one partition, abort
// 	}
//
// 	let mut ll: LinkedList<Vec<Vec<usize>>> = LinkedList::new();
// 	for i in 0..prog.len() {
// 		if sum(&prog[i]) + p[0] > target {
// 			continue;
//  		}
// 		let mut v: Vec<Vec<usize>> = Vec::new();
// 		for j in 0..prog.len() {
// 			if j != i {
// 				v.push(prog[j].clone());
// 			}
// 		}
// 		v.push(push_clone(&prog[i], p[0]));
// 		let mut a = equal_parts_n(&p[1..], v, memo, target);
// 		ll.append(& mut a);
// 	}
// 	ll
// }

#[inline]
fn sum(v: &[usize]) -> usize {
	v.iter().fold(0, |s, a| s + a)
}

fn combine_min(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
	if a.len() == 0 {
		b.clone()
	} else if b.len() == 0 {
		a.clone()
	} else if a[0].len() == b[0].len() {
		let mut v = a.clone();
		v.append(&mut b.clone());
		v
	} else if a[0].len() < b[0].len() {
		a.clone()
	} else {
		b.clone()
	}
}

fn min_sets(p: &[usize], target: usize) -> Option<Vec<Vec<usize>>> {
	// base case
	if p.len() == 1 {
		if p[0] == target {
			Some(vec![vec![p[0].clone()]])
		} else {
			None
		}
	} else if p[0] < target {
		let use_it = match min_sets(&p[1..], target - p[0]) {
			Some(mut vs) => {
				for v in &mut vs {
					v.push(p[0]);
				}
				Some(vs)
			},
			None => None,
		};
		let lose_it = min_sets(&p[1..], target);
		match (use_it, lose_it) {
			(Some(v1), Some(v2)) => {
				Some(combine_min(&v1, &v2))
			},
			(Some(v1), None) => Some(v1),
			(None, Some(v2)) => Some(v2),
			(None, None) => None,
		}
	} else {
		min_sets(&p[1..], target)
	}
}

pub fn part1(input: String) -> String {
	let mut packages = parse_only(all_lines, input.as_bytes()).unwrap();
	packages.reverse();
	let target = sum(&packages) / 3;
	println!("Target: {}", target);
	let m_sets = min_sets(&packages, target).unwrap();
	let min_tangle = m_sets.iter().fold(usize::max_value(), |m, a| {
		let tangle = a.iter().fold(1, |p, a| p * a);
		if tangle < m {
			tangle
		} else {
			m
		}
	});
	format!("{}", min_tangle)
}


pub fn part2(input: String) -> String {
	let mut packages = parse_only(all_lines, input.as_bytes()).unwrap();
	packages.reverse();
	let target = sum(&packages) / 4;
	println!("Target: {}", target);
	let m_sets = min_sets(&packages, target).unwrap();
	let min_tangle = m_sets.iter().fold(usize::max_value(), |m, a| {
		let tangle = a.iter().fold(1, |p, a| p * a);
		if tangle < m {
			tangle
		} else {
			m
		}
	});
	format!("{}", min_tangle)
}
