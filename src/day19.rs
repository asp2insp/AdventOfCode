use crate::utils::*;
use itertools::*;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
struct Scanner {
    id: usize,
	origin: P3,
    points: Vec<P3>,
}

impl Scanner {
    fn translate_to(&self, p: P3) -> Scanner {
        Scanner {
            id: self.id,
			origin: p,
            points: self.points.iter().map(|pt| pt + &p).collect_vec(),
        }
    }

    fn rotate_90_around_axis(&self, axis: char) -> Scanner {
        Scanner {
            id: self.id,
			origin: self.origin,
            points: self
                .points
                .iter()
                .map(|pt| &(pt - &self.origin).rotate_90_around_axis(axis) + &self.origin)
                .collect_vec(),
        }
    }

	fn rotate_around_axis(&self, axis: char, times: isize) -> Scanner {
		let mut ret: Scanner = self.clone();
		for _ in 0..times {
			ret = ret.rotate_90_around_axis(axis);
		}
		ret
	}

	fn all_rotations(&self) -> Vec<Scanner> {
		let mut all = vec![];
		for x_rot in 0..4 {
			for y_rot in 0..4 {
				for z_rot in 0..4 {
					let mut ret = self.clone();
					ret = ret.rotate_around_axis('z', z_rot);
					ret = ret.rotate_around_axis('y', y_rot);
					ret = ret.rotate_around_axis('x', x_rot);
					all.push(ret);
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
            let id = parse!(l.next().unwrap().split_whitespace().nth(2).unwrap(), usize);
            let points = l.map(|s| s.into()).collect_vec();
            Scanner { id, origin: P3{x: 0, y: 0, z: 0}, points }
        })
        .collect_vec()
}

impl Scanner {
    fn dists_set(&self) -> HashSet<isize> {
        (0..self.points.len())
            .cartesian_product(0..self.points.len())
            .filter(|(x, y)| x != y)
            .map(|(x, y)| self.points[x].euclidian_dist_squared(&self.points[y]))
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
        // For each point in A
        for a in &scanners[pair.0].points {
            // For each point in B
            for b in &scanners[pair.1].points {
                // Try to use this as the orientation point
				// Use coordinate system of A
				let scanner_b = &scanners[pair.1].translate_to(a-b);
				let scanner_a = &scanners[pair.0];
				let points_a = scanner_a.points.iter().collect::<HashSet<_>>();

				// Try all the orientations of B
				if scanner_b.all_rotations().iter().any(|new_b| new_b.points.iter().filter(|pt| points_a.contains(pt)).count() >= 12) {
					println!("Confirmed {} and {} match on {:?}", pair.0, pair.1, a);
				}
            }
        }
    }
    "".to_string()
}

pub fn part2(input: String) -> String {
    "part2".to_string()
}
