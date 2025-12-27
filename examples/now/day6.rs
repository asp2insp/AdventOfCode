use crate::utils::*;
use itertools::Itertools;

pub fn part1(input: String) -> String {
    let mut nums = gimme_nums(&input);
    nums.pop(); // Remove the non-num ops line
    let ops = input
        .lines()
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .collect_vec();
    (0..nums[0].len())
        .map(|i| match ops[i] {
            "*" => nums.iter().map(|c| c[i]).product::<isize>(),
            "+" => nums.iter().map(|c| c[i]).sum::<isize>(),
            _ => unimplemented!(),
        })
        .sum::<isize>()
        .to_string()
}

pub fn part2(mut input: String) -> String {
    let g = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut sum = 0;
    let mut acc = vec![];
    for col in (0..g[0].len()).rev() {
        let mut n = 0;
        for row in 0..g.len() {
            let c = if col < g[row].len() { g[row][col] } else { ' ' };
            if c.is_digit(10) {
                n = n * 10 + c.to_digit(10).unwrap() as u64;
            } else if row == g.len() - 1 {
                if n != 0 {
                    acc.push(n);
                }
                if !c.is_ascii_whitespace() {
                    let subproblem = match c {
                        '+' => acc.iter().sum::<u64>(),
                        '*' => acc.iter().product::<u64>(),
                        _ => unimplemented!(),
                    };
                    // println!("{:?} {} = {}", acc, c, subproblem);
                    sum += subproblem;
                    acc = vec![];
                }
            }
        }
    }
    sum.to_string()
}
