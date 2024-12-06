use aoc::parse;
use aoc::utils::*;

pub fn part1(input: String) -> String {
    input
        .lines()
        .map(|l| {
            let mut digits = l.chars().filter(is_digit);
            let f = digits.next().unwrap();
            let l = digits.last().unwrap_or(f);
            let n = format!("{}{}", f, l);
            parse!(n, u32)
        })
        .sum::<u32>()
        .to_string()
}

/// If the string starting from s represents a digit, either
/// literal or spelled out, return it. Otherwise None
fn grab_digit_from_s(s: &str, i: usize) -> Option<u32> {
    if s[i..].starts_with("0") || s[i..].starts_with("zero") {
        return Some(0);
    } else if s[i..].starts_with("1") || s[i..].starts_with("one") {
        return Some(1);
    } else if s[i..].starts_with("2") || s[i..].starts_with("two") {
        return Some(2);
    } else if s[i..].starts_with("3") || s[i..].starts_with("three") {
        return Some(3);
    } else if s[i..].starts_with("4") || s[i..].starts_with("four") {
        return Some(4);
    } else if s[i..].starts_with("5") || s[i..].starts_with("five") {
        return Some(5);
    } else if s[i..].starts_with("6") || s[i..].starts_with("six") {
        return Some(6);
    } else if s[i..].starts_with("7") || s[i..].starts_with("seven") {
        return Some(7);
    } else if s[i..].starts_with("8") || s[i..].starts_with("eight") {
        return Some(8);
    } else if s[i..].starts_with("9") || s[i..].starts_with("nine") {
        return Some(9);
    } else {
        None
    }
}

pub fn part2(input: String) -> String {
    input
        .lines()
        .map(|l| {
            let mut digits = (0..l.len()).flat_map(|i| grab_digit_from_s(l, i));
            let f = digits.next().unwrap();
            let l = digits.last().unwrap_or(f);
            let n = format!("{}{}", f, l);
            parse!(n, u32)
        })
        .sum::<u32>()
        .to_string()
}
