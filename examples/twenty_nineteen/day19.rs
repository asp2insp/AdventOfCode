use crate::intcode::*;
use aoc::utils::*;
use itertools::Itertools;

fn is_pull(x: isize, y: isize, prog: &Vec<isize>) -> isize {
    let mut c = Computer::new(prog.clone());
    c.input(x);
    c.input(y);
    c.run_and_return_output().unwrap()
}

pub fn part1(s: String) -> String {
    let prog = s
        .split(",")
        .map(|n| n.parse::<isize>().unwrap())
        .collect_vec();
    (0..50)
        .cartesian_product(0..50)
        .map(|(x, y)| is_pull(x, y, &prog))
        .sum::<isize>()
        .to_string()
}

pub fn part2(s: String) -> String {
    let prog = s
        .split(",")
        .map(|n| n.parse::<isize>().unwrap())
        .collect_vec();

    for y in 100.. {
        let x = (0..).find(|p| is_pull(*p, y, &prog) == 1).unwrap();
        let (rx, ty) = (x+99, y-99);
        if is_pull(rx, ty, &prog) == 1 {
            return ((x*10000) + ty).to_string()
        }
    }
    "No answer".to_owned()
}
