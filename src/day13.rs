use itertools::*;
use std::collections::{BTreeMap, VecDeque};
use std::thread;
use crate::intcode::*;

fn recv_draw(c: &mut Computer) -> Option<(isize, isize, isize)> {
	match c.run_and_return_output() {
		ProgYield::Output(x) => Some((
			x,
			c.run_and_return_output().unwrap(),
			c.run_and_return_output().unwrap(),
		)),
		_ => None,
	}
}

pub fn part1(input: String) -> String {
	let mut c = Computer::new(parse_program(input));
	let mut count = 0;
	while let Some((x, y, t)) = recv_draw(&mut c) {
		if t == 2 {
			count += 1;
		}
	}
	count.to_string()
}

fn draw_screen(c: &mut Computer, screen: &mut [[char; 37]; 26], score: &mut isize) -> (usize, usize, ProgYield) {
	let mut ret = (0, 0, ProgYield::Halt);
	loop {
		let next = c.run_and_return_output();
		if let ProgYield::Output(x) = next {
			let y = c.run_and_return_output().unwrap();
			let t = c.run_and_return_output().unwrap();
			if x == -1 && y == 0 {
				*score = t;
			} else {
				screen[y as usize][x as usize] = match t {
					0 => ' ',
					1 => '▚',
					2 => '▒',
					3 => {
						ret.1 = x as usize;
						'▄'
					},
					4 => {
						ret.0 = x as usize;
						'o'
					},
					_ => unreachable!(),
				};
			}
		} else {
			ret.2 = next;
			break;
		}
	}

	let s = screen
		.iter()
		.map(|l| {
			l.iter()
				.cloned()
				.chain(std::iter::once('\n'))
				.collect::<String>()
		})
		.chain(std::iter::once(score.to_string()))
		.collect::<String>();
	print!("\x1B[2J");
	println!("{}", s);
	return ret;
}

pub fn part2(input: String) -> String {
	let mut c = Computer::new(parse_program(input));
	let mut screen = [[' '; 37]; 26];
	let mut score = 0;
	c.set(0, 2);
	let wait = std::time::Duration::from_millis(10);
	loop {
		if let (x, pad, ProgYield::Input) = draw_screen(&mut c, &mut screen, &mut score) {
			thread::sleep(wait);
			if x == pad {
				c.input(0);
			} else if x > pad {
				c.input(1);
			} else if x < pad {
				c.input(-1);
			}
		} else {
			break;
		}
	}
	"^^".to_string()
}
