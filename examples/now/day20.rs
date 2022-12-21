use aoc::utils::gimme_nums;
use itertools::Itertools;

pub fn part1(input: String) -> String {
    let mut mix = gimme_nums(&input)
        .into_iter()
        .flatten()
        .enumerate()
        .collect_vec();
    for j in 0..mix.len() {
        let orig_i = mix.iter().position(|&(e, _)| e == j).unwrap();
        let n = mix[orig_i];
        let mut i = orig_i as isize + n.1;
        i %= mix.len() as isize - 1;
        let i = if i < 0 { mix.len() as isize - 1 + i } else { i } as usize;
        mix.remove(orig_i);
        mix.insert(i, n);
    }
    let b = mix.iter().position(|&(_, v)| v == 0).unwrap();
    (mix[(b + 1000) % mix.len()].1 + mix[(b + 2000) % mix.len()].1 + mix[(b + 3000) % mix.len()].1)
        .to_string()
}

pub fn part2(input: String) -> String {
    let key = 811589153;
    let mut mix = gimme_nums(&input)
        .into_iter()
        .flatten()
        .map(|i| i * key)
        .enumerate()
        .collect_vec();
    for _ in 0..10 {
        for j in 0..mix.len() {
            let orig_i = mix.iter().position(|&(e, _)| e == j).unwrap();
            let n = mix[orig_i];
            let mut i = orig_i as isize + n.1;
            i %= mix.len() as isize - 1;
            let i = if i < 0 { mix.len() as isize - 1 + i } else { i } as usize;
            mix.remove(orig_i);
            mix.insert(i, n);
        }
    }
    let b = mix.iter().position(|&(_, v)| v == 0).unwrap();
    (mix[(b + 1000) % mix.len()].1 + mix[(b + 2000) % mix.len()].1 + mix[(b + 3000) % mix.len()].1)
        .to_string()
}

#[test]
fn test() {
    let s = "1\n2\n-3\n3\n-2\n0\n4".to_owned();
    assert_eq!("3", part1(s));
}
