use itertools::Itertools;

fn sigma(p: u32, a: u32) -> u32 {
	 (p.pow(a+1) - 1) / (p-1)
}

fn factors(n: u32, primes: &Vec<u32>) -> Vec<(u32, u32)> {
	let mut factors: Vec<u32> = vec![];
	let mut num = n;
	let mut last_num = n;
	while num > 1 {
		for ip in primes.iter().enumerate() {
			let p = *ip.1;
			if num % p == 0 {
				factors.push(p);
				num /= p;
				break;
			}
		}
		if last_num == num {
			// This is prime, just return it
			factors.push(num);
			break;
		} else {
			last_num = num;
		}
	}
	factors.sort();
	factors.iter()
		.group_by(|f| *f)
		.map(|kv| (*kv.0, kv.1.len() as u32))
		.collect()
}

fn primes_u32() -> Vec<u32> {
	// sqrt(2^32) = 65536. PrimePi(65536 + 1) = 6543
	let mut primes = vec![0; 6543];
	primes[0] = 2;
	primes[1] = 3;
	for i in (5..65538).step_by(2) {
		for p in &mut primes {
			if *p == 0 {
				// We've come to the end of our known primes, add this one
				*p = i;
				break;
			}
			if i % *p == 0 {
				break;
			}
		}
	}
	primes
}

fn house_presents(n: u32, primes: &Vec<u32>) -> u32 {
	// σ(p^a) = (p^a+1 − 1)/(p − 1)
	factors(n, primes).iter()
		.fold(1, |prod, pa| prod * sigma(pa.0, pa.1)) * 10
}

fn house_presents_2(n: u32) -> u32 {
	let mut sum = 0u32;
	for i in 1..50 {
		if n % i == 0 {
			sum += (n/i) * 11;
		}
	}
	sum
}


pub fn part1(input: String) -> String {
	let target:u32 = input[..].parse().unwrap();
	let primes = primes_u32();
	for i in 500_000u32.. {
		let p = house_presents(i, &primes);
		if p > target {
			return format!("i: {}, p: {}", i, p)
		}
	}
	format!("Exhausted search")
}


pub fn part2(input: String) -> String {
	let target:u32 = input[..].parse().unwrap();
	for i in 100_000u32.. {
		let p = house_presents_2(i);
		if p > target {
			return format!("i: {}, p: {}", i, p)
		}
	}
	format!("Exhausted search")
}
