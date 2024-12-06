use aoc::utils::{gimme_usizes_once, munch};
use itertools::{repeat_n, Itertools};
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
        let group_counts = self
            .data
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
            data: repeat_n(self.data.iter().join(""), 5)
                .join("?")
                .chars()
                .collect_vec(),
            groups: repeat_n(self.groups.clone(), 5).flatten().collect_vec(),
        }
    }

    fn rc_count(self) -> usize {
        let mut memo = HashMap::new();
        rec_count_interpretations(&self.data, self.groups.clone(), &mut memo)
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
    s.lines().map(parse_row).collect_vec()
}

fn count_interpretations(row: &Row) -> usize {
    let unknowns = row.count_unknowns();
    let max = 1 << (unknowns + 1) - 1;
    let mut count = 0;
    for i in 0..max {
        let test_row = row.apply_mask(i);
        if test_row.is_valid() {
            count += 1;
        }
    }
    count
}

fn rec_count_interpretations<'a>(
    chars: &'a [char],
    groups: Vec<usize>,
    memo: &mut HashMap<(&'a [char], Vec<usize>), usize>,
) -> usize {
    // println!("{:?} {:?}", chars, groups);
    if let Some(n) = memo.get(&(chars, groups.clone())) {
        return *n;
    }
    if chars.len() == 0 {
        if groups.len() == 0 {
            return 1;
        } else {
            return 0;
        }
    }
    let mut val = 0;
    if chars[0] == '.' {
        // Use the . with the same groups
        val = rec_count_interpretations(&chars[1..], groups.clone(), memo);
    } else if chars[0] == '#' {
        if groups.len() == 0 {
            // If we don't have any groups left, then this is invalid
            return 0;
        } else {
            let mut groups_new = groups.clone();
            let g = groups_new.remove(0);
            if let Some((_, cnew)) = munch(chars, g, &['#', '?'], &[]) {
                if cnew.len() == 0 {
                    if groups_new.len() == 0 {
                        val += 1;
                    }
                } else if let Some((_, cnew)) = munch(cnew, 1, &['.', '?'], &[]) {
                    val += rec_count_interpretations(cnew, groups_new, memo);
                }
            }
        }
    } else {
        // chars[0] == '?' Either it's a . or a #
        // First count the ways it can be a .
        val = rec_count_interpretations(&chars[1..], groups.clone(), memo);
        // Then add the ways to count it as a # if possible
        if groups.len() > 0 {
            let mut groups_new = groups.clone();
            let g = groups_new.remove(0);
            if let Some((_, cnew)) = munch(chars, g, &['#', '?'], &[]) {
                if cnew.len() == 0 {
                    if groups_new.len() == 0 {
                        val += 1;
                    }
                } else if let Some((_, cnew)) = munch(cnew, 1, &['.', '?'], &[]) {
                    val += rec_count_interpretations(cnew, groups_new, memo);
                }
            }
        }
    };
    memo.insert((chars, groups), val);
    val
}

pub fn part1(input: String) -> String {
    let rows = parse(&input);
    let total = rows.into_iter().map(Row::rc_count).sum::<usize>();
    total.to_string()
}

pub fn part2(input: String) -> String {
    let rows = parse(&input);
    let total = rows
        .iter()
        .map(Row::unfold)
        .map(Row::rc_count)
        .sum::<usize>();
    total.to_string()
}

#[test]
fn test_part_1() {
    assert_eq!(count_interpretations(&parse_row("???.### 1,1,3")), 1);
    assert_eq!(count_interpretations(&parse_row(".??..??...?##. 1,1,3")), 4);
    assert_eq!(count_interpretations(&parse_row("?###???????? 3,2,1")), 10);
}

#[test]
fn test_part_2() {
    assert_eq!(parse_row("???.### 1,1,3").rc_count(), 1);
    assert_eq!(parse_row(".??..??...?##. 1,1,3").rc_count(), 4);
    assert_eq!(parse_row("?###???????? 3,2,1").rc_count(), 10);
    assert_eq!(parse_row(".??..??...?##. 1,1,3").unfold().rc_count(), 16384);
    assert_eq!(
        parse_row("????.######..#####. 1,6,5").unfold().rc_count(),
        2500
    );
}
