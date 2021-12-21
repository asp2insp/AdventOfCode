use crate::twenty_nineteen::intcode::*;
use defaultmap::DefaultHashMap;
use itertools::*;
use std::collections::{HashSet, VecDeque};
use std::thread;
use std::io::{self, Write};
use crate::utils::Grid;

fn neighbors(x: isize, y: isize) -> impl Iterator<Item = (isize, isize)> {
	vec![(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)].into_iter()
}

fn detect_scaffold(comp: &mut Computer, print: bool) -> (HashSet<(isize, isize)>, bool) {
	let mut v = vec![];
	let mut scaff = HashSet::new();
	let mut x = 0isize;
	let mut y = 0isize;
	let mut alive = false;
	let mut done = false;
	while let ProgYield::Output(c) = comp.run_and_return_output() {
		v.push(c as u8);
		match c as u8 {
			b'#' => {
				scaff.insert((x, y));
				x += 1;
				done = false;
			}
			b'.' => {
				x += 1;
				done = false;
			}
			b'X' => {
				alive = false;
				done = false;
			}
			b'<' | b'^' | b'>' | b'v' => {
				alive = true;
				done = false;
			}
			10 => {
				y += 1;
				x = 0;
				if done {
					break;
				} else {
					done = true;
				}
			}
			_ => {
				done = false;
			}
		};
	}
	if print && !v.is_empty() {
		print!("\x1B[2J");
		print!("{}", &String::from_utf8(v).unwrap());
		io::stdout().flush().unwrap();
	}
	(scaff, alive)
}

pub fn part1(input: String) -> String {
	let mut comp = Computer::new(parse_program(input));
	let scaff = detect_scaffold(&mut comp, false).0;
	scaff
		.iter()
		.filter(|(x, y)| neighbors(*x, *y).all(|p| scaff.contains(&p)))
		.map(|(x, y)| x * y)
		.sum::<isize>()
		.to_string()
}

pub fn part2(input: String) -> String {
	let mut comp = Computer::new(parse_program(input.clone()));
	let scaff = detect_scaffold(&mut comp, false).0;
	let mut comp = Computer::new(parse_program(input));
	comp.set(0, 2);
	let prog = [
		"A,B,C",              // Main
		"L,12,L,8,L,8,L,12", // A
		"R,0,L,0",            // B
		"R,0,L,0",            // C
		"y",                  // Enable video
	];
	prog.iter()
		.flat_map(|l| {
			l.chars()
				.map(|c| c as u8 as isize)
				.chain(std::iter::once(10))
		})
		.for_each(|i| {
			comp.run_and_return_output();
			comp.input(i);
		});
	let wait = std::time::Duration::from_millis(100);
	let mut alive = true;
	while alive {
		alive = detect_scaffold(&mut comp, true).1;
		thread::sleep(wait);
	}
	"part2".to_string()
}
