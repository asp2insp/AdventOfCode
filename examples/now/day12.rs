use aoc::utils::gimme_usizes_once;
use itertools::{repeat_n,Itertools};
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Row {
    data: Vec<char>,
    groups: Vec<usize>,
}

impl Row {
    fn is_valid(&self) -> bool {
        if self.data.iter().any(|c| *c == '?') {
            return false;
        }
        let group_counts = self.data
            .iter()
            .copied()
            .dedup_with_count()
            .filter(|(_c, n)| *n == '#')
            .map(|(c, _)| c)
			.collect_vec();
		self.groups == group_counts
    }

	fn count_unknowns(&self) -> usize {
		self.data.iter().filter(|&c| *c == '?').count()
	}

	fn apply_mask(&self, mut mask: usize) -> Self {
		let mut new_data = vec![];
		for m in self.data.iter() {
			if *m != '?' {
				new_data.push(*m);
			} else {
				let is_set = mask & 1 == 1;
				new_data.push(if is_set { '#' } else { '.' });
				mask >>= 1
			}
		}
		Self {
			data: new_data,
			groups: self.groups.clone(),
		}
	}

	fn unfold(&self) -> Row {
		Row {
			data: repeat_n(self.data.iter().join(""), 5).join("?").chars().collect_vec(),
			groups: repeat_n(self.groups.clone(), 5).flatten().collect_vec(),
		}
	}
}

fn parse_row(l: &str) -> Row {
	let mut parts = l.split(' ');
	Row {
		data: parts.next().unwrap().chars().collect_vec(),
		groups: gimme_usizes_once(parts.next().unwrap()),
	}
}

fn parse(s: &str) -> Vec<Row> {
    s.lines()
        .map(parse_row)
        .collect_vec()
}

fn count_interpretations(row: &Row) -> usize {
	let unknowns = row.count_unknowns();
	let max = 1 << (unknowns + 1) - 1;
	let mut count = 0;
	for i in 0..max {
		let test_row = row.apply_mask(i);
		if test_row.is_valid() {
			// println!("{:?} is valid", test_row);
			count += 1;
		}
	}
	count
}

fn rec_count_interpretations(chars: Vec<char>, groups: Vec<usize>, memo: &mut HashMap<(Vec<char>, Vec<usize>), usize>) -> usize {
	if let Some(n) = memo.get((chars.clone(), groups.clone())) {
		return *n;
	}
	if chars.len() == 0 {
		return 0;
	}

}

pub fn part1(input: String) -> String {
    let rows = parse(&input);
    let total = rows.par_iter().map(count_interpretations).sum::<usize>();
	total.to_string()
}

pub fn part2(input: String) -> String {
    let rows = parse(&input);
    let total = rows.par_iter().map(Row::unfold).map(|r| count_interpretations(&r)).sum::<usize>();
	total.to_string()
}

#[test]
fn test_part_1() {
	assert_eq!(count_interpretations(&parse_row("???.### 1,1,3")), 1);
	assert_eq!(count_interpretations(&parse_row(".??..??...?##. 1,1,3")), 4);
	assert_eq!(count_interpretations(&parse_row("?###???????? 3,2,1")), 10);
}
