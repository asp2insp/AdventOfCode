use chomp::prelude::{U8Input, Buffer, SimpleResult, parse_only, token, many1, take_while1};
use chomp::ascii::{decimal, is_alpha, skip_whitespace};
use itertools::Itertools;
use std::str;

struct Room {
	name: String,
	sector_id: usize,
	checksum: String,
}

fn most_common_letters(s: &str, n: usize) -> String {
	let mut v = s.chars()
		.filter(|c| *c != '-')
		.collect_vec();
	v.sort();
	let letters = v.iter().group_by(|i| *i);
	let mut v2 = letters.into_iter().map(|(c, group)| {
			(*c, group.count())
		})
		.collect_vec();
	v2.sort_by(|a, b| b.1.cmp(&a.1));
	v2.into_iter().map(|a| a.0).take(n).collect()
}

fn room<I: U8Input>(i: I) -> SimpleResult<I, Room> {
    parse!{i;
        let name = take_while1(|c| is_alpha(c) || c == b'-');
		let id = decimal();
				 token(b'[');
        let checksum = take_while1(is_alpha);
				 token(b']');
				 skip_whitespace();
		ret Room {
			name: String::from_utf8(name.into_vec()).unwrap(),
			sector_id: id,
			checksum: String::from_utf8(checksum.into_vec()).unwrap(),
		}
	}
}

fn rooms<I: U8Input>(i: I) -> SimpleResult<I, Vec<Room>> {
	parse!{i;
		let v= many1(room);
		ret v
	}
}

pub fn part1(input: String) -> String {
	let rooms = parse_only(rooms, input.as_bytes()).unwrap();
	assert!(rooms.len() == 935);
	let sum = rooms.iter()
		.filter(|r| {
			most_common_letters(&r.name, 5) == r.checksum
		})
		.fold(0, |s, r| s + r.sector_id);
	format!("{}", sum)
}

fn rotate(s: &str, n: usize) -> String {
	String::from_utf8(s.as_bytes()
		.iter()
		.map(|b| {
			match b {
				&b'-' => b' ',
				c => {
					let ord = *c - b'a';
					let new_ord = (ord as usize + n) % 26;
					b'a' + new_ord as u8
				},
			}
		})
		.collect_vec())
		.unwrap()
}

pub fn part2(input: String) -> String {
	let rooms = parse_only(rooms, input.as_bytes()).unwrap();
	assert!(rooms.len() == 935);
	let target = rooms.iter()
		.filter(|r| {
			most_common_letters(&r.name, 5) == r.checksum
		})
		.find(|r| rotate(&r.name, r.sector_id).contains("northpole"))
		.unwrap();
	format!("{}", target.sector_id)
}
