use regex::Regex;

#[derive(Default, Debug)]
struct Passport<'a> {
	byr: Option<&'a str>,
	iyr: Option<&'a str>,
	eyr: Option<&'a str>,
	hgt: Option<&'a str>,
	hcl: Option<&'a str>,
	ecl: Option<&'a str>,
	pid: Option<&'a str>,
	cid: Option<&'a str>,
}

impl<'a> Passport<'a> {
	fn is_valid(&self) -> bool {
		self.byr.is_some()
			&& self.iyr.is_some()
			&& self.eyr.is_some()
			&& self.hgt.is_some()
			&& self.hcl.is_some()
			&& self.ecl.is_some()
			&& self.pid.is_some()
	}

	fn is_valid_strict(&self) -> bool {
		self.byr
			.and_then(|yr| yr.parse::<usize>().ok())
			.map(|yr| 1920 <= yr && yr <= 2002)
			.unwrap_or(false)
			&& self
				.iyr
				.and_then(|yr| yr.parse::<usize>().ok())
				.map(|yr| 2010 <= yr && yr <= 2020)
				.unwrap_or(false)
			&& self
				.eyr
				.and_then(|yr| yr.parse::<usize>().ok())
				.map(|yr| 2020 <= yr && yr <= 2030)
				.unwrap_or(false)
			&& self
				.hgt
				.and_then(|hgt| {
					if hgt.ends_with("cm") {
						get_num(hgt).map(|n| n >= 150 && n <= 193)
					} else if hgt.ends_with("in") {
						get_num(hgt).map(|n| n >= 59 && n <= 76)
					} else {
						None
					}
				})
				.unwrap_or(false)
			&& self
				.hcl
				.map(|hcl| Regex::new("#[0-9a-fA-F]{6}").unwrap().is_match(hcl))
				.unwrap_or(false)
			&& self
				.ecl
				.map(|ecl| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl))
				.unwrap_or(false)
			&& self.pid.filter(|pid| pid.len() == 9).and_then(get_num).is_some()
	}
}

fn get_num(s: &str) -> Option<usize> {
	s.trim_matches(|c: char| !c.is_digit(10))
		.parse::<usize>()
		.ok()
}

fn parse(s: &str) -> Vec<Passport> {
	let mut lines = s.lines();
	let mut result = Vec::new();
	let mut pass: Passport = Default::default();
	while let Some(l) = lines.next() {
		if l.is_empty() {
			result.push(pass);
			pass = Default::default();
			continue;
		}
		for part in l.split_whitespace() {
			let mut kv = part.split(":");
			match kv.next().unwrap() {
				"byr" => pass.byr = kv.next(),
				"iyr" => pass.iyr = kv.next(),
				"eyr" => pass.eyr = kv.next(),
				"hgt" => pass.hgt = kv.next(),
				"hcl" => pass.hcl = kv.next(),
				"ecl" => pass.ecl = kv.next(),
				"pid" => pass.pid = kv.next(),
				"cid" => pass.cid = kv.next(),
				_ => unimplemented!(),
			}
		}
	}
	result.push(pass);
	result
}

pub fn part1(input: String) -> String {
	let passports = parse(&input);
	// println!("{:#?}", passports);
	passports
		.into_iter()
		.filter(Passport::is_valid)
		.count()
		.to_string()
}

pub fn part2(input: String) -> String {
	let passports = parse(&input);
	// println!("{:#?}", passports);
	passports
		.into_iter()
		.filter(Passport::is_valid_strict)
		.count()
		.to_string()
}
