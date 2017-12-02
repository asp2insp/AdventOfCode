pub fn part1(input: String) -> String {
	(0..)
		.map(|i| {
			let mut sh = Md5::new();
			sh.input(format!("{}{}", input, i).as_bytes());
			(i, sh.result_str())
		}).filter(|pair| {
		  pair.1.chars().take(5).all(|c| c == '0')
		})
		.map(|pair| {
			//println!("{}", pair.1);
			pair.1.chars().nth(5).unwrap()
		})
		.take(8)
		.collect()
	"part1".to_string()
}


pub fn part2(input: String) -> String {
	"part2".to_string()
}
