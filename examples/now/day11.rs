use aoc::utils::gimme_usizes_once;

fn round(v: Vec<usize>) -> Vec<usize> {
	v.into_iter()
		.flat_map(|i| match i {
			0 => vec![1],
			x if len(x) == 0 => {
				let s = x.to_string();
				let l = s.len();
				let sc1 = s.chars();
				let sc2 = s.chars();
				let first = sc1.take(l / 2).collect::<String>().parse::<usize>().unwrap();
				let second = sc2.skip(l/2).collect::<String>().parse::<usize>().unwrap();
				vec![first, second]
			},
			x => vec![x * 2024],
		})
		.collect()
}

pub fn part1(input: String) -> String {
	let mut nums = gimme_usizes_once(&input);
	for _ in 0..25 {
		nums = round(nums);
	}
	nums.len().to_string()
}


// const BOUNDARY: usize = 4940711462450593;

fn len(mut u: usize) -> usize {
	let mut l = 0;
	while u > 0 {
		u /= 10;
		l += 1;
	}
	l
}

pub fn part2(input: String) -> String {
	let nums = gimme_usizes_once(&input);
	let mut size = 0;
	for num in nums {
		let mut n = vec![num];
		for _ in 0..25 {
			n = round(n);
		}
		size += n.len();
	}
	size.to_string()
}
