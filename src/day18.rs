

const L: usize = 101;
const R: usize = 102;

use itertools::Itertools;

use crate::utils::div_up;
use std::iter::once;



fn p(s: &str) -> Vec<usize> {
	s.chars().filter_map(|c| match c {
		'[' => Some(L),
		']' => Some(R),
		',' => None,
		c => c.to_digit(10).map(|u| u as usize),
	})
	.collect_vec()
}

fn walk_add(s: &mut Vec<usize>, mut i: usize, n: usize, is_left: bool) {
	while i > 0 && i < s.len() {
		if s[i] < 100 {
			s[i] += n;
			return
		}
		if is_left {
			i -= 1;
		} else {
			i += 1;
		}
	}
}

fn maybe_explode(s: &mut Vec<usize>) -> bool {
	let mut depth = 0;
	for i in 0..s.len() {
		match s[i] {
			R => depth -= 1,
			L => depth += 1,
			_ => {},
		};
		if depth == 5 {
			// Explode!
			// We're assured explodes are always 2 regulars
			let li = s[i+1];
			let ri = s[i+2];
			walk_add(s, i-1, li, true);
			walk_add(s, i+1, ri, false);
			// remove i[ li, ri, ] and replace with 0
			s.splice(i..i+4, once(0));
			return true;
		}
	}
	false
}

fn maybe_split(s: &mut Vec<usize>) -> bool {
	for i in 0..s.len() {
		let n = s[i];
		if n < 100 && n >= 10 {
			s.splice(i..i+1, [L, n/2, div_up(n, 2), R].into_iter());
			return true;
		}
	}
	false
}

fn reduce(mut s: Vec<usize>) -> Vec<usize> {
	while maybe_explode(&mut s) || maybe_split(&mut s) {}
	s
}

fn mag(s: Vec<usize>) -> usize {
	let mut stack = vec![];
	for elem in s.into_iter() {
		if elem == R {
			let r = stack.pop().unwrap() * 2;
			let l = stack.pop().unwrap() * 3;
			stack.push(l + r);
		} else if elem < 100 {
			stack.push(elem);
		}
	}
	assert_eq!(1, stack.len());
	stack.into_iter().next().unwrap()
}

fn add(a: Vec<usize>, b: Vec<usize>) -> Vec<usize> {
	once(L).chain(a.into_iter()).chain(b.into_iter()).chain(once(R)).collect_vec()
}

pub fn part1(input: String) -> String {
	let sum = input.lines()
		.map(p)
		.fold1(|a, b|
			reduce(add(a, b))
		)
		.unwrap();
	mag(sum).to_string()
}


pub fn part2(input: String) -> String {
	"part2".to_string()
}

#[test]
fn test_basics() {
	let a = p("[1,1]");
	let b =  p("[2,2]");
	let c = p("[3,3]");
	let d = p("[4,4]");
	assert_eq!(p("[[[[1,1],[2,2]],[3,3]],[4,4]]"), add(add(add(a, b), c), d));
}

#[test]
fn test_mag() {
	let sum = p("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");
	assert_eq!(4140, mag(sum));
}

#[test]
fn test_reduction() {
	let a = p("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
	assert_eq!(p("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"), reduce(a));
}