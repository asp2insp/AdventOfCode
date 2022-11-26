
fn get_bits_count<'a>(mut input: &[&str], size: usize) -> Vec<(usize, usize)> {
	let mut ge = vec![(0, 0); size];
	for l in input {
		for (i, c) in l.chars().enumerate() {
			match c {
				'0' => ge[i].0 += 1,
				'1' => ge[i].1 += 1,
				_ => unimplemented!(),
			};
		}
	}
	ge
}


pub fn part1(input: String) -> String {
	let size = input.lines().next().unwrap().len();
	let ge = get_bits_count(&input.lines().collect::<Vec<_>>(), size);
	let s_gamma:String = ge.iter().map(|(zeros, ones)| if zeros > ones { '0' } else { '1' }).collect();
	let s_epsilon:String = ge.iter().map(|(zeros, ones)| if zeros > ones { '1' } else { '0' }).collect();

	let gamma = u32::from_str_radix(&s_gamma, 2).unwrap();
	let epsilon = u32::from_str_radix(&s_epsilon, 2).unwrap();
	format!("{}", gamma * epsilon)
}


pub fn part2(input: String) -> String {
	let size = input.lines().next().unwrap().len();
	let nums: Vec<&str> = input.lines().collect();
	let mut ge = get_bits_count(&nums, size);

	let mut oxy_candidates = nums.clone();
	let mut oxy_num = 1;
	for i in 0..ge.len() {
		let desired = if ge[i].0 > ge[i].1 { '0' } else { '1' };
		oxy_candidates = oxy_candidates.into_iter().filter(|c| c.chars().nth(i).unwrap() == desired).collect();
		ge = get_bits_count(&oxy_candidates, size);

		if oxy_candidates.len() == 1 {
			oxy_num = u32::from_str_radix(oxy_candidates[0], 2).unwrap();
			break;
		}
	}

	ge = get_bits_count(&nums, size);
	let mut co2_candidates = nums.clone();
	let mut co2_num = 0;
	for i in 0..ge.len() {
		let desired = if ge[i].1 >= ge[i].0 { '0' } else { '1' };
		co2_candidates = co2_candidates.into_iter().filter(|c| c.chars().nth(i).unwrap() == desired).collect();
		ge = get_bits_count(&co2_candidates, size);

		if co2_candidates.len() == 1 {
			co2_num = u32::from_str_radix(co2_candidates[0], 2).unwrap();
			break;
		}
	}

	format!("{} * {} = {}", oxy_num, co2_num, oxy_num * co2_num)
}
