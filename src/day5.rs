use regex::*;
use std::collections::HashMap;
use crate::utils::IterUtils;
use itertools::Itertools;

#[derive(Debug)]
struct Line {
	x1: isize,
	y1: isize,
	x2: isize,
	y2: isize,
}

impl Line {
	fn new(mut x1: isize, mut y1: isize, mut x2: isize, mut y2: isize) -> Line {
		if x1 > x2 {
			std::mem::swap(&mut x1, &mut x2);
			std::mem::swap(&mut y1, &mut y2);
		}
		Line {x1, y1, x2, y2}
	}

	fn is_straight(&self) -> bool {
		// println!("{:?}", self);
		self.x1 == self.x2 || self.y1 == self.y2
	}

	fn iter_y(&self) -> Box<dyn Iterator<Item=isize>> {
		if self.y1 > self.y2 {
			Box::new((self.y2..=self.y1).rev())
		} else {
			Box::new((self.y1..=self.y2))
		}
	}

	fn points_on_line(&self) -> Vec<(isize, isize)> {
		if !self.is_straight() {
			(self.x1..=self.x2).zip_eq(self.iter_y()).collect()
		} else if self.x1 == self.x2 {
			if self.y1 > self.y2 {
				(self.y2..=self.y1).map(|y| (self.x1, y)).collect()
			} else {
				(self.y1..=self.y2).map(|y| (self.x1, y)).collect()
			}
		} else {
			(self.x1..=self.x2).map(|x| (x, self.y1)).collect()
		}
	}
}

fn parse_lines(s: &str) -> Vec<Line> {
	let re = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();
	re.captures_iter(s).map(|c| Line::new(
		parse!(c["x1"], isize),
		parse!(c["y1"], isize),
		parse!(c["x2"], isize),
		parse!(c["y2"], isize),
	))
	.collect()
}


pub fn part1(input: String) -> String {
	parse_lines(&input)
		.into_iter()
		.filter(|l| l.is_straight())
		.flat_map(|l| l.points_on_line().into_iter())
		.counting_set()
		.into_iter()
		.filter(|(pt, c)| *c > 1)
		.count()
		.to_string()
}


pub fn part2(input: String) -> String {
	parse_lines(&input)
		.into_iter()
		.flat_map(|l| l.points_on_line().into_iter())
		.counting_set()
		.into_iter()
		.filter(|(pt, c)| *c > 1)
		.count()
		.to_string()
}

#[cfg(test)]
const SAMPLE: &'static str = 
r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

#[test]
fn test_with_sample() {
	assert_eq!(part1(SAMPLE.to_string()), "5");	
	assert_eq!(part2(SAMPLE.to_string()), "12");	
}