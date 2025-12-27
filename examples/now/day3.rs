pub fn part1(input: String) -> String {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<_>>()
        })
        .map(|v| find_max_toggle(&v, 2))
        .sum::<u64>()
        .to_string()
}

fn find_max_toggle(list: &[u64], rem: usize) -> u64 {
    if list.len() < rem || rem == 0 {
        return 0;
    }
    let (i, d) = list[0..list.len() - (rem - 1)]
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(&a.0)))
        .unwrap();
    d * 10u64.pow(rem as u32 - 1) + find_max_toggle(&list[i + 1..], rem - 1)
}

pub fn part2(input: String) -> String {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<_>>()
        })
        .map(|v| find_max_toggle(&v, 12))
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max() {
        assert_eq!(
            98,
            find_max_toggle(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 2)
        );
        assert_eq!(
            987654321111,
            find_max_toggle(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 12)
        );

        assert_eq!(
            89,
            find_max_toggle(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 2)
        );
        assert_eq!(
            811111111119,
            find_max_toggle(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 12)
        );

        assert_eq!(
            78,
            find_max_toggle(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 2)
        );
        assert_eq!(
            434234234278,
            find_max_toggle(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 12)
        );

        assert_eq!(
            92,
            find_max_toggle(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 2)
        );
        assert_eq!(
            888911112111,
            find_max_toggle(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 12)
        );
    }
}
