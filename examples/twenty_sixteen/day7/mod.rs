fn supports_tls(s: &str) -> bool {
	let mut in_brackets = false;
	let mut history = vec![];
	let mut valid = false;
	for c in s.chars() {
		match (c, history.len()) {
			('[', _) => {
				in_brackets = true;
				history.clear();
			},
			(']', _) => {
				in_brackets = false;
				history.clear();
			},
			(_, 3) => {
				history.push(c);
				if history[..2].iter().eq(history[2..].iter().rev()) {
					if in_brackets {
						return false
					}
					valid |= !history[..2].eq(&history[2..]);
				}
				history.remove(0);
			},
			(_, _) => {
				history.push(c);
			},
		};
	}
	return valid;
}

fn supports_ssl(s: &str) -> bool {
	let mut in_brackets = false;
	let mut history = vec![];
	let mut aba: Vec<Vec<char>> = vec![];
	let mut bab: Vec<Vec<char>> = vec![];

	for c in s.chars() {
		match (c, history.len()) {
			('[', _) => {
				in_brackets = true;
				history.clear();
			},
			(']', _) => {
				in_brackets = false;
				history.clear();
			},
			(_, 2) => {
				history.push(c);
				if history[0] == history[2] && history[0] != history[1] {
					if in_brackets {
						bab.push(history.clone());
					} else {
						aba.push(history.clone());
					}
				}
				history.remove(0);
			},
			(_, _) => {
				history.push(c);
			},
		};
	}
	for a in &aba {
		for b in &bab {
			if a[1] == b[0] && a[0] == b[1] {
				return true
			}
		}
	}
	return false
}

pub fn part1(input: String) -> String {
	let count = input.lines()
		.filter(|l| supports_tls(l))
		.count();
	format!("{}", count)
}


pub fn part2(input: String) -> String {
	let count = input.lines()
		.filter(|l| supports_ssl(l))
		.count();
	format!("{}", count)
}
