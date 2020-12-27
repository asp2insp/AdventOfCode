use regex::Regex;
use std::collections::HashMap;

enum Instr {
	Mask(Vec<u8>),
	Mem(Vec<u8>, Vec<u8>),
}

fn to_bits(n: usize) -> Vec<u8> {
	let mut v = vec![0u8; 36];
	for i in 0..36 {
		v[35 - i] = ((n >> i) & 1) as u8;
	}
	v
}

fn from_bits(bs: &[u8]) -> usize {
	let mut n = 0;
	for i in 0..36 {
		n += (bs[35-i] as usize) << i;
	}
	n
}

fn apply_mask(val: &[u8], mask: &[u8]) -> Vec<u8> {
	val.into_iter().zip(mask.into_iter())
		.map(|(&v, &m)| if m == 3 {v} else {m})
		.collect()
}

fn read_mask(s: &str) -> Vec<u8> {
	s.trim().chars().map(|c| match c {
		'1' => 1,
		'0' => 0,
		'X' => 3,
		_ => unimplemented!(),
	})
	.collect()
}

fn read_mem(s: &str) -> Instr {
	let re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
	let cap = re.captures(s).unwrap();
	Instr::Mem(to_bits(cap[1].parse::<usize>().unwrap()), to_bits(cap[2].parse::<usize>().unwrap()))
}

fn parse_ln(s: &str) -> Instr {
	if s.starts_with("mask") {
		Instr::Mask(read_mask(s.split('=').nth(1).unwrap()))
	} else {
		read_mem(s)
	}
}

pub fn part1(input: String) -> String {
	let mut mem = HashMap::new();
	let mut mask = vec![3u8; 36];
	for l in input.lines() {
		match parse_ln(l) {
			Instr::Mask(nm) => {
				mask = nm;
			},
			Instr::Mem(addr, val) => {
				mem.insert(addr, apply_mask(&val, &mask));
			},
		}
	}
	mem.values().fold(0, |acc, n| acc + from_bits(n)).to_string()
}


fn apply_mem_mask(addr: &[u8], mask: &[u8]) -> Vec<Vec<u8>> {
	if addr.len() == 0 {
		return vec![Vec::with_capacity(36)]
	}
	let mut sub = apply_mem_mask(&addr[..addr.len()-1], &mask[..addr.len()-1]);
	if *mask.last().unwrap() == 0 {
		sub.iter_mut().for_each(|s| s.push(*addr.last().unwrap()));
		sub
	} else if *mask.last().unwrap() == 1 {
		sub.iter_mut().for_each(|s| s.push(1u8));
		sub
	} else {
		// mask is floating
		let mut res = vec![];
		for mut s in sub {
			let mut s2 = s.clone();
			s2.push(1);
			res.push(s2);
			s.push(0);
			res.push(s);
		}
		res
	}
}

pub fn part2(input: String) -> String {
	let mut mem = HashMap::new();
	let mut mask = vec![0u8; 36];
	for l in input.lines() {
		match parse_ln(l) {
			Instr::Mask(nm) => {
				mask = nm;
			},
			Instr::Mem(addr, val) => {
				for a2 in apply_mem_mask(&addr, &mask) {
					mem.insert(a2, val.clone());
				}
			},
		}
	}
	mem.values().fold(0, |acc, n| acc + from_bits(n)).to_string()
}

#[test]
fn test_conv() {
	assert_eq!(to_bits(123),   read_mask("000000000000000000000000000001111011  "));
	assert_eq!(123, from_bits(&read_mask("000000000000000000000000000001111011 ")));
}