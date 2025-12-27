use aoc::utils::gimme_usize_nums;
use aoc::utils::IterUtils;

pub fn part1(input: String) -> String {
    let (mut a, mut b) =
        gimme_usize_nums(&input)
            .into_iter()
            .fold((Vec::new(), Vec::new()), |mut acc, x| {
                acc.0.push(x[0]);
                acc.1.push(x[1]);
                acc
            });
    a.sort();
    b.sort();
    a.into_iter()
        .zip(b.into_iter())
        .map(|(x, y)| x.abs_diff(y))
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let (a, b) =
        gimme_usize_nums(&input)
            .into_iter()
            .fold((Vec::new(), Vec::new()), |mut acc, x| {
                acc.0.push(x[0]);
                acc.1.push(x[1]);
                acc
            });
    let counts = b.into_iter().counting_set();
    a.into_iter()
        .map(|x| counts.get(&x).unwrap_or(&0) * x)
        .sum::<usize>()
        .to_string()
}
