
fn parse(s: &str) -> Vec<usize> {
	s.lines().filter_map(|l| l.parse::<usize>().ok()).collect()
}

fn abs_diff(a: usize, b: usize) -> usize {
	if a > b { a - b } else { b - a}
}


pub fn part1(input: String) -> String {
	let mut adapters = parse(&input);
	adapters.sort();
	adapters.insert(0, 0);
	adapters.push(adapters.last().unwrap() + 3);
	let ones = adapters.windows(2).filter(|win| abs_diff(win[0], win[1]) == 1).count();
	let threes = adapters.windows(2).filter(|win| abs_diff(win[0], win[1]) == 3).count();
	(ones * threes).to_string()
}

fn num_orders(i: usize, adapters: &[usize], table: &mut[usize]) -> usize {
	if table[i] > 0 {
		return table[i]
	}
	let curr = adapters[i];
	let mut total = 0;
	for cand in i.saturating_sub(3)..i {
		if abs_diff(adapters[cand], curr) <= 3 {
			total += num_orders(cand, adapters, table);			
		}
	}
	table[i] = total;
	total
}


pub fn part2(input: String) -> String {
	let mut adapters = parse(&input);
	adapters.sort();
	adapters.insert(0, 0);
	let mut table: Vec<usize> = vec![0; adapters.len()];
	let n = table.len() - 1;
	table[0] = 1;
	num_orders(table.len() - 1, &adapters, &mut table[..]).to_string()
}
