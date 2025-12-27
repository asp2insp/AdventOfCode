use itertools::Itertools;

use crate::utils::*;

fn is_invalid(mut i: usize) -> bool {
    let tens = (i as f64).log10() as u32;
    let half_size = (tens + 1) / 2;
    (i / 10usize.pow(half_size)) == (i % 10usize.pow(half_size))
}

fn parse(s: &str) -> Vec<BetterRange> {
    s.split(",")
        .map(|r| {
            let (b, t) = r.split("-").collect_tuple().unwrap();
            BetterRange::new_unordered_inclusive(b.parse().unwrap(), t.parse().unwrap())
        })
        .collect_vec()
}

pub fn part1(input: String) -> String {
    let ranges = parse(&input);
    let mut sum = 0;
    for r in ranges {
        for i in r.iter() {
            if is_invalid(i) {
                // println!("Invalid: {}", i);
                sum += i;
            }
        }
    }
    sum.to_string()
}

fn is_invalid_2(i: usize) -> bool {
    let s = i.to_string();
	for i in 1..=s.len()/2 {
		let sub = s.chars().take(i).collect::<String>();
		if s.split(&sub).all(|inter| inter.is_empty()) {
			return true
		}
	}
	false
}

pub fn part2(input: String) -> String {
    let ranges = parse(&input);
    let mut sum = 0;
    for r in ranges {
        for i in r.iter() {
            if is_invalid_2(i) {
                // println!("Invalid: {}", i);
                sum += i;
            }
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_invalid() {
        assert!(is_invalid(55));
        assert!(is_invalid(6464));
        assert!(is_invalid(11));
        assert!(is_invalid(123123));
    }

    #[test]
    fn test_valid() {
        assert!(!is_invalid(63));
        assert!(!is_invalid(123124));
    }

	#[test]
	fn test_invalid_2() {
		assert!(is_invalid_2(11));
		assert!(is_invalid_2(22));
		assert!(is_invalid_2(38593859));
		assert!(is_invalid_2(565656));
	}
}
