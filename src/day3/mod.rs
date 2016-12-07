use chomp::prelude::{U8Input, SimpleResult, parse_only, many1};
use chomp::ascii::{decimal, skip_whitespace};
use itertools::Itertools;

type Triangle = (usize, usize, usize);

fn triangle<I: U8Input>(i: I) -> SimpleResult<I, Triangle> {
    parse!{i;
        let a = decimal() <* skip_whitespace();
		let b = decimal() <* skip_whitespace();
		let c = decimal() <* skip_whitespace();
		ret (a, b, c)
	}
}

fn all_tris<I: U8Input>(i: I) -> SimpleResult<I, Vec<Triangle>> {
    parse!{i;
		let v = many1(triangle);
		ret v
	}
}

pub fn part1(input: String) -> String {
	let tris = parse_only(all_tris, input.as_bytes()).unwrap();
	assert!(tris.len() == input.lines().count());
	let count = tris.into_iter()
		.filter(|&(a, b, c)| {
			(
				a + b > c &&
				a + c > b &&
				b + c > a
			)
		})
		.count();
	format!("{}", count)
}

fn transpose(m: Vec<Triangle>) -> Vec<Triangle> {
	let mut col0 = vec![];
	let mut col1 = vec![];
	let mut col2 = vec![];

	for t in m.into_iter() {
		col0.push(t.0);
		col1.push(t.1);
		col2.push(t.2);
	}
	col0.into_iter()
		.chain(col1.into_iter())
		.chain(col2.into_iter())
		.tuples::<(_, _, _)>()
		.collect()
}

pub fn part2(input: String) -> String {
	let tris = parse_only(all_tris, input.as_bytes()).unwrap();
	assert!(tris.len() == input.lines().count());
	let tris = transpose(tris);
	let count = tris.into_iter()
		.filter(|&(a, b, c)| {
			(
				a + b > c &&
				a + c > b &&
				b + c > a
			)
		})
		.count();
	format!("{}", count)
}
