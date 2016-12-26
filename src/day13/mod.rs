use std::collections::{HashSet, BinaryHeap};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

const INPUT: usize = 1364;
const GOAL: Point = Point {
	x: 31,
	y: 39,
	// x: 7, y: 4,
	steps: 0,
};

fn is_wall(x: usize, y: usize) -> bool {
	let z = x*x + 3*x + 2*x*y + y + y*y + INPUT;
	z.count_ones() % 2 == 1
}

#[derive(Eq, Clone)]
struct Point {
	x: usize,
	y: usize,
	steps: usize,
}

impl Hash for Point {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.x.hash(state);
		self.y.hash(state);
	}
}

impl PartialEq for Point {
	fn eq(&self, other: &Point) -> bool {
		self.x == other.x && self.y == other.y
	}
}

impl Point {
	fn distance_to(&self, other: &Point) -> usize {
		(other.x as isize - (self.x as isize)).abs() as usize +
		(other.y as isize - (self.y as isize)).abs() as usize
	}
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
		let self_score = self.distance_to(&GOAL) + self.steps;
		let other_score = other.distance_to(&GOAL) + other.steps;
		other_score.cmp(&self_score)
    }
}

fn next(p: Point, seen: &HashSet<Point>) -> Vec<Point> {
	let mut v = Vec::with_capacity(4);
	if p.x > 0 {
		v.push(Point{
			x: p.x - 1,
			y: p.y,
			steps: p.steps + 1,
		});
	}
	if p.y > 0 {
		v.push(Point {
			x: p.x,
			y: p.y - 1,
			steps: p.steps + 1,
		});
	}
	v.push(Point {
		x: p.x + 1,
		y: p.y,
		steps: p.steps + 1,
	});
	v.push(Point {
		x: p.x,
		y: p.y + 1,
		steps: p.steps + 1,
	});
	v.into_iter()
		.filter(|n| !seen.contains(n))
		.filter(|n| !is_wall(n.x, n.y))
		.collect()
}

fn print_map() {
	for y in 0..10 {
		let mut v = vec![];
		for x in 0..10 {
			if is_wall(x, y) {
				v.push(b'#');
			} else {
				v.push(b'.');
			}
		}
		println!("{}", String::from_utf8(v).unwrap());
	}
}

pub fn part1(_input: String) -> String {
	let mut q = BinaryHeap::new();
	let mut seen = HashSet::new();
	q.push(Point {x: 1, y: 1, steps: 0});
	// print_map();
	let mut count = 0;
	loop {
		let p = q.pop().unwrap();
		seen.insert(p.clone());
		if p == GOAL {
			return format!("{}", p.steps);
		}
		for n in next(p, &mut seen) {
			q.push(n);
		}
	}
}


pub fn part2(input: String) -> String {
	let mut q = BinaryHeap::new();
	let mut seen = HashSet::new();
	q.push(Point {x: 1, y: 1, steps: 0});
	loop {
		let p = match q.pop() {
			Some(n) => n,
			None => break,
		};
		seen.insert(p.clone());
		if p == GOAL {
			return format!("{}", p.steps);
		}
		for n in next(p, &mut seen) {
			if n.steps <= 50 {
				q.push(n);
			}
		}
	}
	format!("{}", seen.len())
}
