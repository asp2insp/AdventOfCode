use self::State::*;
use std::str;

enum State {
	Outside,
	Marker,
	Inside,
}

pub fn decompress(s: &str) -> String {
	let mut ret: Vec<u8> = vec![];
	let mut tmp: Vec<u8> = vec![];
	let mut n_chars = 0;
	let mut n_repeat = 0;
	let mut state = Outside;

	for c in s.bytes() {
		state = match (state, c, n_chars) {
			(Outside, b'(', _) => {
				tmp.clear();
				Marker
			},
			(Outside, _, _) => {
				ret.push(c);
				Outside
			},
			(Marker, b'x', _) => {
				n_chars = str::from_utf8(&tmp).map(str::parse::<usize>).unwrap().unwrap();
				tmp.clear();
				Marker
			},
			(Marker, b')', _) => {
				n_repeat = str::from_utf8(&tmp).map(str::parse::<usize>).unwrap().unwrap();
				tmp.clear();
				Inside
			},
			(Marker, _, _) => {
				tmp.push(c);
				Marker
			},
			(Inside, _, 1) => {
				tmp.push(c);
				for _ in 0..n_repeat {
					ret.extend_from_slice(&tmp);
				}
				tmp.clear();
				Outside
			},
			(Inside, _, n) if n > 1 => {
				tmp.push(c);
				n_chars -= 1;
				Inside
			},
			(Inside, _, _) => unimplemented!(),
		};
	}
	String::from_utf8(ret).unwrap()
}

pub fn decompressed_len(s: &[u8]) -> usize {
	let mut count = 0;
	let mut tmp: Vec<u8> = vec![];
	let mut n_chars = 0;
	let mut n_repeat = 0;
	let mut state = Outside;

	for c in s.iter() {
		state = match (state, c, n_chars) {
			(Outside, &b'(', _) => {
				tmp.clear();
				Marker
			},
			(Outside, _, _) => {
				count += 1;
				Outside
			},
			(Marker, &b'x', _) => {
				n_chars = str::from_utf8(&tmp).map(str::parse::<usize>).unwrap().unwrap();
				tmp.clear();
				Marker
			},
			(Marker, &b')', _) => {
				n_repeat = str::from_utf8(&tmp).map(str::parse::<usize>).unwrap().unwrap();
				tmp.clear();
				Inside
			},
			(Marker, _, _) => {
				tmp.push(*c);
				Marker
			},
			(Inside, _, 1) => {
				tmp.push(*c);
				count += n_repeat * decompressed_len(&tmp);
				tmp.clear();
				Outside
			},
			(Inside, _, n) if n > 1 => {
				tmp.push(*c);
				n_chars -= 1;
				Inside
			},
			(Inside, _, _) => unimplemented!(),
		};
	}
	count
}

pub fn part1(input: String) -> String {
	format!("{}", decompress(&input).len())
}


pub fn part2(input: String) -> String {
	format!("{}", decompressed_len(input.as_bytes()))
}
