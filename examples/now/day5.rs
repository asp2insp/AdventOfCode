use crate::utils::*;

fn get_data(s: &str) -> (Vec<BetterRange>, Vec<usize>) {
    let v = gimme_nums(s);
    let mut ranges = vec![];
    let mut nums = vec![];
    for i in v {
		if i.is_empty() {
			continue;
		}
        if i.len() > 1 {
            ranges.push(BetterRange::new_unordered_inclusive(
                i[0] as usize,
                i[1].abs() as usize,
            ));
        } else {
            nums.push(i[0] as usize);
        }
    }
    (ranges, nums)
}

pub fn part1(input: String) -> String {
	let (ranges, nums) = get_data(&input);
	nums.iter()
		.filter(|n| ranges.iter().any(|r| r.includes(**n)))
		.count()
		.to_string()
}

pub fn part2(input: String) -> String {
	let (mut ranges, _nums) = get_data(&input);
	ranges.sort();
	let mut changed = true;
	while changed {
		changed = false;
		let mut combined = vec![];
		for r in ranges {
			if combined.is_empty() {
				combined.push(r);
			} else {
				let mut found_overlap = false;
				for c in &mut combined {
					if c.try_expand(&r) {
						found_overlap = true;
						break
					}
				}
				if !found_overlap {
					combined.push(r);
				}
				changed |= found_overlap;
			}
		}
		ranges = combined;
	}
    ranges.into_iter()
		.map(|r| r.len())
		.sum::<usize>()
		.to_string()
}
