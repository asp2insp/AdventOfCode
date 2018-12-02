use itertools::*;

pub fn part1(input: String) -> String {
    format!("{}", 
        input.lines()
            .map(|l| {
                sorted(l.chars()).into_iter().group_by(|e| *e).into_iter().fold((0,0), |acc, n| {
                    match n.1.count() {
                        3 => (acc.0, 1),
                        2 => (1, acc.1),
                        _ => acc,
                    }
                })
            })
            .fold([0, 0], |acc, n| [acc[0] + n.0, acc[1] + n.1])
        .into_iter()
        .fold(1, |prod, n| prod * n)
    )
}

pub fn part2(input: String) -> String {
    for (l, r) in input.lines().cartesian_product(input.lines()) {
        let diffs = l.chars().zip(r.chars()).map(|(c1, c2)| if c1 == c2 {0} else {1}).collect::<Vec<_>>();
        if diffs.iter().sum::<usize>() == 1 {
            return l.chars().enumerate().filter(|(i, _)| diffs[*i] == 0).map(|(_, c)| c).collect()
        }
    }
    "No Answer".to_owned()
}