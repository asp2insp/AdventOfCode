use aoc::parse;
use regex::Regex;

pub fn part1(input: String) -> String {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(&input)
        .map(|c| parse!(c[1], usize) * parse!(c[2], usize))
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut acc = 0;
    let mut enabled = true;
    for c in re.captures_iter(&input) {
        if c[0].starts_with("mul") && enabled {
            acc += parse!(c[1], usize) * parse!(c[2], usize);
        } else if c[0].starts_with("don't") {
            enabled = false;
        } else if c[0].starts_with("do") {
            enabled = true;
        }
    }
    acc.to_string()
}
