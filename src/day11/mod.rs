use std::collections::{HashSet, VecDeque};
use itertools::Itertools;
use regex::Regex;
use self::Object::*;

#[derive(Clone, Debug)]
enum Object {
	Generator(String),
	Microchip(String),
}

fn is_collection_valid(objects: &[Object]) -> bool {
	let mut no_chips = true;
	let mut no_rtgs = true;
	let mut unpaired_chips = HashSet::new();
	let mut unpaired_rtgs = HashSet::new();

	for o in objects {
		match o {
			&Generator(ref name) => {
				no_rtgs = false;
				if unpaired_chips.contains(name) {
					unpaired_chips.remove(name);
					// and we're good
				} else {
					unpaired_rtgs.insert(name.clone());
				}
			},
			&Microchip(ref name) => {
				no_chips = false;
				if unpaired_rtgs.contains(name) {
					unpaired_rtgs.remove(name);
					// and we're good
				} else {
					unpaired_chips.insert(name.clone());
				}
			},
		}
	}
	no_chips || no_rtgs || unpaired_chips.is_empty()
}

fn will_elevator_move(objects: &[Object]) -> bool {
	objects.len() > 0 && objects.len() <= 2
}

fn find_objects(s: &str) -> Vec<Object> {
	let gens = Regex::new(r"a (\w+) generator").unwrap();
	let chips = Regex::new(r"a (\w+)-compatible microchip").unwrap();
	let gen_iter = gens.captures_iter(s)
		.map(|cap| cap.at(1).unwrap().to_owned())
		.map(|gen_name| Generator(gen_name));
	let chip_iter = chips.captures_iter(s)
		.map(|cap| cap.at(1).unwrap().to_owned())
		.map(|gen_name| Microchip(gen_name));
	gen_iter.chain(chip_iter).collect()
}

#[derive(Debug)]
struct State {
	current_floor: usize,
	floors: Vec<Vec<Object>>,
	elevator: Vec<Object>,
	steps: usize,
	key: String,
}

impl State {
	fn new(floors: Vec<Vec<Object>>) -> State {
		let mut s = State {
			current_floor: 0,
			floors: floors,
			elevator: Vec::new(),
			steps: 0,
			key: "".to_string(),
		};
		s.calc_key();
		s
	}

	fn calc_key(&mut self) {
		let floors = self.floors.iter()
			.map(|f| {
				let mut v = f.iter()
					.map(|o| format!("{:?}", o))
					.collect_vec();
				v.sort();
				v
			})
			.collect_vec();
		let mut elevator = self.elevator.iter()
			.map(|o| format!("{:?}", o))
			.collect_vec();
		elevator.sort();
		self.key = format!("{}{:?}{:?}", self.current_floor, floors, elevator);
	}

	fn is_winning(&self) -> bool {
		// Are all floors but the last one empty? are we on the top floor?
		self.current_floor == self.floors.len() - 1 &&
		self.floors[0..(self.floors.len() - 1)].iter()
			.flat_map(|f| f.iter())
			.count() == 0
	}

	fn is_valid(&self) -> bool {
		let elevator_and_floor = self.floors[self.current_floor]
			.iter()
			.cloned()
			.chain(self.elevator.iter().cloned())
			.collect_vec();

		is_collection_valid(&elevator_and_floor) &&
		will_elevator_move(&self.elevator)
	}

	fn next_states(self) -> Vec<State> {
		let exchanges = all_possible_exchanges(
			&self.elevator,
			&self.floors[self.current_floor]
		);
		let mut ret = vec![];
		for ex in exchanges {
			let mut new_floors = self.floors.clone();
			new_floors[self.current_floor] = ex.1.clone();
			if self.current_floor > 0 {
				ret.push(State {
					current_floor: self.current_floor - 1,
					elevator: ex.0.clone(),
					floors: new_floors.clone(),
					steps: self.steps + 1,
					key: "".to_string(),
				});
			}
			if self.current_floor < self.floors.len() - 1 {
				ret.push(State {
					current_floor: self.current_floor + 1,
					elevator: ex.0.clone(),
					floors: new_floors.clone(),
					steps: self.steps + 1,
					key: "".to_string(),
				});
			}
		}
		for r in &mut ret {
			r.calc_key();
		}

		ret.into_iter()
			.filter(State::is_valid)
			//.unique_by(|s| s.key.clone())
			.collect()
	}
}

fn all_possible_exchanges(elevator: &Vec<Object>, floor: &Vec<Object>)
	-> Vec<(Vec<Object>, Vec<Object>)> {
	let all_objs = elevator.iter()
		.chain(floor.iter())
		.collect_vec();
	let mut ret = Vec::<(Vec<Object>, Vec<Object>)>::new();

	for i in 0..all_objs.len() {
		for j in (i+1)..all_objs.len() {
			let oi = all_objs[i].clone();
			let oj = all_objs[j].clone();
			let mut rest = all_objs.iter().map(|o| (*o).clone()).collect_vec();
			rest.remove(j);
			rest.remove(i);
			// Take both on elevator
			ret.push((
				vec![oi.clone(), oj.clone()],
				rest.clone(),
			));
			// Take one on elevator
			let mut rest_with_oi = rest.clone();
			rest_with_oi.push(oi.clone());
			ret.push((
				vec![oj.clone()],
				rest_with_oi,
			));
		}
		let mut rest = all_objs.iter().map(|o| (*o).clone()).collect_vec();
		rest.remove(i);
		ret.push((
			vec![all_objs[i].clone()],
			rest,
		));
	}
	ret
}

fn min_steps(initial: State) -> usize {
	let mut q = VecDeque::new();
	q.push_back(initial);
	let mut seen = HashSet::new();
	let mut count = 0;
	loop {
		let state = q.pop_front().unwrap();
		seen.insert(state.key.clone());
		if state.is_winning() {
			return state.steps
		}
		for new_state in state.next_states() {
			if !seen.contains(&new_state.key) {
				q.push_back(new_state)
			}
		}
		count += 1;
		if count % 10_000 == 0 {
			println!("{}", count);
		}
	}
}

pub fn part1(input: String) -> String {
	let floors = input.lines()
		.map(|l| find_objects(l))
		.collect_vec();
	let initial_state = State::new(floors);
	format!("{}", min_steps(initial_state))
}

pub fn part2(input: String) -> String {
	"part2".to_string()
}
