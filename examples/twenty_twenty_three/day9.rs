use aoc::utils::gimme_nums;

fn extrapolate_value(nums: &[isize], dfun: impl Fn(&[isize], isize) -> isize) -> isize {
    let mut history = vec![];
    let mut last = nums.to_vec();
    while !last.iter().all(|n| *n == 0) {
        history.push(last.clone());
        last = last.windows(2).map(|w| w[1] - w[0]).collect();
    }
    let mut diff = 0;
    while !history.is_empty() {
        let last = history.pop().unwrap();
        diff = dfun(&last, diff);
    }
    diff
}

pub fn part1(input: String) -> String {
    gimme_nums(&input)
        .into_iter()
        .map(|nums| extrapolate_value(&nums, |last, diff| last.last().unwrap() + diff))
        .sum::<isize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    gimme_nums(&input)
        .into_iter()
        .map(|nums| extrapolate_value(&nums, |last, diff| last.first().unwrap() - diff))
        .sum::<isize>()
        .to_string()
}

const INPUT: &str = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

#[test]
fn test_example() {
    assert_eq!(part1(INPUT.to_string()), "114");
}
