use itertools::Itertools;

fn parse_input(s: &str) -> Vec<isize> {
	s.lines()
		.map(|l| l.split_at(1))
		.map(|(ll, lr)| match ll {
			"R" => 1,
			"L" => -1,
			_ => 0,
		} * lr.parse::<isize>().unwrap())
		.collect_vec()
}

pub fn part1(input: String) -> String {
	let mut num = 50isize;
	let mut count = 0;
	for n in parse_input(&input) {
		num += n;
		num %= 100;
		if num == 0 {
			count += 1;
		}
	}

	count.to_string()
}


pub fn part2(input: String) -> String {
	let mut num = 50isize;
	let mut prev_num = num;
	let mut count = 0;
	for n in parse_input(&input) {
		num += n;
		count  += (num.div_euclid(100) - prev_num.div_euclid(100)).abs();
		if n < 0 {
			if prev_num == 0 {
				count -= 1;
			}
			if num.rem_euclid(100) == 0 {
				count += 1;
			}
		}
		num = (num.rem_euclid(100)).abs();
		prev_num = num;
	}
	count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_l150_should_be_2() {
        let result = part2("L150".to_string());
        assert_eq!(result, "2");
    }

    #[test]
    fn test_r150_should_be_2() {
        let result = part2("R150".to_string());
        assert_eq!(result, "2");
    }

    #[test]
    fn test_r_start_at_0() {
        let result = part2("L50\nR100".to_string());
        assert_eq!(result, "2");
    }

    #[test]
    fn test_l_start_at_0() {
        let result = part2("L50\nL100".to_string());
        assert_eq!(result, "2");
    }

    #[test]
    fn test_100() {
        let result = part2("L100\nR50".to_string());
        assert_eq!(result, "2");
    }
}