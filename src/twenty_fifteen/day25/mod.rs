fn mod_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
	if modulus == 1 {
		return 0
	}
	let mut result = 1;
	let mut base = base % modulus;
	let mut exp = exponent;
	while exp > 0 {
		if exp & 1 == 1 {
			result = (result * base) % modulus;
		}
		exp = exp >> 1;
		base = (base * base) % modulus;
	}
	result
}

fn rc_to_nth(r: u64, c: u64) -> u64 {
	(r-1)*r/2 + (r + c - 1)*(r + c)/2 - r*(r + 1)/2
}

pub fn part1(input: String) -> String {
	let m = 20151125;
	let a = 252533;
	let p = 33554393;

	// Row 3010, Col 3019 is the nth number in the sequence, given by:
	// (r-1)(r)/2 + (r+c-1)(r+c)/2 - (r)(r+1)/2 = nth
	// So:
	//(3009*3010)/2 + 1 + (3010+3020)(3010+3019)/2 - 3010*3011/2 = 18174426
	let n = rc_to_nth(3010, 3019);
	// compute (m * a^18174426) % p
	let r = (m * mod_pow(a, n, p)) % p;
	format!("m*a^{} % p = {}", n, r)
}


pub fn part2(input: String) -> String {
	"part2".to_string()
}
