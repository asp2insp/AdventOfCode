use std::cmp::Ordering;

use aoc::utils::IterUtils;
use itertools::Itertools;

fn card_to_value(c: char) -> usize {
    match c {
        'A' => 14,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'T' => 10,
        _ => c.to_digit(10).expect(&format!("{} ??", c)) as usize,
    }
}

fn get_hand_rank(hand: &str) -> usize {
    match hand
        .chars()
        .counting_set()
        .values()
        .sorted()
        .rev()
        .collect::<Vec<_>>()
        .as_slice()
    {
        [5] => 7,
        [4, 1] => 6,
        [3, 2] => 5,
        [3, 1, 1] => 4,
        [2, 2, 1] => 3,
        [2, 1, 1, 1] => 2,
        [1, 1, 1, 1, 1] => 1,
        _ => panic!("Invalid hand"),
    }
}

fn cmp_poker_hands(a: &str, b: &str) -> Ordering {
    get_hand_rank(a).cmp(&get_hand_rank(b)).then(
        a.chars()
            .map(card_to_value)
            .collect::<Vec<_>>()
            .as_slice()
            .cmp(b.chars().map(card_to_value).collect::<Vec<_>>().as_slice()),
    )
}

fn parse(s: &str) -> Vec<(&str, usize)> {
    s.lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            (
                parts.next().expect(l),
                parts.next().expect(l).parse().expect(l),
            )
        })
        .collect()
}

pub fn part1(input: String) -> String {
    let mut hands_and_bids = parse(&input);
    hands_and_bids.sort_by(|a, b| cmp_poker_hands(a.0, b.0));
    hands_and_bids
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum::<usize>()
        .to_string()
}

fn card_to_value_with_j(c: char) -> usize {
    match c {
        'A' => 14,
        'J' => 0,
        'Q' => 12,
        'K' => 13,
        'T' => 10,
        _ => c.to_digit(10).expect(&format!("{} ??", c)) as usize,
    }
}

fn get_hand_rank_with_j(hand: &str) -> usize {
    let mut set = hand.chars().counting_set();
    let joker_count = set.remove(&'J').unwrap_or(0);
    let mut values = set.into_values().sorted().rev().collect::<Vec<_>>();
    if values.is_empty() {
        values.push(0);
    }
    values[0] += joker_count;
    match values.as_slice() {
        [5] => 7,
        [4, 1] => 6,
        [3, 2] => 5,
        [3, 1, 1] => 4,
        [2, 2, 1] => 3,
        [2, 1, 1, 1] => 2,
        [1, 1, 1, 1, 1] => 1,
        _ => panic!("Invalid hand"),
    }
}

fn cmp_poker_hands_with_j(a: &str, b: &str) -> Ordering {
    get_hand_rank_with_j(a).cmp(&get_hand_rank_with_j(b)).then(
        a.chars()
            .map(card_to_value_with_j)
            .collect::<Vec<_>>()
            .as_slice()
            .cmp(
                b.chars()
                    .map(card_to_value_with_j)
                    .collect::<Vec<_>>()
                    .as_slice(),
            ),
    )
}

pub fn part2(input: String) -> String {
    let mut hands_and_bids = parse(&input);
    hands_and_bids.sort_by(|a, b| cmp_poker_hands_with_j(a.0, b.0));
    hands_and_bids
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum::<usize>()
        .to_string()
}
