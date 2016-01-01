fn sigma(p: u32, a: u32) -> u32 {
	 (p.pow(a+1) - 1) / (p-1)
}

fn factors(n: usize) -> usize {
	0
}

fn primes() {

}

fn house_presents(n: u32, target: u32) -> u32 {
	let mut sum = 0u32;
	let top = (n as f32).sqrt().floor() as u32;
	for i in 1..top {
		if n % i == 0 {
			sum += 10 * i;
		}
		if sum > target {
			break;
		}
	}
	sum
}


pub fn part1(input: String) -> String {
	let target:u32 = input[..].parse().unwrap();
	let mut min = 1u32;
	let mut max = u32::max_value();
	loop {
		let i = min/2 + max/2;
		let p = house_presents(i, target);
		if p > target {
			println!("Testing {} -- got {} (too high)", i, p);
			max = i;
		} else if p < target {
			min = i;
			println!("Testing {} -- got {} (too low)", i, p);
		} else {
			return format!("Got {}", i)
		}
		if max < min {
			return format!("Got {}", i)
		}
	}
}


pub fn part2(input: String) -> String {
	"part2".to_string()
}

mod bitfield {
	pub struct Bitfield {
		bytes: Vec<u32>,
	}

	impl Bitfield {
		fn new(n: usize) -> Bitfield {
			Bitfield {
				bytes: vec![0u32; n],
			}
		}

		fn set(&mut self, n: usize) {
			let mut outer = self.bytes[n/32];
			outer |= 1 << (n%32);
			self.bytes[n/32] = outer;
		}

		fn get(&self, n: usize) -> bool {
			(self.bytes[n/32] & 1 << (n%32)) != 0
		}
	}
}
