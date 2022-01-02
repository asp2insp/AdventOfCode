use crate::{utils::*, get_input};
use itertools::*;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Scanner {
    id: usize,
	translation: P3,
	rotation: P3,
    _points: Vec<P3>,
}

impl Scanner {
    fn translate_to(&self, p: P3) -> Scanner {
        Scanner {
            id: self.id,
			translation: p,
			rotation: self.rotation,
            _points: self._points.clone(),
        }
    }

    fn points<'a>(&'a self) -> impl Iterator<Item=P3> + 'a {
        self._points
			.iter()
			.map(|pt| pt.flip_and_rotate(self.rotation))
			.map(|pt| &pt + &self.translation)
    }

	fn rotate_axes(&self, rots: P3) -> Scanner {
		let mut ret = self.clone();
		ret.rotation = &ret.rotation + &rots;
		ret
	}

	fn all_rotations(&self) -> Vec<Scanner> {
		let mut all = vec![];
		for x_rot in -3..4 {
			for y_rot in -3..4 {
				for z_rot in -3..4 {
					all.push(self.rotate_axes(P3::new(x_rot, y_rot, z_rot)));
				}
			}
		}
		all
	}
}

fn parse(s: &str) -> Vec<Scanner> {
    s.split("\n\n")
        .map(|chunk| {
            let mut l = chunk.lines();
            let id = parse!(l.next().unwrap().trim().split_whitespace().nth(2).unwrap(), usize);
            let points = l.map(|s| s.trim().into()).collect_vec();
            Scanner { id, translation: P3::origin(), rotation: P3::origin(), _points: points }
        })
        .collect_vec()
}

impl Scanner {
    fn dists_set(&self) -> HashSet<isize> {
        (0..self._points.len())
            .cartesian_product(0..self._points.len())
            .filter(|(x, y)| x != y)
            .map(|(x, y)| self._points[x].euclidian_dist_squared(&self._points[y]))
            .collect()
    }
}

fn find_scanner_locations(scanners: Vec<Scanner>) -> HashMap<usize, Scanner> {
	let dists = scanners.iter().map(Scanner::dists_set).collect_vec();
    let possible_pairs = (0..scanners.len())
        .cartesian_product(0..scanners.len())
        .filter(|(x, y)| x != y)
        .filter_map(|(x, y)| {
            let overlap = dists[x].intersection(&dists[y]).count();
            if overlap >= 66 {
                Some((x, y))
            } else {
                None
            }
        })
        .collect_vec();
	#[cfg(test)] println!("{:?}", possible_pairs);

	let mut results = HashMap::new();
	results.insert(0, scanners[0].clone());
	let mut q = vec![0];
	while let Some(n) = q.pop() {
		for to in possible_pairs.iter().filter(|(from, _)| *from == n).map(|(_, to)| *to) {
			if results.contains_key(&to) {
				continue;
			}
			if let Some(scb) = find_translation_rotation_match(&results[&n], &scanners[to], 12) {
				#[cfg(test)] println!("{} => {} at {:?}", n, to, scb.translation);
				q.push(to);
				results.insert(to, scb);
			}
		}
	}
	results
}

pub fn part1(input: String) -> String {
    let mut scanners = parse(&input);
    let results = find_scanner_locations(scanners);
    results.values().flat_map(Scanner::points).sorted().unique().count().to_string()
}

fn find_translation_rotation_match(scanner_a: &Scanner, sc_b: &Scanner, target: usize) -> Option<Scanner> {
	let points_a = scanner_a.points().collect::<HashSet<_>>();
	// For each point in A
	scanner_a.points().collect_vec().par_iter().filter_map(|a| {
		// For all orientations of B
		for b_prime in sc_b.all_rotations() {
			// For each point in B'
			for b in b_prime.points() {
				// Try to use this as the orientation point
				// Use coordinate system of A
				let scanner_b = b_prime.translate_to(a-&b);
				// #[cfg(test)] println!("{:?}", scanner_b);
				if scanner_b.points().filter(|pt| points_a.contains(pt)).count() >= target {
					return Some(scanner_b)
				}
			}
		}
		None
	})
	.find_first(|_| true)
}

pub fn part2(input: String) -> String {
    let mut scanners = parse(&input);
    let results = find_scanner_locations(scanners);
    let points = results.values().map(|v| v.translation).collect_vec();
	points.iter().cartesian_product(points.iter())
		.map(|(a,b)| a.dist(b))
		.max()
		.unwrap()
		.to_string()
}



#[test]
fn test_scanner_ops() {
	let sc = Scanner {
		id: 0,
		translation: P3::origin(),
		rotation: P3::origin(),
		_points: vec![P3::new(-5, -10, 2)],
	};
	assert_eq!(Some(P3::new(10, 20, 2)), sc.translate_to(P3::new(15, 30, 0)).points().next());

	assert_eq!(Some(P3::new(5, 10, 2)), sc.rotate_axes(P3::new(0, 0, 2)).points().next());
}

#[test]
fn test_simple() {
	let s = "- scanner 0 -\n7,-4,0\n7,-3,0\n\n- scanner 1 -\n-10,-10,0\n-10,-9,0";
	let scanners = parse(s);
	assert_eq!(Some(scanners[1].translate_to(P3::new(17, 6, 0)).points().collect_vec()), find_translation_rotation_match(&scanners[0], &scanners[1], 2).map(|sc| sc.points().collect_vec()));
}

#[test]
fn test_orientations() {
	let s = r"--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7

--- scanner 0 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0

--- scanner 0 ---
-1,-1,-1
-2,-2,-2
-3,-3,-3
-1,-3,-2
4,6,5
-7,0,8

--- scanner 0 ---
1,1,-1
2,2,-2
3,3,-3
1,3,-2
-4,-6,5
7,0,8

--- scanner 0 ---
1,1,1
2,2,2
3,3,3
3,1,2
-6,-4,-5
0,7,-8";
	let scs = parse(s);
	let rots = scs[0].all_rotations();
	// println!("{:?}", rots);
	for or in scs {
		// println!("{:?}", or.points().collect_vec());
		assert_eq!(true, rots.iter().any(|sc| sc.points().collect_vec() == or.points().collect_vec()));
	}
}

#[test]
fn test_example_case() {
	let s = get_input("day19_test");
	assert_eq!("79", part1(s));
}