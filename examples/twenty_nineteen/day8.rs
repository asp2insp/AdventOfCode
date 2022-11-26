use itertools::*;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

pub fn part1(input: String) -> String {
	let pixels: Vec<u32> = input.chars()
		.filter_map(|c| c.to_digit(10))
		.collect();
	let ans = pixels[..].chunks(WIDTH * HEIGHT)
		.min_by_key(|buf| buf.iter().filter(|n| **n == 0).count())
		.map(|buf| buf.iter().filter(|n| **n == 1).count() * buf.iter().filter(|n| **n == 2).count())
		.unwrap();
	format!("{}", ans)
}


fn print_pic(pixels: &[u32; WIDTH * HEIGHT]) {
	for h in 0..HEIGHT {
		for w in 0..WIDTH {
			match pixels[h*WIDTH + w] {
				1 => print!("█"),
				0 => print!(" "),
				_ => unreachable!(),
			};
		}
		print!("\n");
	}
}

pub fn part2(input: String) -> String {
	let pixels: Vec<u32> = input.chars()
		.filter_map(|c| c.to_digit(10))
		.collect();
	let ans = pixels[..].chunks(WIDTH * HEIGHT)
		.rev()
		.fold([2u32; WIDTH * HEIGHT], |mut curr, frame| {
			for (c, f) in curr.iter_mut().zip(frame.iter()) {
				*c = if *f == 2 { *c } else { *f }
			}
			curr
		});
	print_pic(&ans);
	"^^".to_owned()
}
