use chomp::*;
use chomp::ascii::{skip_whitespace,is_uppercase,is_lowercase};
use std::collections::{HashSet,VecDeque};

fn maybe_lower(i: Input<u8>) -> U8Result<String> {
	option(i, |i| parse!{i;
		let s = satisfy(is_lowercase);
		ret format!("{}", s as char)
	}, "".to_string())
}

fn atom(i: Input<u8>) -> U8Result<String> {
	parse!{i;
		let s1 = satisfy(is_uppercase);
		let s2 = maybe_lower();
		ret format!("{}{}", s1 as char, s2)
	}
}

fn electron(i: Input<u8>) -> U8Result<String> {
	parse!{i;
		let t = token(b'e');
		ret format!("{}", t as char)
	}
}

fn atom_or_electron(i: Input<u8>) -> U8Result<String> {
	or(i, electron, atom)
}

fn rule(i: Input<u8>) -> U8Result<(String, Vec<String>)> {
	parse!{i;
		let from = atom_or_electron();
				   string(b" => ");
		let to   = many1(atom);
				   skip_whitespace();
		ret (from, to)
	}
}

fn all_rules(i: Input<u8>) -> U8Result<Vec<(String, Vec<String>)>> {
	parse!{i;
		let v = many1(rule);
		ret v
	}
}

fn molecule(i: Input<u8>) -> U8Result<Vec<String>> {
	parse!{i;
		let v = many1(atom_or_electron);
		ret v
	}
}

fn everything(i: Input<u8>) -> U8Result<(Vec<(String, Vec<String>)>, Vec<String>)> {
	parse!{i;
		let rules = all_rules();
					skip_whitespace();
		let mol   = molecule();
		ret (rules, mol)
	}
}

fn join(v: &Vec<String>) -> String {
	v.iter().flat_map(|s| s.chars()).collect()
}

pub fn part1(input: String) -> String {
	let (rules, mol) = parse_only(everything, input.as_bytes()).unwrap();
	let mut set: HashSet<String> = HashSet::new();
	for i in 0..mol.len() {
		for rule in &rules {
			if mol[i] == rule.0 {
				let mut copy = mol.clone();
				copy[i] = join(&rule.1);
				set.insert(join(&copy));
			}
		}
	}
	format!("{:?}", set.len())
}

// fn next(mol: &Vec<String>, rules: &Vec<(String, Vec<String>)>) -> Vec<Vec<String>> {
// 	let mut v: Vec<Vec<String>> = Vec::new();
// 	for i in 0..mol.len() {
// 		for rule in rules {
// 			if mol[i] == rule.0 {
// 				let mut copy = mol.clone();
// 				copy[i] = join(&rule.1);
// 				copy.push(".".to_string()); // terminator
// 				v.push(parse_only(molecule, join(&copy).as_bytes()).unwrap());
// 			}
// 		}
// 	}
// 	v
// }

// fn replace(ins: &String, s: &String, with: &String) -> Vec<String> {
// 	let inp: &str = &ins[..];
// 	let withp: &str = &with[..];
//
// 	let mut v: Vec<String> = Vec::new();
// 	for im in ins.match_indices(s) {
// 		v.push(
// 			vec![
// 				&inp[..im.0], withp, &inp[im.0+im.1.len()..],
// 			].iter()
// 			 .flat_map(|s| s.chars())
// 			 .collect()
// 		);
// 	}
// 	v
// }
//
// #[inline]
// fn contains(ins: &String, s: &String) -> bool {
// 	let inp: &str = &ins[..];
// 	let sp: &str = &s[..];
// 	inp.contains(sp)
// }

// fn prev(mol: &String, rules: &Vec<(String, String)>) -> Vec<String> {
// 	let mut v: Vec<String> = Vec::new();
// 	let mut check_set: HashSet<String> = HashSet::new();
// 	for rule in rules {
// 		if rule.0 == "e".to_string() && mol.len() != rule.1.len() {
// 			continue; // Skip all "e" rules unless it can match the full molecule
// 		}
// 		if contains(&mol, &rule.1) {
// 			for replacement in &mut replace(&mol, &rule.1, &rule.0) {
// 				if !check_set.contains(replacement) {
// 					v.push(replacement.clone());
// 					check_set.insert(replacement.clone());
// 				}
// 			}
// 		}
// 	}
// 	v
// }

// fn prev_longest(mol: &String, rules: &Vec<(String, String)>) -> Vec<String> {
// 	let mut v: Vec<String> = Vec::new();
// 	for rule in rules {
// 		if rule.0 == "e".to_string() && mol.len() != rule.1.len() {
// 			continue; // Skip all "e" rules unless it can match the full molecule
// 		}
// 		if contains(&mol, &rule.1) {
// 			for replacement in &replace(&mol, &rule.1, &rule.0) {
// 				v.push(replacement.clone());
// 				break;
// 			}
// 			if v.len() > 0 {
// 				break;
// 			}
// 		}
// 	}
// 	println!("{:?}", v);
// 	v
// }

pub fn part2(input: String) -> String {
	let (rules, mol) = parse_only(everything, input.as_bytes()).unwrap();
	let tokens = mol.iter().count();
	let count_rnar = mol.iter().filter(|t| &t[..] == "Rn" || &t[..] == "Ar").count();
	let count_y = mol.iter().filter(|t| &t[..] == "Y").count();
	format!("{}", tokens - count_rnar - 2*count_y - 1)
}
