use std::ops::RangeInclusive;

use aoc::utils::gimme_usize_nums;

pub fn part1(input: String) -> String {
    let nums = gimme_usize_nums(&input);
    nums.into_iter()
        .map(|pair| {
            let l = (pair[0]..=pair[1]);
            let r = (pair[2]..=pair[3]);
            if l.contains(r.start()) && l.contains(r.end())
                || r.contains(l.start()) && r.contains(l.end())
            {
                1
            } else {
                0
            }
        })
        .sum::<isize>()
        .to_string()
}

fn has_overlap(l: (usize, usize), r: (usize, usize)) -> bool {
	let mut s = vec![l, r];
	s.sort();
	(s[0].0..=s[0].1).contains(&s[1].0)
}

pub fn part2(input: String) -> String {
    let nums = gimme_usize_nums(&input);
    nums.into_iter()
        .map(|pair| {
            if has_overlap((pair[0], pair[1]), (pair[2], pair[3])) {
                1
            } else {
                0
            }
        })
        .sum::<isize>()
        .to_string()
}
