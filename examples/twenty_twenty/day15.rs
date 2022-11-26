use std::collections::HashMap;

fn run_n_times(nums: &[usize], times: usize) -> usize {
	let mut t = 1;
	let mut map: HashMap<usize, usize> = HashMap::new();
	let mut last = 0usize;
	for n in nums {
		map.insert(*n, t);
		t += 1;
	}
	loop {
		let next = if map.contains_key(&last) {
			t - map.get(&last).unwrap()
		} else {
			0
		};
		map.insert(last, t);
		if t == times {
			break;
		}
		t += 1;
		last = next;
	}
	last
}


pub fn part1(input: String) -> String {
	run_n_times(&input.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>(), 2020).to_string()
}


pub fn part2(input: String) -> String {
	run_n_times(&input.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>(), 30000000).to_string()
}
