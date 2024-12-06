use std::collections::HashMap;

use itertools::Itertools;

fn hash(s: &str) -> usize {
    s.chars()
        .map(|c| c as usize)
        .fold(0, |acc, x| ((acc + x) * 17) % 256)
}

pub fn part1(input: String) -> String {
    input.split(",").map(hash).sum::<usize>().to_string()
}

pub fn part2(input: String) -> String {
    let mut map: Vec<Vec<(String, usize)>> = vec![Vec::new(); 256];
    for s in input.split(",") {
        let parts = s.split(&['=', '-']).collect_vec();
        let h = hash(parts[0]);
        let bucket = &mut map[h];
        if parts[1].is_empty() {
            // This is a -, try to remove the matching lens
            bucket.retain(|(s, _)| s != parts[0]);
        } else if let Some((i, _)) = bucket.iter().find_position(|(s, _)| s == &parts[0]) {
            // This is a =, try to update the matching lens
            bucket[i] = (
                parts[0].to_string(),
                parts[1]
                    .parse()
                    .expect(&format!("Error parsing {:?}", parts)),
            );
        } else {
            // This is a new lens
            bucket.push((
                parts[0].to_string(),
                parts[1]
                    .parse()
                    .expect(&format!("Error parsing {:?}", parts)),
            ));
        }
    }
    map.into_iter()
        .enumerate()
        .map(|(i, bucket)| {
            bucket
                .into_iter()
                .enumerate()
                .map(|(b, (_, l))| (i + 1) * (b + 1) * l)
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

#[test]
fn test() {
    assert_eq!(
        part2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string()),
        "145".to_string()
    )
}
