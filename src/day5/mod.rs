use crypto::md5::Md5;
use crypto::digest::Digest;

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
}


pub fn part2(input: String) -> String {
	let mut temp = ['\0'; 8];
	let mut iter = (0..)
		.map(|i| {
			let mut sh = Md5::new();
			sh.input(format!("{}{}", input, i).as_bytes());
			(i, sh.result_str())
		}).filter(|pair| {
		  pair.1.chars().take(5).all(|c| c == '0')
		})
		.map(|pair| {
			(
				pair.1.chars().nth(5).unwrap().to_digit(10),
			 	pair.1.chars().nth(6).unwrap()
		 	)
		});
	while temp.iter().any(|c| *c == '\0') {
		let (pos, c) = iter.next().unwrap();
		if pos.is_none() {
			continue;
		}
		let pos = pos.unwrap() as usize;
		if pos >= 8 || temp[pos] != '\0' {
			continue
		}
		temp[pos] = c;
	}
	temp.iter().cloned().collect()
}
