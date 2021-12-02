fn subexpr(s: &[u8]) -> &[u8] {
	let mut n = 1;
	let mut i = 0;
	while n > 0 {
		match s[i] {
			b'(' => n += 1,
			b')' => n -= 1,
			_ => {}
		};
		i += 1;
	}
	&s[..i - 1]
}

fn apply(a: isize, op: u8, b: isize) -> isize {
	match op {
		b'+' => a + b,
		b'*' => a * b,
		_ => unimplemented!(),
	}
}

fn eval(s: &[u8]) -> isize {
	// println!("Eval: {}", String::from_utf8_lossy(s));
	let mut i = 0;
	let mut res = 0;
	let mut op = b'+';
	while i < s.len() {
		match s[i] {
			b'(' => {
				let sub = subexpr(&s[i + 1..]);
				i += 2 + sub.len();
				res = apply(res, op, eval(sub));
			}
			b' ' => {
				i += 1;
			}
			x if x >= b'0' && x <= b'9' => {
				res = apply(res, op, (x - b'0') as isize);
				i += 1;
			}
			x if x == b'*' || x == b'+' => {
				op = x;
				i += 1;
			}
			x => unimplemented!("{}", x),
		}
	}
	res
}

pub fn part1(input: String) -> String {
	input
		.lines()
		.map(|l| eval(&l.as_bytes()))
		.sum::<isize>()
		.to_string()
}

fn walk_while(s: &[u8], left: bool, mut i: usize) -> usize {
	let mut count = 0;
	loop {
		if s[i] >= b'0' && s[i] <= b'9' && count == 0 {
			return i + if left { 0 } else { 1 };
		}
		if s[i] == b'(' && left || s[i] == b')' && !left {
			count -= 1;
			if count == 0 {
				return i + if left { 0 } else { 1 };
			}
		} else if s[i] == b')' && left || s[i] == b'(' && !left {
			count += 1;
		}
		if i == 0 && left {
			return 0;
		} else if i == s.len() - 1 && !left {
			return s.len() - 1;
		}
		if left {
			i -= 1;
		} else {
			i += 1
		};
	}
}

fn apply_precedence(s: &[u8]) -> Vec<u8> {
	let mut res = s.to_owned();
	let mut i = 0;
	while i < res.len() {
		if res[i] == b'+' {
			let l_loc = walk_while(&res, true, i);
			let r_loc = walk_while(&res, false, i);
			res.insert(r_loc, b')');
			res.insert(l_loc, b'(');
			i += 1;
		}
		i += 1;
	}
	res
}

pub fn part2(input: String) -> String {
	input
		.lines()
		.map(|l| apply_precedence(&l.as_bytes()))
		.map(|l| eval(&l))
		.sum::<isize>()
		.to_string()
}

#[test]
fn test() {
	let first = apply_precedence("1 + 2 * 3 + 4 * 5 + 6".as_bytes());
	assert_eq!(
		"(1 + 2) * (3 + 4) * (5 + 6)",
		String::from_utf8(first.clone()).unwrap()
	);
	assert_eq!(231, eval(&first));

	let opt = apply_precedence("1 * (9 * 8) + 6".as_bytes());
	assert_eq!(
		"1 * ((9 * 8) + 6)",
		String::from_utf8(opt.clone()).unwrap()
	);
	assert_eq!(78, eval(&opt));

	let second = apply_precedence("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".as_bytes());
	assert_eq!(
		"(((((2 + 4) * 9) * (((6 + 9) * (8 + 6)) + 6)) + 2) + 4) * 2",
		String::from_utf8(second.clone()).unwrap()
	);
	assert_eq!(23340, eval(&second));
	let third = apply_precedence("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".as_bytes());
	assert_eq!(
		"5 * 9 * (7 * 3 * (3 + 9) * (3 + ((8 + 6) * 4)))",
		String::from_utf8(third.clone()).unwrap()
	);
	assert_eq!(669060, eval(&third));
}
