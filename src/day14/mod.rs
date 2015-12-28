use std::str;
use chomp::*;
use chomp::ascii::{skip_whitespace,decimal,is_alpha};
use permutohedron::LexicalPermutation;
use std::collections::{HashMap,HashSet};

fn alpha_string(i: Input<u8>) -> U8Result<String> {
	parse!{i;
		let s = take_while(is_alpha);
		ret str::from_utf8(s).unwrap().to_string()
	}
}

fn reindeer(i: Input<u8>) -> U8Result<Reindeer> {
	parse!{i;
		let n  		   = alpha_string();
				 		 skip_whitespace();
				 		 string(b"can fly");
				 		 skip_whitespace();
		let speed: usize = decimal();
				 		 skip_whitespace();
				 	 	 string(b"km/s for");
				 	 	 skip_whitespace();
		let dur: usize = decimal();
				 		 skip_whitespace();
				 	 	 string(b"seconds, but then must rest for");
				 	 	 skip_whitespace();
		let rest: usize = decimal();
						 skip_whitespace();
						 string(b"seconds.");
						 skip_whitespace();
		ret Reindeer {
			name: n,
			speed: speed,
			duration: dur,
			rest: rest,
		}
	}
}

fn all_reindeer(i: Input<u8>) -> U8Result<Vec<Reindeer>> {
	parse!{i;
		let v = many1(reindeer);
		ret v
	}
}

#[derive(Clone, Debug)]
struct Reindeer {
	name: String,
	speed: usize,
	duration: usize,
	rest: usize,
}

#[derive(Debug)]
enum Mode {
	Running(usize),
	Resting(usize),
}

#[derive(Debug)]
struct Tracker {
	reindeer: Reindeer,
	distance: usize,
	score: usize,

	mode: Mode,
}

impl Tracker {
	/// Runs simulation for 1 second
	fn run(&mut self) {
		// Apply current state
		match self.mode {
			Mode::Running(i) if i > 0 => self.distance += self.reindeer.speed,
			_ => {},
		}
		// Transition to next state
		self.mode = match self.mode {
			Mode::Running(i) if i == 1 => Mode::Resting(self.reindeer.rest),
			Mode::Running(i) => Mode::Running(i-1),
			Mode::Resting(i) if i == 1 => Mode::Running(self.reindeer.duration),
			Mode::Resting(i) => Mode::Resting(i-1),
		}
	}
}

pub fn part1(input: String) -> String {
	let mut trackers:Vec<Tracker> = parse_only(all_reindeer, input.as_bytes())
		.unwrap()
		.iter()
		.map(|r| {
			Tracker {
				reindeer: r.clone(),
				distance: 0,
				score: 0,
				mode: Mode::Running(r.duration),
			}
		})
		.collect();
	for s in 0..2503 {
		for t in &mut trackers {
			t.run();
		}
	}

	let best = trackers.iter().fold(0, |best, t| {
		if t.distance > best {
			t.distance
		} else {
			best
		}
	});
	format!("{}", best)
}


pub fn part2(input: String) -> String {
	let mut trackers:Vec<Tracker> = parse_only(all_reindeer, input.as_bytes())
		.unwrap()
		.iter()
		.map(|r| {
			Tracker {
				reindeer: r.clone(),
				distance: 0,
				score: 0,
				mode: Mode::Running(r.duration),
			}
		})
		.collect();
	for s in 0..2503 {
		for t in &mut trackers {
			t.run();
		}
		let best = trackers.iter().fold(0, |best, t| {
			if t.distance > best {
				t.distance
			} else {
				best
			}
		});
		for t in &mut trackers {
			if t.distance == best {
				t.score += 1;
			}
		}
	}
	let best = trackers.iter().fold(0, |best, t| {
		if t.score > best {
			t.score
		} else {
			best
		}
	});
	format!("{}", best)
}
