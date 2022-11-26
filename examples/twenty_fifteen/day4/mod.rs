use crypto::md5::Md5;
use crypto::digest::Digest;

pub fn part1(input: String) -> String {
	let item = (0..)
		.map(|i| {
			let mut sh = Md5::new();
			sh.input(format!("{}{}", input, i).as_bytes());
			(i, sh.result_str())
		}).skip_while(|pair| {
		  !pair.1.chars().take(5).all(|c| c == '0')
		})
		.next();
	let i = item.unwrap();
	format!("{} -- {}", i.0, i.1)
}


pub fn part2(input: String) -> String {
	let item = (0..)
		.map(|i| {
			let mut sh = Md5::new();
			sh.input(format!("{}{}", input, i).as_bytes());
			(i, sh.result_str())
		}).skip_while(|pair| {
		  !pair.1.chars().take(6).all(|c| c == '0')
		})
		.next();
	let i = item.unwrap();
	format!("{} -- {}", i.0, i.1)
}
