use std::collections::HashMap;

use aoc::parse;
use aoc::utils::*;
use itertools::Itertools;
use regex::Regex;

struct Region {
    rows: usize,
    cols: usize,
    reqs: Vec<usize>,
}

impl Region {
    fn try_fit_tiles(&self, tiles: &[Tile<char>]) -> bool {
        try_fit_tiles(
            Tile::new(self.rows, self.cols, '.'),
            self.reqs.clone(),
            tiles,
            &mut HashMap::new(),
        )
    }
}

fn try_fit_tiles(
    region: Tile<char>,
    remaining: Vec<usize>,
    tiles: &[Tile<char>],
    memo: &mut HashMap<(Tile<char>, Vec<usize>), bool>,
) -> bool {
    if remaining.iter().all(|e| *e == 0) {
        // println!("Found solution for region\n{}", region);
        return true;
    }
	// Check area remaining
	if remaining.iter().enumerate().map(|(t, n)| n*tiles[t].count('#')).sum::<usize>() > region.count('.') {
		return false
	}
    let key = (region.clone(), remaining.clone());
    if let Some(result) = memo.get(&key) {
        return *result;
    }
    for t in 0..remaining.len() {
        if remaining[t] == 0 {
            continue;
        }
        for tt in tiles[t].all_orientations() {
            for offset in region.all_offsets() {
                if region.fits_at_offset(&tt, offset) {
                    if let Some(new_region) = region.overlay_at_offset(
                        &tt,
                        offset,
                        Some(|l, r| match (l, r) {
                            ('.', '#') => Some('#'),
                            ('#', '#') => None,
                            ('#', '.') => Some('#'),
                            ('.', '.') => Some('.'),
                            _ => None,
                        }),
                    ) {
                        let mut new_remain = remaining.clone();
                        new_remain[t] -= 1;
						// println!("Recurse");
                        if try_fit_tiles(new_region.clone(), new_remain.clone(), tiles, memo) {
                            return true;
                        } else {
							memo.insert((new_region, new_remain), false);
						}
                    }
                }
            }
        }
    }
    memo.insert(key, false);
    false
}

fn parse(s: &str) -> (Vec<Tile<char>>, Vec<Region>) {
    let re = Regex::new(r"(\d+)x(\d+):([\d ]+)").unwrap();
    let mut presents = vec![];
    let mut curr = vec![];
    for l in s.lines().take_while(|ls| !ls.contains('x')) {
        if l.is_empty() {
            presents.push(Tile { contents: curr });
            curr = vec![];
        } else if l.len() <= 2 {
            continue;
        } else {
            curr.push(l.to_char_array());
        }
    }

    let regions = s
        .lines()
        .filter(|ls| ls.contains('x'))
        .map(|l| {
            let caps = re.captures(l).unwrap();
            Region {
                rows: parse!(caps[2], usize),
                cols: parse!(caps[1], usize),
                reqs: caps[3]
                    .split_whitespace()
                    .map(|d| parse!(d, usize))
                    .collect_vec(),
            }
        })
        .collect_vec();
    (presents, regions)
}

pub fn part1(input: String) -> String {
    let (presents, regions) = parse(&input);
    // println!("{:?}", presents);
    regions
        .into_iter()
        .filter(|r| r.try_fit_tiles(&presents))
        .count()
        .to_string()
}

pub fn part2(input: String) -> String {
    "part2".to_string()
}
