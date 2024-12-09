
fn checksum(map: Vec<usize>) -> usize {
	map.into_iter().enumerate().take_while(|(_, v)| *v != 0).map(|(i, v)| i * (v-1)).sum::<usize>()
}

pub fn part1(input: String) -> String {
	let mut map = vec![];
	let mut free = false;
	let mut fnum = 1; // Need to subtract 1 when calculating so we can use 0 as sentinel
	for d in input.chars().map(|c| c.to_digit(10).unwrap() as usize) {
		if free {
			map.extend(vec![0; d]);
		} else {
			map.extend(vec![fnum; d]);
			fnum += 1;
		}
		free = !free;
	}
	let mut write = 0;
	let mut read = map.len() - 1;
	while write < read {
		if map[write] != 0 {
			write += 1;
		} else if map[read] != 0 {
			map.swap(write, read);
			write += 1;
			read -= 1;
		} else {
			read -= 1;
		}
	}
	checksum(map).to_string()
}

#[derive(Debug, Default, Copy, Clone)]
struct Span {
	start: usize, // inclusive
	end: usize,   // exclusive
	inner_len: usize,
	fnum: usize,
}

impl Span {
	fn len(&self) -> usize {
		self.end - self.start
	}

	fn checksum(&self, start_idx: usize) -> usize {
		(start_idx..start_idx+self.inner_len).map(|i| i * self.fnum).sum::<usize>()
	}

	fn is_free(&self) -> bool {
		self.inner_len == 0
	}

	// Insert other into self, and return the new free span of remaining space
	fn insert(&mut self, other: &Span) -> Span {
		let breakpoint = self.start + other.inner_len;
		let s2 = Span {
			start: breakpoint,
			end: self.end,
			inner_len: 0,
			fnum: 0,
		};
		self.end = breakpoint;
		self.inner_len = other.inner_len;
		self.fnum = other.fnum;
		s2
	}
}

pub fn part2(input: String) -> String {
	let mut fnum = 0;
	let mut free = false;
	let mut spans = vec![];
	let mut idx = 0;
	for d in input.chars().map(|c| c.to_digit(10).unwrap() as usize) {
		if free {
			spans.push(Span {start: idx, end: idx + d, inner_len: 0, fnum: 0});
		} else {
			spans.push(Span {start: idx, end: idx + d, inner_len: d, fnum});
			fnum += 1;
		}
		idx += d;
		free = !free;
	}
	let mut i = spans.len() - 1;
	while i > 0 {
		if spans[i].is_free() {
			i -= 1;
			continue;
		}
		for j in 0..i {
			if spans[j].is_free() && spans[j].len() >= spans[i].len() {
				let mov = spans[i].clone();
				let s2 = spans[j].insert(&mov);
				if s2.len() > 0 {
					spans.insert(j+1, s2);
					i += 1; // account for newly inserted span
				}
				spans[i].inner_len = 0;
				spans[i].fnum = 0;
				break;
			}
		}
		i -= 1;
	}
	// println!("{:#?}", spans);
	let mut idx = 0;
	let mut acc = 0;
	for span in spans {
		acc += span.checksum(idx);
		idx += span.len();
	}
	acc.to_string()
}


#[test]
fn test() {
	assert_eq!(part1("2333133121414131402".to_string()), "1928");
	assert_eq!(part2("2333133121414131402".to_string()), "2858");
}
