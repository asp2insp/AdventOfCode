pub fn part1(input: String) -> String {
	let mut nums = input.split(",").map(|s| parse!(s, isize)).collect::<Vec<_>>();
	let min = *nums.iter().min().unwrap();
	let max = *nums.iter().max().unwrap();
	
	(min..=max).map(|d| nums.iter().map(|n| (n-d).abs()).sum::<isize>()).min().unwrap().to_string()

}


pub fn part2(input: String) -> String {
	let mut nums = input.split(",").map(|s| parse!(s, isize)).collect::<Vec<_>>();
	let min = *nums.iter().min().unwrap();
	let max = *nums.iter().max().unwrap();
	
	(min..=max).map(|d| nums.iter().map(|n| (n-d).abs()).map(|c| (c*c + c)/2).sum::<isize>()).min().unwrap().to_string()

}
