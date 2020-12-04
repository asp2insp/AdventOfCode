use itertools::*;
use std::collections::HashMap;
use rayon::prelude::*;

const BASE_PAT: [i32; 4] = [0, 1, 0, -1];

fn make_pat(n: usize) -> impl Iterator<Item=i32> {
	use std::iter::repeat;

	repeat(BASE_PAT[0]).take(n)
		.chain(repeat(BASE_PAT[1]).take(n))
		.chain(repeat(BASE_PAT[2]).take(n))
		.chain(repeat(BASE_PAT[3]).take(n))
		.cycle()
		.skip(1)
}

fn parse_digits(s: &str) -> Vec<i32> {
	s.chars()
		.flat_map(|c| c.to_digit(10))
		.map(|n| n as i32)
		.collect()
}

fn fft_single(pos: usize, items: &[i32], repeats: usize) -> i32 {
	let ordinal = pos + 1;
	let after_pat: i32 = items
		.iter()
		.cycle()
		.take(repeats * items.len())
		.enumerate()
		.map(|(n, i)| (i, get_pat_n(ordinal, n)))
		.map(|(i, p)| *i * p)
		.sum();
	(after_pat % 10).abs()
}

fn fft(items: &[i32], repeats: usize, offset: usize) -> Vec<i32> {
	(0..items.len() * repeats)
		.into_par_iter()
		.map(|n| fft_single(n + offset, &items, repeats))
		.collect()
}

pub fn part1(input: String) -> String {
	let mut signal = parse_digits(&input);
	for _ in 0..100 {
		signal = fft(&signal, 1, 0);
	}
	signal[0..8].iter().map(|i| i.to_string()).join("")
}

fn get_pat_n(ordinal: usize, loc: usize) -> i32 {
	let strides = loc % (ordinal * 4);
	let quadrant = (strides + 1) / ordinal;
	BASE_PAT[quadrant % 4]
}

fn ifft(round: usize, pos: usize, repeats: usize, orig: &[i32]) -> i32 {
	let mut memo = HashMap::new();
	ifft_inner(round, pos, repeats, orig, &mut memo)
}

fn ifft_inner(round: usize, pos: usize, repeats: usize, orig: &[i32], memo: &mut HashMap<(usize, usize), i32>) -> i32 {
	if round == 0 {
		return orig[pos % orig.len()];
	}
	if memo.contains_key(&(round, pos)) {
		return *memo.get(&(round, pos)).unwrap()
	}
	let ordinal = pos + 1;
	let summed: i32 = (0..orig.len() * repeats)
		.filter_map(|n| {
			let p = get_pat_n(ordinal, n);
			if p == 0 {
				None
			} else {
				Some((n, p))
			}
		})
		.map(|(n, pat)| ifft_inner(round - 1, n, repeats, orig, memo) * pat)
		.sum();
	let ans = (summed % 10).abs();
	memo.insert((round, pos), ans);
	ans
}

fn fft_sum(items: &[i32]) -> Vec<i32> {
	let mut ret = Vec::with_capacity(items.len());
	let mut acc = 0;
	for i in items.iter().rev() {
		acc += i;
		ret.push( (acc % 10).abs() );
	}
	ret.reverse();
	ret
}

pub fn part2(input: String) -> String {
	let mut signal = parse_digits(&input);
	let skip: usize = input[0..7].parse().unwrap();

	let start = skip % signal.len();
	let len = signal.len() * 10_000 - skip;
	let mut new_signal = Vec::with_capacity(len);
	for i in 0..len {
		new_signal.push(signal[(start + i) % signal.len()]);
	}

	for _ in 0..100 {
		new_signal = fft_sum(&new_signal)
	}

	new_signal[0..8].iter().map(|i| i.to_string()).join("")
}

#[test]
fn test_pat() {
	let ordinal = 3;
	let control = make_pat(ordinal).take(20).collect::<Vec<_>>();
	let exp = (0..20).map(|n| get_pat_n(ordinal, n)).collect::<Vec<_>>();
	assert_eq!(control, exp);
}


#[test]
fn test_equiv() {
	let signal = parse_digits("12345678");
	let forward = fft(&signal, 1, 0);
	let reverse: Vec<i32> = (0..8).map(|i| ifft(1, i, 1, &signal)).collect();
	assert_eq!(forward, reverse);
}