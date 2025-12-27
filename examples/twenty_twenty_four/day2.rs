use aoc::utils::gimme_nums;
use aoc::utils::CloneWithoutIdx;
use itertools::Itertools;

pub fn is_safe(r: &[isize]) -> bool {
    let diffs = r.windows(2).map(|w| w[1] - w[0]).collect_vec();
    (diffs.iter().all(|&d| d > 0) || diffs.iter().all(|&d| d < 0))
        && diffs.iter().all(|d| d.abs() >= 1 && d.abs() <= 3)
}

pub fn part1(input: String) -> String {
    gimme_nums(&input)
        .into_iter()
        .filter(|r| is_safe(&r))
        .count()
        .to_string()
}

pub fn part2(input: String) -> String {
    gimme_nums(&input)
        .into_iter()
        .filter(|r| (0..r.len()).any(|i| is_safe(&r.clone_without_idx(i)[..])))
        .count()
        .to_string()
}
