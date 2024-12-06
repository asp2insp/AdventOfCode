use aoc::utils::*;
use itertools::Itertools;
use std::collections::HashMap;

pub fn part1(input: String) -> String {
    let g = Grid::new(&input, ());
    let mut sum = 0;
    let mut num_parts = vec![];
    for (xy, c, _) in g.iter_range_rows(None, None) {
        if !num_parts.is_empty() && (!c.is_ascii_digit() || xy.x == 0) {
            if num_parts.iter().any(|(pt, _)| {
                g.neighbors_with_diagonals(*pt)
                    .any(|p| !g.read_pt(&p).is_ascii_digit() && g.read_pt(&p) != '.')
            }) {
                // println!("Found numparts with adj sym: {:?}", num_parts);
                sum += num_parts.iter().fold(0, |acc, x| acc * 10 + x.1);
            }
            num_parts.clear();
        }
        if c.is_ascii_digit() {
            num_parts.push((xy, c.to_digit(10).unwrap()));
        }
    }
    sum.to_string()
}

pub fn part2(input: String) -> String {
    let g = Grid::new(&input, ());
    let mut num_parts: Vec<(Point, u32)> = vec![];
    let mut gears: HashMap<Point, Vec<u32>> = HashMap::new();
    for (xy, c, _) in g.iter_range_rows(None, None) {
        if !num_parts.is_empty() && (!c.is_ascii_digit() || xy.x == 0) {
            let n = num_parts.iter().fold(0, |acc, x| acc * 10 + x.1);
            num_parts
                .iter()
                .flat_map(|(pt, _)| g.neighbors_with_diagonals(*pt))
                .filter(|p| g.read_pt(&p) == '*')
                .unique()
                .for_each(|gp| {
                    gears.entry(gp).or_insert(vec![]).push(n);
                });
            num_parts.clear();
        }
        if c.is_ascii_digit() {
            num_parts.push((xy, c.to_digit(10).unwrap()));
        }
    }
    // println!("{:?}", gears);
    gears
        .into_iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .sum::<u32>()
        .to_string()
}

const TEST: &str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
#[test]
fn test_basic() {
    assert_eq!(4361.to_string(), part1(TEST.to_owned()));
    assert_eq!(467835.to_string(), part2(TEST.to_owned()));
}
