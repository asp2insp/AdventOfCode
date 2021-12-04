use std::collections::HashSet;

#[derive(Debug)]
struct Board {
    contents: Vec<Vec<(usize, bool)>>,
}

impl Board {
	fn check_number(&mut self, num: usize) -> bool {
		let mut has_update = false;
		for r in &mut self.contents {
			for c in r.iter_mut() {
				if c.0 == num {
					c.1 = true;
					has_update = true;
				}
			}
		}
		has_update
	}

	fn is_win(&self) -> bool {
		// First check rows
		for r in &self.contents {
			if r.iter().all(|c| c.1) {
				return true
			}
		}
		for i in 0..5 {
			if self.contents.iter().all(|r| r[i].1) {
				return true
			}
		}
		false
	}

	fn score(&self) -> usize {
		self.contents.iter().flat_map(|r| r.iter()).filter(|c| !c.1).map(|c| c.0).sum()
	}

}

fn parse_input(s: &str) -> (Vec<usize>, Vec<Board>) {
    let mut ls = s.lines();
    let mut boards = vec![];
    let nums: Vec<usize> = ls
        .next()
        .unwrap()
        .split(",")
        .map(|s| parse!(s, usize))
        .collect();
    loop {
        if let Some(_s) = ls.next() {
            // blank line, we have another board
            let b = Board {
                contents: (0..5)
                    .map(|_| ls.next().unwrap())
                    .map(|l| {
                        l.split_whitespace()
                            .map(|s| (parse!(s, usize), false))
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>(),
            };
            boards.push(b);
        } else {
            break;
        }
    }
    (nums, boards)
}

pub fn part1(input: String) -> String {
    let (nums, mut boards) = parse_input(&input);
	for n in nums {
		for b in boards.iter_mut() {
			b.check_number(n);
			if b.is_win() {
				return format!("{} * {} = {}", b.score(), n, b.score() * n)
			}
		}
	}
    format!("womp")
}

pub fn part2(input: String) -> String {
    let (nums, mut boards) = parse_input(&input);
	let mut has_win: HashSet<usize> = HashSet::new();
	for n in nums {
		for bi in 0..boards.len() {
			boards[bi].check_number(n);
			if !has_win.contains(&bi) && boards[bi].is_win() {
				has_win.insert(bi);
				if has_win.len() == boards.len() {
					let score = boards[bi].score();
					return format!("{} * {} = {}", score, n, score * n)
				}
			}
		}
	}
	format!("womp")
}
