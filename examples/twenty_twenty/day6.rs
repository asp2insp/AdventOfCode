use std::collections::HashSet;

fn groups(lines: &str) -> Vec<Vec<&str>> {
	let mut res = Vec::new();
	let mut acc = Vec::new();
	for l in lines.lines() {
		if l.is_empty() {
			res.push(acc);
			acc = Vec::new();
		} else {
			acc.push(l);
		}
	}
	res.push(acc);
	res
}

pub fn part1(input: String) -> String {
	groups(&input)
		.into_iter()
		.map(|g| {
			g.into_iter()
				.flat_map(|s| s.trim().chars())
				.collect::<HashSet<_>>()
				.len()
		})
		.sum::<usize>()
		.to_string()
}

pub fn part2(input: String) -> String {
	groups(&input)
		.into_iter()
		.map(|g| {
			g.into_iter()
				.map(|s| s.chars().collect::<HashSet<_>>())
				// .inspect(|n| println!("{:?}", n))
				.fold(None, |acc: Option<HashSet<_>>, n| {
					if let Some(ac) = acc {
						Some(ac.intersection(&n).cloned().collect())
					} else {
						Some(n)
					}
				})
				.unwrap()
				.len()
		})
		.sum::<usize>()
		.to_string()
}


const TEST_SMALL: &str = r#"t
t
tx
t

fd
of
wejznfa
f
fs
"#;

#[test]
fn test_small() {
	assert_eq!("2", part2(TEST_SMALL.to_owned()));
}