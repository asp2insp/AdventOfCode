use itertools::Itertools;

const INPUT: usize = 598701;

fn bake(r1: usize, r2: usize) -> Vec<usize> {
    let mut v = vec![];
    let mut n = r1 + r2;
    while n > 0 {
        v.push(n%10);
        n = n / 10;
    }
    if v.len() == 0 {
        v.push(0);
    }
    v.into_iter().rev().collect()
}

pub fn part1(input: String) -> String {
    let mut r = vec![3, 7];
    let mut i = 0;
    let mut j = 1;
    while r.len() < INPUT + 10 {
        let newr = bake(r[i], r[j]);
        r.extend(newr);
        i = (i + r[i] + 1) % r.len();
        j = (j + r[j] + 1) % r.len();
    }
    format!("{}", &r[INPUT..INPUT+10].iter().join(""))
}

const INPUT2: [usize; 6] = [5, 9, 8, 7, 0,1];

fn rfind(needle: &[usize], haystack: &[usize]) -> usize {
    for offset in 0..needle.len()+3 {
        if offset + needle.len() >= haystack.len() {
            return 0
        }
        let base = haystack.len()-needle.len()-offset;
        if haystack[base..base+needle.len()] == *needle {
            return base
        }
    }
    0
}

pub fn part2(input: String) -> String {
    let mut r = vec![3, 7];
    let mut i = 0;
    let mut j = 1;
    while rfind(&INPUT2, &r) == 0 {
        let newr = bake(r[i], r[j]);
        r.extend(newr);
        i = (i + r[i] + 1) % r.len();
        j = (j + r[j] + 1) % r.len();
    }
    format!("{}", rfind(&INPUT2, &r))
}