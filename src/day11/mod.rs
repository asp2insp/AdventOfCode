use itertools::Itertools;
use std::collections::HashSet;

fn add1(s: String) -> String {
	let mut rep: Vec<char> = s.chars().collect();
	let mut i = rep.len() - 1;
	loop {
		if rep[i] == 'z' {
			rep[i] = 'a';
			i -= 1;
		} else {
			rep[i] = (rep[i] as u8 + 1u8) as char;
			break;
		}
	}
	rep.iter().map(|c| c.clone()).collect()
}

fn is_valid(s: &String) -> bool {
	 let has_straight = s.chars()
	 	.fold(('\0', '\0', false), |state, c| {
			let mut is_straight = state.0 as u32 + 1 == state.1 as u32;
			is_straight &= state.1 as u32 + 1 == c as u32;
			(state.1, c, state.2 | is_straight)
		}).2;
	let free_of_confusion = s.chars()
		.fold(true, |good, c| {
			good && c != 'i' && c != 'o' && c != 'l'
		});
	let has_pairs = s.chars()
		.group_by(|c| c.clone())
		.filter(|kg| kg.1.len() > 1)
		.fold(HashSet::<char>::new(), |mut map, kg| {
			map.insert(kg.0.clone());
			map
		})
		.len() > 1;
	let valid = has_straight && free_of_confusion && has_pairs;
	// println!("{} > {}", s, valid);
	valid
}

pub fn part1(input: String) -> String {
	let mut s = input.clone();
	loop {
		if is_valid(&s) {
			break;
		} else {
			s = add1(s);
		}
	}
	s
}


pub fn part2(input: String) -> String {
	let mut s = input.clone();
	let mut first = false;
	loop {
		if is_valid(&s) {
			if first {
				break;
			} else {
				first = true;
			}
		}
		s = add1(s);
	}
	s
}
