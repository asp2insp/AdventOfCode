use std::collections::HashSet;

use aoc::utils::gimme_nums;
fn score_card(card: Vec<isize>) -> usize {
    let winning: HashSet<isize> = card[1..=10].iter().cloned().collect(); // First is card index
    card[11..]
        .into_iter()
        .filter(|&c| winning.contains(&c))
        .count()
}

pub fn part1(input: String) -> String {
    gimme_nums(&input)
        .into_iter()
        .map(score_card)
        .map(|c| if c == 0 { 0 } else { 2usize.pow(c as u32 - 1) })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let mut horizon = vec![1];
    let cards = gimme_nums(&input);
    let mut count = 0;
    for card in cards {
        let mult = if horizon.len() > 0 {
            horizon.remove(0)
        } else {
            1
        };
        count += mult;
        let n = score_card(card);
        for h in 0..n {
            if h >= horizon.len() {
                horizon.push(mult + 1);
            } else {
                horizon[h] += mult;
            }
        }
    }
    count.to_string()
}
