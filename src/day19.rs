use crate::utils::*;
use itertools::*;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
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
			.map(|pt| pt.rotate(self.rotation))
			.map(|pt| &pt + &self.translation)
    }

	fn rotate_axes(&self, rots: P3) -> Scanner {
		let mut ret = self.clone();
		ret.rotation = &ret.rotation + &rots;
		ret
	}

	fn all_rotations(&self) -> Vec<Scanner> {
		let mut all = vec![];
		for x_rot in 0..4 {
			for y_rot in 0..4 {
				for z_rot in 0..4 {
					all.push(self.rotate_axes(P3::new(x_rot, y_rot, z_rot)));
					all.push()
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

pub fn part1(input: String) -> String {
    let scanners = parse(&input);
    let dists = scanners.iter().map(Scanner::dists_set).collect_vec();
    let possible_pairs = (0..scanners.len())
        .cartesian_product(0..scanners.len())
        .filter(|(x, y)| x != y)
        .filter_map(|(x, y)| {
            let overlap = dists[x].intersection(&dists[y]).count();
            if overlap > 66 {
                Some((x, y))
            } else {
                None
            }
        })
        .collect_vec();

    for pair in possible_pairs {
        if let Some((t, r)) = find_translation_rotation_match(&scanners[pair.0], &scanners[pair.1], 12) {
			println!("Confirmed {} and {} match on {:?}, {:?}", pair.0, pair.1, t, r);
		}
    }
    "".to_string()
}

fn find_translation_rotation_match(scanner_a: &Scanner, sc_b: &Scanner, target: usize) -> Option<(P3, P3)> {
	let points_a = scanner_a.points().collect::<HashSet<_>>();
	// For each point in A
	for a in scanner_a.points() {
		// For all orientations of B
		for b_prime in sc_b.all_rotations() {
			// For each point in B'
			for b in b_prime.points() {
				// Try to use this as the orientation point
				// Use coordinate system of A
				let scanner_b = sc_b.translate_to(a-b);
				#[cfg(test)] println!("{:?}", scanner_b);
				if scanner_b.points().filter(|pt| points_a.contains(pt)).count() >= target {
					return Some((scanner_b.translation, scanner_b.rotation))
				}
			}
		}
	}
	None
}

pub fn part2(input: String) -> String {
    "part2".to_string()
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
	assert_eq!(Some((P3::new(17, 6, 0), P3::new(0, 0, 0))), find_translation_rotation_match(&scanners[0], &scanners[1], 2));
	assert_eq!(Some((P3::new(17, 6, 0), P3::new(0, 0, 3))), find_translation_rotation_match(&scanners[0], &scanners[1].rotate_axes(P3::new(0, 0, 1)), 2));

}