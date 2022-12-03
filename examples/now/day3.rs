use std::collections::HashSet;

use itertools::Itertools;

fn priority(c: char) -> isize {
    if c >= 'a' && c <= 'z' {
        c as isize - 'a' as isize + 1
    } else {
        c as isize - 'A' as isize + 27
    }
}

pub fn part1(input: String) -> String {
    input
        .lines()
        .map(|l| {
            let left = l.chars().take(l.len() / 2).collect::<HashSet<char>>();
            let right = l.chars().skip(l.len() / 2).collect::<HashSet<char>>();
            priority(*left.intersection(&right).next().unwrap())
        })
        .sum::<isize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|c| {
            c.map(|l| l.chars().collect::<HashSet<char>>())
                .fold(None, |o: Option<HashSet<char>>, n| match o {
                    Some(m) => Some(m.intersection(&n).cloned().collect::<HashSet<char>>()),
                    None => Some(n),
                })
                .and_then(|m| m.into_iter().next())
                .map(priority)
				.unwrap()
        })
        .sum::<isize>()
        .to_string()
}
