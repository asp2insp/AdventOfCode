use std::collections::HashSet;

use aoc::makeset;
use itertools::Itertools;

pub fn part1(input: String) -> String {
    (input
        .chars()
        .enumerate()
        .tuple_windows::<(_, _, _, _)>()
        .find(|tups| makeset!(tups.0 .1, tups.1 .1, tups.2 .1, tups.3 .1).len() == 4)
        .unwrap()
        .3
         .0
        + 1)
    .to_string()
}

pub fn part2(input: String) -> String {
    (input
        .as_bytes()
        .windows(14)
        .enumerate()
        .find(|tup| tup.1.iter().cloned().collect::<HashSet<u8>>().len() == 14)
        .unwrap()
         .0
        + 14)
    .to_string()
}

#[test]
fn test_2() {
	assert_eq!("19", part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_owned()))
}