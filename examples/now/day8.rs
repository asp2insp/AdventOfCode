use fnv::FnvHashSet;

use aoc::makeset;
use aoc::utils::*;
use itertools::Itertools;

fn coalesce<T: Clone + Eq + std::hash::Hash>(mut sets: Vec<FnvHashSet<T>>) -> Vec<FnvHashSet<T>> {
    let mut changed = true;
    while changed {
        changed = false;
        let mut output: Vec<FnvHashSet<T>> = Vec::with_capacity(sets.len());
        for s in sets {
            let mut found_combo = false;
            for o in &mut output {
                if !o.is_disjoint(&s) {
                    o.extend(s.iter().cloned());
                    found_combo = true;
                    break;
                }
            }
            if !found_combo {
                output.push(s);
            }
            changed |= found_combo;
        }
        sets = output;
    }
    sets
}

pub fn part1(input: String) -> String {
    let boxes = gimme_nums(&input)
        .into_iter()
        .map(|l| P3::new(l[0], l[1], l[2]))
        .collect_vec();
    let ordering = boxes
        .iter()
        .combinations(2)
        .sorted_by(|a, b| {
            a[0].real_dist(&a[1])
                .partial_cmp(&b[0].real_dist(&b[1]))
                .unwrap()
        })
        .collect_vec();
    let sets = ordering
        .iter()
        .take(1000)
        .map(|o| makeset! {*o[0], *o[1]})
        .collect_vec();
    coalesce(sets)
        .iter()
        .map(|s| s.len())
        .sorted()
        .rev()
        .take(3)
        .product::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let boxes = gimme_nums(&input)
        .into_iter()
        .map(|l| P3::new(l[0], l[1], l[2]))
        .collect_vec();
    let ordering = boxes
        .iter()
        .combinations(2)
        .sorted_by(|a, b| {
            a[0].real_dist(&a[1])
                .partial_cmp(&b[0].real_dist(&b[1]))
                .unwrap()
        })
        .collect_vec();
	let mut connected = boxes.iter().map(|b| makeset!{b}).collect_vec();
    for conn in ordering {
		connected.push(makeset!{conn[0], conn[1]});
		connected = coalesce(connected);
		if connected[0].len() == boxes.len() {
			return (conn[0].x * conn[1].x).to_string()
		}
	}
	"No solution found".to_string()
}
