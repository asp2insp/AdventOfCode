use std::collections::HashMap;

pub fn part1(input: String) -> String {
	let count = input.split_whitespace()
		 .filter(|s| {
			 s.chars().filter(|c| {
				 match c {
					 &'a' | &'e' | &'i' | &'o' | &'u' => true,
					 _ => false
				 }
			 }).count() >= 3
		 })
		 .filter(|s| {
			 s.chars().fold(('\0', false), |state, c| {
				 (c, state.1 || state.0 == c)
			 }).1
		 })
		 .filter(|s| {
			 !s.contains("ab") &&
			 !s.contains("cd") &&
			 !s.contains("pq") &&
			 !s.contains("xy")
		 }).count();
	format!("{}", count)
}

pub fn part2(input: String) -> String {
	let count = input.split_whitespace()
		 .filter(|s| {
			 s.chars().fold(('\0', '\0', false), |state, c| {
				 (state.1, c, state.2 || state.0 == c)
			 }).2
		 })
		 .filter(|s| {
			 s.chars()
			  .zip(s.chars().skip(1))
			  .enumerate()
			  .fold(HashMap::<String, Vec<usize>>::new(), |mut m, p| {
				  let (a, b) = p.1;
				  m.entry(format!("{}{}", a, b)).or_insert(Vec::<usize>::new()).push(p.0);
				  m
			  })
			  .values()
			  .filter(|v| v.len() > 1)
			  .any(|v| {
				  let l = v[0];
				  v.iter().any(|i| i-l > 1)
			  })
		 })
		 .count();
	format!("{}", count)
}
