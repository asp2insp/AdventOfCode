use std::collections::VecDeque;

use aoc::{
    dict, makeset,
    utils::{gimme_usize_nums, ToDebugString},
};

fn step(mut n: usize) -> usize {
    let s1 = n * 64;
    n = n ^ s1;
    n = n % 16777216;
    let s2 = n / 32;
    n = n ^ s2;
    n = n % 16777216;
    let s3 = n * 2048;
    n = n ^ s3;
    n = n % 16777216;
    n
}

pub fn part1(input: String) -> String {
    gimme_usize_nums(&input)
        .into_iter()
        .map(|v| v[0])
        .map(|mut n| {
            for _ in 0..2000 {
                n = step(n);
            }
            n
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let mut map = dict!();
    for buyer in gimme_usize_nums(&input).into_iter().map(|v| v[0]) {
        let mut n = buyer;
        let mut seq = VecDeque::new();
        let mut seen = makeset!();
        for _ in 0..2000 {
            let prev_price = n % 10;
            n = step(n);
            let new_price = n % 10;
            let diff = new_price as isize - prev_price as isize;
            seq.push_back(diff);
            if seq.len() >= 4 {
                if !seen.contains(&seq) {
                    seen.insert(seq.clone());
                    map.entry(seq.clone()).or_insert(Vec::new()).push(new_price);
                }
                seq.pop_front();
            }
        }
    }
    map.into_iter()
        .map(|(k, v)| (k, v.into_iter().sum::<usize>()))
        .max_by_key(|(k, v)| *v)
        .unwrap()
        .to_debug_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(step(123), 15887950);
        assert_eq!(step(15887950), 16495136);
    }

    #[test]
    fn test_2_ex() {
        let input = r#"1
			2
			3
			2024"#;
        assert_eq!(part2(input.to_string()), "([-2, 1, -1, 3], 23)");
    }
}
