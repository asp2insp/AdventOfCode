use itertools::Itertools;


fn from_snafu(s: &str) -> isize {
	let mut ret = 0;
	let mut op = 1;
	for c in s.chars().rev() {
		ret += match c {
			'2' => 2,
			'1' => 1,
			'0' => 0,
			'-' => -1,
			'=' => -2,
			_ => unreachable!(),
		} * op;
		op *= 5;
	}
	ret
}

fn to_snafu(mut i: isize) -> String {
	let mut ret = vec![];
	let mut op = 1;
	while op * 2 < i {
		op *= 5;
	}
	while i > 0 || op > 0 {
		let top = i / op;
		ret.push(top);
		for i in (0..ret.len()).rev() {
			match ret[i] {
				4 => {
					ret[i] = -1;
					if i == 0 {
						ret.insert(0, 1);
					} else {
						ret[i-1] += 1;
					}
				},
				3 => {
					ret[i] = -2;
					if i == 0 {
						ret.insert(0, 1);
					} else {
						ret[i-1] += 1;
					}
				},
				_ => {},
			};
		}
		i = i % op;
		op /= 5;
	}

	ret.into_iter().skip_while(|e| *e == 0).map(|j| {
		match j {
			-2 => '=',
			-1 => '-',
			0 => '0',
			1 => '1',
			2 => '2',
			_ => unreachable!(),
		}
	})
	.collect()
}

pub fn part1(input: String) -> String {
	to_snafu(input.lines().map(from_snafu).sum::<isize>())
}


pub fn part2(input: String) -> String {
	"part2".to_string()
}


#[test]
fn test() {
	let s = r#"1=-0-2     1747
	12111      906
	 2=0=      198
	   21       11
	 2=01      201
	  111       31
	20012     1257
	  112       32
	1=-1=      353
	 1-12      107
	   12        7
	   1=        3
	  122       37
	  20       10             
1=0       15            
1-0      20            
1=11-2     2022         
1-0---0    12345        
1121-1110-1=0 314159265  "#.lines().map(|l| l.split_whitespace().collect_vec()).collect_vec();

	for pair in s {
		let n = pair[1].trim().parse::<isize>().unwrap();
		let snafu = pair[0].trim();
		assert_eq!(n, from_snafu(snafu));
		assert_eq!(to_snafu(n), snafu);
	}
}