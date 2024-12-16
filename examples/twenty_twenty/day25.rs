use aoc::utils::*;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

pub fn part1(s: String) -> String {
    let (card, door) = gimme_usize_nums(&s)
        .into_iter()
        .flatten()
        .collect_tuple()
        .unwrap();
    const MOD: usize = 20201227;
    const SUBJECT_NUMBER: usize = 7;
    let mut acc = 1;
    let mut card_secret = 0;
    for i in 0.. {
        acc = (acc * SUBJECT_NUMBER) % MOD;
        if acc == card {
            card_secret = i;
            break;
        }
    }
    (0..card_secret)
        .fold(door, |acc, _| (acc * door) % MOD)
        .to_string()
}

pub fn part2(s: String) -> String {
    "2".to_string()
}

#[test]
fn test() {
    assert_eq!(part1("5764801\n17807724".to_string()), "14897079");
}
