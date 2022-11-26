use itertools::*;

fn score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn score2(c: char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn is_match(a: char, b: char) -> bool {
    match (a, b) {
        ('(', ')') => true,
        ('[', ']') => true,
        ('{', '}') => true,
        ('<', '>') => true,
        _ => false,
    }
}

pub fn part1(input: String) -> String {
    input
        .lines()
        .map(|l| {
            l.trim().chars().try_fold(vec![], |mut stack, c| match c {
                '(' | '[' | '{' | '<' => {
                    stack.push(c);
                    Ok(stack)
                }
                ')' | ']' | '}' | '>' => {
                    if let Some(l) = stack.pop() {
                        if is_match(l, c) {
                            Ok(stack)
                        } else {
                            Err(score(c))
                        }
                    } else {
                        Err(score(c))
                    }
                }
                _ => unreachable!(),
            })
        })
        .partition::<Vec<_>, _>(Result::is_ok)
        .1
        .into_iter()
        .map(Result::unwrap_err)
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let mut scores = input
        .lines()
        .map(|l| {
            l.trim().chars().try_fold(vec![], |mut stack, c| match c {
                '(' | '[' | '{' | '<' => {
                    stack.push(c);
                    Ok(stack)
                }
                ')' | ']' | '}' | '>' => {
                    if let Some(l) = stack.pop() {
                        if is_match(l, c) {
                            Ok(stack)
                        } else {
                            Err(score(c))
                        }
                    } else {
                        Err(score(c))
                    }
                }
                _ => unreachable!(),
            })
        })
        .partition::<Vec<_>, _>(Result::is_ok)
        .0
        .into_iter()
        .map(Result::unwrap)
        .map(|s| s.into_iter().rev().map(score2).fold(0, |r, s| r * 5 + s))
        .collect::<Vec<_>>();
    scores.sort();
    scores[scores.len() / 2].to_string()
}

#[test]
fn test() {
    assert_eq!("1197", part1("{([(<{}[<>[]}>{[]{[(<()>".to_string()));
    let test_in = r"[({(<(())[]>[[{[]{<()<>>
		[(()[<>])]({[<{<<[]>>(
		{([(<{}[<>[]}>{[]{[(<()>
		(((({<>}<{<{<>}{[]{[]{}
		[[<[([]))<([[{}[[()]]]
		[{[{({}]{}}([{[{{{}}([]
		{<[[]]>}<{[{[{[]{()[[[]
		[<(<(<(<{}))><([]([]()
		<{([([[(<>()){}]>(<<{{
		<{([{{}}[<[[[<>{}]]]>[]]"
        .to_string();
    assert_eq!("26397", part1(test_in.clone()));
	assert_eq!("288957", part2(test_in));
}
