use std::collections::VecDeque;

use itertools::Itertools;
use aoc::makeset;

fn build_decks(s: String) -> (VecDeque<usize>, VecDeque<usize>) {
    let decks = s
        .lines()
        .filter_map(|l| l.parse::<usize>().ok())
        .collect_vec();
    let p1 = decks[0..decks.len() / 2]
        .iter()
        .cloned()
        .collect::<VecDeque<usize>>();
    let p2 = decks[decks.len() / 2..]
        .iter()
        .cloned()
        .collect::<VecDeque<usize>>();
    (p1, p2)
}

pub fn part1(s: String) -> String {
    let (mut p1, mut p2) = build_decks(s);
    while p1.len() > 0 && p2.len() > 0 {
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();
        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
        // println!("p1: {:?}\np2: {:?}", p1, p2);
    }
    score(&[p1, p2]).to_string()
}

fn score(decks: &[VecDeque<usize>]) -> usize {
    decks
        .iter()
        .max_by_key(|d| d.len())
        .unwrap()
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i + 1) * c)
        .sum::<usize>()
}

fn scores_after_game(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) -> (usize, usize) {
    let mut memo = makeset!();
    while p1.len() > 0 && p2.len() > 0 {
        let key = (
            p1.iter().cloned().collect_vec(),
            p2.iter().cloned().collect_vec(),
        );
        if memo.contains(&key) {
            return (1, 0);
        }
        memo.insert(key);
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();
        let wins_p1 = if c1 > p1.len() || c2 > p2.len() {
            c1 > c2
        } else {
            let sub_p1 = p1.iter().take(c1).cloned().collect();
            let sub_p2 = p2.iter().take(c2).cloned().collect();
            let result = scores_after_game(sub_p1, sub_p2);
            result.0 > result.1
        };
        if wins_p1 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }
    (score(&[p1]), score(&[p2]))
}

pub fn part2(s: String) -> String {
    let (p1, p2) = build_decks(s);
    format!("{:?}", scores_after_game(p1, p2))
}

#[test]
fn test() {
    let s = r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"#;
    assert_eq!("306", part1(s.to_string()));
    assert_eq!("(0, 291)", part2(s.to_string()));
}
