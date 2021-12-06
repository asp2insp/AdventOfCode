use crate::utils::*;

pub fn part1(input: String) -> String {
	let mut fish = input.split(",").map(|s| parse!(s, usize)).collect::<Vec<_>>();
	for _ in 0..80 {
		let mut next_fish = Vec::with_capacity(fish.len());
		for mut f in fish {
			if f == 0 {
				next_fish.push(6);
				next_fish.push(8);
			} else {
				next_fish.push(f-1);
			}
		}
		fish = next_fish;
	}
	fish.len().to_string()
}


pub fn part2(input: String) -> String {
	let mut fish = input.split(",").map(|s| parse!(s, usize)).counting_set();
	let mut pending = vec![0, 0];
	for i in 0..256 {
		let trigger = i % 7;
		pending.push(fish.get(&trigger).cloned().unwrap_or(0));
		*fish.entry(trigger).or_insert(0) += pending.remove(0);
	}
	(fish.values().sum::<usize>() + pending.into_iter().sum::<usize>()).to_string()
}

#[test]
fn example_one() {
	let state = "3,4,3,1,2".to_string();
	assert_eq!("5934", part1(state));
}

#[test]
fn example_two() {
	let state = "3,4,3,1,2".to_string();
	assert_eq!("26984457539", part2(state));
}