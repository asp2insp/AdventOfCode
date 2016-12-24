use std::collections::{HashSet, BinaryHeap};
use itertools::Itertools;
use regex::Regex;
use self::Element::*;
use crossbeam::scope;
use parking_lot::{Mutex, RwLock};
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
use std::cmp;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Hash)]
#[repr(u8)]
enum Element {
	Polonium,
	Promethium,
	Cobalt,
	Ruthenium,
	Thulium,
	Dilithium,
	Elerium,
}

impl <'a> From<&'a str> for Element {
	fn from(s: &'a str) -> Element {
		match s {
			"polonium" => Polonium,
			"promethium" => Promethium,
			"cobalt" => Cobalt,
			"ruthenium" => Ruthenium,
			"thulium" => Thulium,
			"dilithium" => Dilithium,
			"elerium" => Elerium,
			_ => unimplemented!(),
		}
	}
}

const MASK_BOTTOM: u64 = 0xFFFF_FFFF;

fn is_collection_valid(objects: u64) -> bool {
	let chips = objects & MASK_BOTTOM;
	let rtgs = objects >> 32;
	rtgs == 0 || chips == 0 || (rtgs ^ chips) & chips == 0
}

fn find_objects(s: &str) -> u64 {
	let gens = Regex::new(r"(\w+) generator").unwrap();
	let chips = Regex::new(r"(\w+)-compatible microchip").unwrap();
	let gens = gens.captures_iter(s)
		.map(|cap| {
			let e: Element = cap.at(1).unwrap().into();
			e
		})
		.fold(0, |g, element| g| 1 << element as usize);
	let chips = chips.captures_iter(s)
		.map(|cap| {
			let e: Element = cap.at(1).unwrap().into();
			e
		})
		.fold(0, |c, element| c | 1 << element as usize);

	gens << 32 | chips
}

const NUM_FLOORS: usize = 4;

#[derive(Eq, Clone)]
struct State {
	current_floor: usize,
	floors: [u64; NUM_FLOORS],
	elevator: u64,
	steps: usize,
}

impl PartialEq for State {
	fn eq(&self, other: &State) -> bool {
		self.current_floor.eq(&other.current_floor) &&
		self.elevator.eq(&other.elevator) &&
		self.floors.eq(&other.floors)
	}
}

impl Hash for State {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.current_floor.hash(state);
		self.floors.hash(state);
		self.elevator.hash(state);
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
	fn new(floors: Vec<u64>) -> State {
		let mut v = [0; NUM_FLOORS];
		v.copy_from_slice(&floors);
		State {
			current_floor: 0,
			floors: v,
			elevator: 0,
			steps: 0,
		}
	}

	fn distance(&self) -> usize {
		self.floors.iter()
			.enumerate()
			.map(|(i, f)| (NUM_FLOORS-i) * f.count_ones() as usize)
			.sum()
	}

	fn is_winning(&self) -> bool {
		// Are all floors but the last one empty? are we on the top floor?
		self.current_floor == self.floors.len() - 1 &&
		self.floors[0..(self.floors.len() - 1)]
			.iter()
			.sum::<u64>() == 0
	}

	fn is_valid(&self) -> bool {
		let elevator_and_floor = self.elevator | self.floors[self.current_floor];
		is_collection_valid(elevator_and_floor)
	}

	fn next_states(self) -> Vec<State> {
		let floor_below_empty =
			self.current_floor > 1 &&
			self.floors[self.current_floor - 1] == 0;
		let exchanges = all_possible_exchanges(
			self.elevator,
			self.floors[self.current_floor]
		);
		let mut ret = Vec::with_capacity(exchanges.len());
		for ex in exchanges {
			let mut new_floors = self.floors.clone();
			new_floors[self.current_floor] = ex.1.clone();
			if self.current_floor > 0 && ! floor_below_empty {
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

fn all_possible_exchanges(elevator: u64, floor: u64) -> Vec<(u64, u64)> {
	let all_objs = elevator | floor;

	let mut ret = Vec::<(u64, u64)>::new();

	for i in 1..64 {
		let j = i-1;
		let oi = 1 << i;
		let oj = 1 << j;
		if oj & all_objs != 0 {
			// take j on elevator
			ret.push((oj, all_objs - oj));

			if oi & all_objs != 0 {
				// take both on elevator
				ret.push((oi + oj, all_objs - oi - oj));
			}
		}
	}
	ret
}

static ID: AtomicUsize = ATOMIC_USIZE_INIT;
static GUARD: AtomicUsize = ATOMIC_USIZE_INIT;

fn t_loop(q: &Mutex<BinaryHeap<State>>, seen: &RwLock<HashSet<State>>) {
	let mut count = 0;
	let id = ID.fetch_add(1, Ordering::Acquire);
	let mut cached: Option<State> = None;
	loop {
		if GUARD.load(Ordering::Relaxed) > 0 {
			break
		}
		let state = match cached.take() {
			Some(s) => s,
			None => {
				let mut q_guard = q.lock();
				match q_guard.pop() {
					Some(s) => s,
					None => continue,
				}
			}
		};
		let mut set = seen.write();
		if set.contains(&state) {
			continue;
		}
		set.insert(state.clone());
		drop(set);

		count += 1;
		if count % 500_000 == 0 {
			println!("{} - {}", id, count);
		}
		if state.is_winning() {
			println!("FOUND: {}", state.steps);
			GUARD.fetch_add(1, Ordering::Acquire);
			break
		}

		let next = state.next_states();
		let set = seen.read();
		let next = next.into_iter()
			.filter(|next| !set.contains(next))
			.collect_vec();
		drop(set);

		let mut next_heap = BinaryHeap::with_capacity(next.len());
		next_heap.extend(next);

		let mut q_guard = q.lock();
		q_guard.append(&mut next_heap);
		cached = q_guard.pop();
	}
}

fn min_steps(initial: State) {
	let q = Mutex::new(BinaryHeap::new());
	q.lock().push(initial);
	let seen = RwLock::new(HashSet::new());

	scope(|scope| {
		for _ in 0..3 {
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
	let mut floors = input.lines()
		.map(|l| find_objects(l))
		.collect_vec();
	let extras = "An elerium generator An elerium-compatible microchip. A dilithium generator A dilithium-compatible microchip";
	let extra_objects = find_objects(extras);
	floors[0] = floors[0] | extra_objects;

	let initial_state = State::new(floors);
	min_steps(initial_state);
	"Done".to_string()
}
