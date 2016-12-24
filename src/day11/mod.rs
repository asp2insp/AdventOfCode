use std::collections::{HashSet, BinaryHeap};
use itertools::Itertools;
use regex::Regex;
use self::Object::*;
use crossbeam::scope;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
use std::cmp;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
// #[derive(Copy, Hash)]
// enum Element {
// 	Polonium,
// 	Promethium,
// 	Cobalt,
// 	Ruthenium,
// 	Thulium,
// 	Dilithium,
// 	Elerium,
// }

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

fn find_objects(s: &str) -> Vec<Object> {
	let gens = Regex::new(r"(\w+) generator").unwrap();
	let chips = Regex::new(r"(\w+)-compatible microchip").unwrap();
	let gen_iter = gens.captures_iter(s)
		.map(|cap| cap.at(1).unwrap()[..2].to_owned())
		.map(|gen_name| Generator(gen_name));
	let chip_iter = chips.captures_iter(s)
		.map(|cap| cap.at(1).unwrap()[..2].to_owned())
		.map(|gen_name| Microchip(gen_name));
	gen_iter.chain(chip_iter).collect()
}

#[derive(Eq, Clone)]
struct State {
	current_floor: usize,
	floors: Vec<Vec<Object>>,
	elevator: Vec<Object>,
	steps: usize,
}

impl PartialEq for State {
	fn eq(&self, other: &State) -> bool {
		let mut self_hasher = DefaultHasher::new();
		self.hash(&mut self_hasher);
		let mut other_hasher = DefaultHasher::new();
		other.hash(&mut other_hasher);
		self_hasher.finish() == other_hasher.finish()
	}
}

impl Hash for State {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.current_floor.hash(state);
		let _ = self.floors.iter()
			.all(|objects| {
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
				paired_count.hash(state);
				unpaired_chips.hash(state);
				unpaired_rtgs.hash(state);
				true
			});
		let mut elevator = self.elevator.clone();
		elevator.sort();
		elevator.hash(state);
	}
}

impl Ord for State {
    fn cmp(&self, other: &State) -> cmp::Ordering {
		let my_score = self.distance() + self.steps;
		let other_score = other.distance() + other.steps;
        other_score.cmp(&my_score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}


impl State {
	fn new(floors: Vec<Vec<Object>>) -> State {
		State {
			current_floor: 0,
			floors: floors,
			elevator: Vec::new(),
			steps: 0,
		}
	}

	fn distance(&self) -> usize {
		let n_floors = self.floors.len();
		self.floors.iter()
			.enumerate()
			.map(|(i, f)| (n_floors-i) * f.len())
			.sum()
	}

	fn is_winning(&self) -> bool {
		// Are all floors but the last one empty? are we on the top floor?
		self.current_floor == self.floors.len() - 1 &&
		self.floors[0..(self.floors.len() - 1)].iter()
			.map(|f| f.len())
			.sum::<usize>() == 0
	}

	fn is_valid(&self) -> bool {
		let elevator_and_floor = self.floors[self.current_floor]
			.iter()
			.cloned()
			.chain(self.elevator.iter().cloned())
			.collect_vec();

		is_collection_valid(&elevator_and_floor)
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
		let mut ret = Vec::with_capacity(exchanges.len());
		for ex in exchanges {
			let mut new_floors = self.floors.clone();
			new_floors[self.current_floor] = ex.1.clone();
			if self.current_floor > 0 {
				let s = State {
					current_floor: self.current_floor - 1,
					elevator: ex.0.clone(),
					floors: new_floors.clone(),
					steps: self.steps + 1,
				};
				if s.is_valid() {
					ret.push(s);
				}
			}
			if self.current_floor < self.floors.len() - 1 {
				let s = State {
					current_floor: self.current_floor + 1,
					elevator: ex.0,
					floors: new_floors,
					steps: self.steps + 1,
				};
				if s.is_valid() {
					ret.push(s);
				}
			}
		}
		ret
	}
}

fn all_possible_exchanges(elevator: &Vec<Object>, floor: &Vec<Object>)
	-> Vec<(Vec<Object>, Vec<Object>)> {
	let all_objs = elevator.iter()
		.cloned()
		.chain(floor.iter().cloned())
		.collect_vec();
	let mut ret = Vec::<(Vec<Object>, Vec<Object>)>::with_capacity(all_objs.len() * 3);

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

fn t_loop(q: &Mutex<BinaryHeap<State>>, seen: &Mutex<HashSet<State>>) {
	let mut count = 0;
	let id = ID.fetch_add(1, Ordering::Acquire);
	loop {
		if GUARD.load(Ordering::Relaxed) > 0 {
			break
		}
		let mut q_guard = q.lock();
		let state = match q_guard.pop() {
			Some(s) => s,
			None => continue,
		};
		drop(q_guard);
		{
			let mut set = seen.lock();
			if set.contains(&state) {
				continue;
			}
			set.insert(state.clone());
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
		let next = {
			let set = seen.lock();
			state.next_states().into_iter()
				.filter(|next| !set.contains(next))
				.collect_vec()
		};

		let mut q_guard = q.lock();
		for new_state in next {
			q_guard.push(new_state);
		}
	}
}

fn min_steps(initial: State) {
	let q = Mutex::new(BinaryHeap::new());
	q.lock().push(initial);
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
	min_steps(initial_state);
	"Done".to_string()
}

pub fn part2(input: String) -> String {
	GUARD.store(0, Ordering::SeqCst);
	let floors = input.lines()
		.map(|l| find_objects(l))
		.collect_vec();
	let mut initial_state = State::new(floors);
	initial_state.floors[0].push(Generator("el".to_string()));
	initial_state.floors[0].push(Generator("di".to_string()));
	initial_state.floors[0].push(Microchip("el".to_string()));
	initial_state.floors[0].push(Microchip("di".to_string()));
	min_steps(initial_state);
	"Done".to_string()
}
