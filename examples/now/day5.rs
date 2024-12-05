use aoc::utils::gimme_usize_nums;
use itertools::Itertools;
struct Rule {
    x: usize,
    y: usize,
}

impl Rule {
    fn check_seq(&self, seq: &[usize]) -> bool {
        let (x_idx, y_idx) = self.idx(&seq);
        match (x_idx, y_idx) {
            (Some(x), Some(y)) if y < x => false,
            _ => true,
        }
    }

    fn enforce(&self, seq: &mut [usize]) {
        let (x_idx, y_idx) = self.idx(&seq);
        match (x_idx, y_idx) {
            (Some(x), Some(y)) if y < x => {
                seq.swap(x, y);
            }
            _ => {}
        }
    }

    fn idx(&self, seq: &[usize]) -> (Option<usize>, Option<usize>) {
        let mut x_idx = None;
        let mut y_idx = None;
        for (i, &n) in seq.iter().enumerate() {
            if n == self.x {
                x_idx = Some(i);
            }
            if n == self.y {
                y_idx = Some(i);
            }
        }
        (x_idx, y_idx)
    }
}

fn parse(input: &str) -> (Vec<Rule>, Vec<Vec<usize>>) {
    gimme_usize_nums(&input).into_iter().fold(
        (Vec::new(), Vec::new()),
        |(mut rules, mut seqs), nums| {
            if nums.len() == 2 {
                rules.push(Rule {
                    x: nums[0],
                    y: nums[1],
                });
            } else if nums.len() > 0 {
                seqs.push(nums);
            }
            (rules, seqs)
        },
    )
}

pub fn part1(input: String) -> String {
    let (rules, seqs) = parse(&input);
    seqs.iter()
        .filter(|seq| rules.iter().all(|rule| rule.check_seq(seq)))
        .map(|s| s[s.len() / 2])
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let (rules, seqs) = parse(&input);
    let bad_seqs = seqs
        .into_iter()
        .filter(|seq| !rules.iter().all(|rule| rule.check_seq(seq)))
        .collect_vec();
    bad_seqs
        .into_iter()
        .map(|mut seq| {
            while !rules.iter().all(|rule| rule.check_seq(&seq)) {
                rules.iter().for_each(|rule| rule.enforce(&mut seq));
            }
            seq[seq.len() / 2]
        })
        .sum::<usize>()
        .to_string()
}
