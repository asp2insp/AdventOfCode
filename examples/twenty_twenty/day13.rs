
fn parse(s: &str) -> Result<(isize, Vec<Option<isize>>), ()> {
	let mut l = s.lines();
	let start = l.next().ok_or(())?.parse::<isize>().map_err(|_| ())?;
	let bus = l
		.next()
		.ok_or(())?
		.split(',')
		.map(|n| n.parse::<isize>().ok())
		.collect();
	Ok((start, bus))
}

pub fn part1(input: String) -> String {
	let (start, buslines) = parse(&input).unwrap();
	let mut t = start;
	loop {
		let m = buslines.iter().filter_map(|b| *b).find(|b| t % b == 0);
		if let Some(i) = m {
			return ((t - start) * i).to_string();
		}
		t += 1;
	}
}

fn run2(bs: &[(isize, isize)]) -> isize {
	let mut base = 1;
	let mut increment = 1;
	for &(b, r) in bs {
		for i in 1.. {
			if (base + increment * i + r) % b == 0 {
				base += increment * i;
				increment *= b;
				break;
			}
		}
	}
	base
}

pub fn part2(input: String) -> String {
	let (start, bus) = parse(&input).unwrap();
	let n = bus.len();
	let bs = bus
		.iter()
		.enumerate()
		.filter(|(_, b)| b.is_some())
		.map(|(i, b)| (b.unwrap(), i as isize % b.unwrap()))
		.collect::<Vec<_>>();
	run2(&bs).to_string()
}

#[test]
fn test() {
	let bs = [(17, 0), (13, 2), (19, 3)];
	assert_eq!(3417, run2(&bs));
	let bs = [(67, 0), (7, 1), (59, 2), (61, 3)];
	assert_eq!(754018, run2(&bs));
}