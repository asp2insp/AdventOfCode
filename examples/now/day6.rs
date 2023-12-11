// Time:        44     80     65     72
// Distance:   208   1581   1050   1102

use aoc::utils::gimme_usize_nums;
use itertools::Itertools;

pub fn part1(input: String) -> String {
    let races = gimme_usize_nums(&input);
    let times = &races[0];
    let dists = &races[1];
    let nraces = times.len();
    (0..nraces)
        .map(|i| {
            (0..times[i])
                .map(|t| t * (times[i] - t))
                .filter(|d| *d > dists[i])
                .count()
        })
        .product::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let pieces = gimme_usize_nums(&input);
    let time = pieces[0].iter().join("").parse::<usize>().unwrap();
    let dist = pieces[1].iter().join("").parse::<usize>().unwrap();
    (0..time)
        .map(|t| t * (time - t))
        .filter(|d| *d > dist)
        .count()
        .to_string()
}
