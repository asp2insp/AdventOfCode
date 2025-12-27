use aoc::parse;
use aoc::utils::gimme_usize_nums;

fn possible(target: usize, current: usize, remaining: &[usize]) -> bool {
    if remaining.is_empty() {
        return current == target;
    }
    return possible(target, current + remaining[0], &remaining[1..])
        || possible(target, current * remaining[0], &remaining[1..]);
}

pub fn part1(input: String) -> String {
    gimme_usize_nums(&input)
        .into_iter()
        .filter(|vs| possible(vs[0], vs[1], &vs[2..]))
        .map(|vs| vs[0])
        .sum::<usize>()
        .to_string()
}

fn concat(a: usize, b: usize) -> usize {
    parse!(a.to_string() + &b.to_string(), usize)
}

fn possible2(target: usize, current: usize, remaining: &[usize]) -> bool {
    if remaining.is_empty() {
        return current == target;
    }
    return possible2(target, current + remaining[0], &remaining[1..])
        || possible2(target, current * remaining[0], &remaining[1..])
        || possible2(target, concat(current, remaining[0]), &remaining[1..]);
}

pub fn part2(input: String) -> String {
    gimme_usize_nums(&input)
        .into_iter()
        .filter(|vs| possible2(vs[0], vs[1], &vs[2..]))
        .map(|vs| vs[0])
        .sum::<usize>()
        .to_string()
}

#[test]
fn test() {
    assert_eq!(concat(48, 6), 486);
    assert_eq!(concat(17, 8), 178);
    assert!(possible2(312, 15, &[6, 2]));
    assert!(possible2(7290, 6, &[8, 6, 15]));
}
