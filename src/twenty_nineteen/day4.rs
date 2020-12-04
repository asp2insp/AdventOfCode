
fn has_adjacent_pair(num: &usize) -> bool {
	let mut num = *num;
	let mut first_digit = num % 10;
	while num > 0 {
		num = num / 10;
		if num % 10 == first_digit {
			return true;
		}
		first_digit = num % 10;
	}
	false
}

fn monotonic(num: &usize) -> bool {
	let mut num = *num;
	let mut first_digit = num % 10;
	while num > 0 {
		num = num / 10;
		if num % 10 > first_digit {
			return false;
		}
		first_digit = num % 10;
	}
	true
}

fn has_adjacent_pair_not_triple(num: &usize) -> bool {
	let mut num = *num;
	let mut check = 11;
	let mut first_digit = num % 10;
	while num > 0 {
		num = num / 10;
		if num % 10 == first_digit {
			if check != first_digit && (num / 10) % 10 != first_digit {
				return true;
			}
		}
		check = first_digit;
		first_digit = num % 10;
	}
	false
}

pub fn part1(input: String) -> String {
	(136760..=595730)
		.filter(has_adjacent_pair)
		.filter(monotonic)
		.count()
		.to_string()
}


pub fn part2(input: String) -> String {
	(136760..=595730)
		.filter(has_adjacent_pair_not_triple)
		.filter(monotonic)
		.count()
		.to_string()
}

#[test]
fn test_adjacent_pair() {
	assert_eq!(true, has_adjacent_pair_not_triple(&111122));
	assert_eq!(false, has_adjacent_pair_not_triple(&1111222));
	assert_eq!(false, has_adjacent_pair_not_triple(&123444));
	assert_eq!(true, has_adjacent_pair_not_triple(&112233));
	assert_eq!(false, has_adjacent_pair_not_triple(&1111111));
}