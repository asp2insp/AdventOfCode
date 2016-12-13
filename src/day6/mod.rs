use itertools::Itertools;

pub fn part1(input: String) -> String {
	let mut initial = vec![vec![]; input.lines().next().unwrap().len()];
	input.lines()
		.fold(initial, |mut v, line| {
			for (i, c) in line.chars().enumerate() {
				v[i].push(c);
			}
			v
		})
		.into_iter()
		.map(|mut col| {
			col.sort();
			col.into_iter().group_by(|c| *c)
		})
		.map(|col_grouped| {
			col_grouped.into_iter()
				.fold((' ', 0), |max, (c, group)| {
					let l = group.count();
					if l > max.1 {
						(c, l)
					} else {
						max
					}
				})
		})
		.map(|(c, _)| c.clone())
		.collect()
}


pub fn part2(input: String) -> String {
	let mut initial = vec![vec![]; input.lines().next().unwrap().len()];
	input.lines()
		.fold(initial, |mut v, line| {
			for (i, c) in line.chars().enumerate() {
				v[i].push(c);
			}
			v
		})
		.into_iter()
		.map(|mut col| {
			col.sort();
			col.into_iter().group_by(|c| *c)
		})
		.map(|col_grouped| {
			col_grouped.into_iter()
				.fold((' ', 1000), |min, (c, group)| {
					let l = group.count();
					if l < min.1 {
						(c, l)
					} else {
						min
					}
				})
		})
		.map(|(c, _)| c.clone())
		.collect()
}
