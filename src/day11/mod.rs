use std::collections::HashSet;
use itertools::Itertools;
use regex::Regex;
use self::Object::*;
use crossbeam::sync::SegQueue;
use crossbeam::scope;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
use std::fmt;


#[derive(Clone)]
enum Object {
	Generator(String),
	Microchip(String),
}

impl fmt::Debug for Object {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Generator(ref n) => write!(f, "G{}", n),
			&Microchip(ref n) => write!(f, "M{}", n),
		}
	}
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
	let gens = Regex::new(r"(\w+) generator").unwrap();
	let chips = Regex::new(r"(\w+)-compatible microchip").unwrap();
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
		let s = State {
			current_floor: 0,
			floors: floors,
			elevator: Vec::new(),
			steps: 0,
			key: "".to_string(),
		};
		s.calc_key()
	}

	fn calc_key(mut self) -> State {
		let floors = self.floors.iter()
			.map(|objects| {
				let mut unpaired_chips = HashSet::new();
				let mut unpaired_rtgs = HashSet::new();
				let mut paired_count = 0;
				for o in objects {
					match o {
						&Generator(ref name) => {
							if unpaired_chips.contains(name) {
								unpaired_chips.remove(name);
								paired_count += 1;
							} else {
								unpaired_rtgs.insert(name.clone());
							}
						},
						&Microchip(ref name) => {
							if unpaired_rtgs.contains(name) {
								unpaired_rtgs.remove(name);
								paired_count += 1;
							} else {
								unpaired_chips.insert(name.clone());
							}
						},
					}
				}
				let mut unpaired_chips = unpaired_chips.into_iter().collect_vec();
				unpaired_chips.sort();
				let mut unpaired_rtgs = unpaired_rtgs.into_iter().collect_vec();
				unpaired_rtgs.sort();
				format!("{}{:?}{:?}", paired_count, unpaired_chips, unpaired_rtgs)
			})
			.collect_vec();
		let mut elevator = self.elevator.iter()
			.map(|o| format!("{:?}", o))
			.collect_vec();
		elevator.sort();
		self.key = format!("{}{:?}{:?}", self.current_floor, floors, elevator);
		self
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

	fn next_states(mut self) -> Vec<State> {
		if self.floors.len() > 1 &&
		   self.floors[0].len() == 0 &&
		   self.current_floor > 1 {
			// Clear lower floors as we empty them
			self.floors.remove(0);
			self.current_floor -= 1;
		}
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
		ret.into_iter()
			.filter(State::is_valid)
			//.unique_by(|s| s.key.clone())
			.map(State::calc_key)
			.collect()
	}
}

fn all_possible_exchanges(elevator: &Vec<Object>, floor: &Vec<Object>)
	-> Vec<(Vec<Object>, Vec<Object>)> {
	let all_objs = elevator.iter()
		.cloned()
		.chain(floor.iter().cloned())
		.collect_vec();
	let mut ret = Vec::<(Vec<Object>, Vec<Object>)>::new();

	for i in 1..all_objs.len() {
		let j = i - 1;
		let mut rest = all_objs.clone();
		let oi = rest.remove(i);
		let oj = rest.remove(j);
		// Take both on elevator
		ret.push((
			vec![oi.clone(), oj.clone()],
			rest.clone(),
		));
		// Take one on elevator
		rest.push(oi);
		ret.push((
			vec![oj],
			rest,
		));
	}
	if !all_objs.is_empty() {
		let mut rest = all_objs;
		let last = rest.pop().unwrap();
		ret.push((
			vec![last],
			rest,
		));
	}
	ret
}

static ID: AtomicUsize = ATOMIC_USIZE_INIT;
static GUARD: AtomicUsize = ATOMIC_USIZE_INIT;

fn t_loop(q: &SegQueue<State>, seen: &Mutex<HashSet<String>>) {
	let mut count = 0;
	let id = ID.fetch_add(1, Ordering::Acquire);
	loop {
		if GUARD.load(Ordering::Relaxed) > 0 {
			break
		}
		let state = match q.try_pop() {
			Some(s) => s,
			None => continue,
		};
		{
			let mut set = seen.lock();
			if set.contains(&state.key) {
				continue;
			}
			set.insert(state.key.clone());
		}
		count += 1;
		if count % 100_000 == 0 {
			println!("{} - {}", id, count);
		}
		if state.is_winning() {
			println!("FOUND: {}", state.steps);
			GUARD.fetch_add(1, Ordering::Acquire);
			break
		}
		for new_state in state.next_states() {
			q.push(new_state);
		}
	}
}

fn min_steps(initial: State) {
	let q = SegQueue::new();
	q.push(initial);
	let seen = Mutex::new(HashSet::new());

	scope(|scope| {
		for _ in 0..4 {
			scope.spawn(|| t_loop(&q, &seen));
		}
	});
}

pub fn part1(input: String) -> String {
	GUARD.store(0, Ordering::SeqCst);
	let floors = input.lines()
		.map(|l| find_objects(l))
		.collect_vec();
	let initial_state = State::new(floors);
	// min_steps(initial_state);
	"Done".to_string()
}

pub fn part2(input: String) -> String {
	GUARD.store(0, Ordering::SeqCst);
	let floors = input.lines()
		.map(|l| find_objects(l))
		.collect_vec();
	let mut initial_state = State::new(floors);
	initial_state.floors[0].push(Generator("elerium".to_string()));
	initial_state.floors[0].push(Generator("dilithium".to_string()));
	initial_state.floors[0].push(Microchip("elerium".to_string()));
	initial_state.floors[0].push(Microchip("dilithium".to_string()));
	min_steps(initial_state.calc_key());
	"Done".to_string()
}
