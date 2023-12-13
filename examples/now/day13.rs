use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<Vec<char>>> {
    let mut result = vec![];
    let mut block = vec![];
    for line in input.lines() {
        if line.is_empty() {
            if !block.is_empty() {
                result.push(block);
            }
            block = vec![];
            continue;
        }
        block.push(line.chars().collect());
    }
    result.push(block);
    result
}

fn palistr(s: &str, idx: usize) -> bool {
    is_palindrome(&s.chars().collect_vec(), idx)
}

fn is_palindrome(s: &[char], idx: usize) -> bool {
    let len = s.len();
    let mut l = idx;
    let mut r = idx + 1;
    loop {
        if r >= len {
            break;
        }
        if s[l] != s[r] {
            return false;
        }
        if l == 0 || r == len - 1 {
            break;
        }
        l -= 1;
        r += 1;
    }
    true
}

fn is_col_palindrome(s: &[Vec<char>], col: usize, idx: usize) -> bool {
    let len = s.len();
    let mut l = idx;
    let mut r = idx + 1;
    loop {
        if r >= len {
            break;
        }
        if s[l][col] != s[r][col] {
            return false;
        }
        if l == 0 || r == len - 1 {
            break;
        }
        l -= 1;
        r += 1;
    }
    true
}

fn get_block_palindrome_score(block: Vec<Vec<char>>) -> Vec<usize> {
    (0..block[0].len() - 1)
        .filter(|idx| block.iter().all(|row| is_palindrome(row, *idx)))
        .map(|r| r + 1)
        .chain(
            (0..block.len() - 1)
                .filter(|idx| (0..block[0].len()).all(|col| is_col_palindrome(&block, col, *idx)))
                .map(|c| (c + 1) * 100),
        )
        .collect_vec()
}

fn find_different_palindrome_score(block: Vec<Vec<char>>) -> usize {
    let score = get_block_palindrome_score(block.clone())[0];
    for row in 0..block.len() {
        for col in 0..block[0].len() {
            let mut bc = block.clone();
            if block[row][col] == '#' {
                bc[row][col] = '.';
            } else {
                bc[row][col] = '#';
            }
            let newscore = get_block_palindrome_score(bc.clone());
            if let Some(n) = newscore.into_iter().find(|n| *n != 0 && *n != score) {
                return n;
            }
        }
    }
    panic!("no different palindrome found")
}

pub fn part1(input: String) -> String {
    parse(&input)
        .into_iter()
        .map(get_block_palindrome_score)
        .map(|v| v[0])
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    parse(&input)
        .into_iter()
        .map(find_different_palindrome_score)
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

    const EX2: &str = r"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_pali() {
        assert!(palistr("aa", 0));
        assert!(!palistr("#.##..##.", 2));
        assert!(!palistr(&"#.##..##.", 3));

        // ex1
        assert!(palistr(&"#.##..##.", 4));
        assert!(palistr(&"..#.##.#.", 4));
        assert!(palistr(&"##......#", 4));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX1.to_string()), "5");
        assert_eq!(part1(EX2.to_string()), "400");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX1.to_string()), "300");
        // assert_eq!(part2(EX2.to_string()), "100");
    }
}
