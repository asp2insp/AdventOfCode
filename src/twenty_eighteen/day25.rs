use crate::utils::*;
use std::collections::HashSet;
use itertools::*;


fn dist(a: &[isize], b:&[isize]) -> isize {
    a.iter().zip(b.iter()).map(|(c1,c2)| c1.max(c2) - c1.min(c2)).sum()
}

pub fn part1(s: String) -> String {
    let pts = gimme_nums(&s);
    let mut cts: Vec<Vec<Vec<isize>>> = vec![];
    for pt in pts.into_iter() {
        let mut joins = vec![];
        for (i, ct) in cts.iter_mut().enumerate() {
            if ct.iter().any(|p| dist(p, &pt) <= 3) {
                ct.push(pt.clone());
                joins.push(i);
            }
        }
        if joins.len() > 1 {
            let mut all = HashSet::new();
            joins.into_iter().for_each(|i| cts[i].drain(..).for_each(|p| {
                all.insert(p);
            }));
            cts.push(all.into_iter().collect_vec());
        } else if joins.is_empty() {
            cts.push(vec![pt]);
        }
    }
    cts.iter().filter(|ct| !ct.is_empty()).count().to_string()
}

pub fn part2(s: String) -> String {
    "".into()
}