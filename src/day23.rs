use crate::utils::*;

// #############
// #...........#
// ###A#D#A#C###
//   #C#D#B#B#
//   #########

const EMPTY: char = '\0';
const A: char = 'A';
const B: char = 'B';
const C: char = 'C';
const D: char = 'D';

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
	left_hall: [char; 2],
	a_hall: [char; 2],
	ab_slot: char,
	b_hall: [char; 2],
	bc_slot: char,
	c_hall: [char; 2],
	cd_slot: char,
	d_hall: [char; 2],
	right_hall: [char; 2],
}

fn parse(s: &str) -> State {
	let g = Grid::new(s, ());
	State {
		left_hall: [EMPTY, EMPTY],
		a_hall: [g.read(3, 1), g.read(3, 2)],
		ab_slot: EMPTY,
		b_hall: [g.read(5, 1), g.read(5, 2)],
		bc_slot: EMPTY,
		c_hall: [g.read(7, 1), g.read(7, 2)],
		cd_slot: EMPTY,
		d_hall: [g.read(9, 1), g.read(7, 2)],
		right_hall: [EMPTY, EMPTY],
	}
}

pub fn part1(input: String) -> String {
	let start = parse(&input);
	"part1".to_string()
}


pub fn part2(input: String) -> String {
	"part2".to_string()
}
