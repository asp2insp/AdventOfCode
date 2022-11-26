use itertools::*;
use regex::*;

struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

fn parse_line(s: &str, re: &Regex) -> Claim {
    let cap = re.captures(s).unwrap();
    Claim {
        id: cap[1].parse().unwrap(),
        left: cap[2].parse().unwrap(),
        top: cap[3].parse().unwrap(),
        width: cap[4].parse().unwrap(),
        height: cap[5].parse().unwrap(),
    }
}

pub fn part1(input: String) -> String {
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let claims = input
        .lines()
        .map(|l| parse_line(l, &re))
        .collect::<Vec<_>>();
    let mut map = [[0u8; 1000]; 1000];
    for claim in claims {
        for x in claim.left..claim.left + claim.width {
            for y in claim.top..claim.top + claim.height {
                map[x][y] += 1;
            }
        }
    }
    let count_overlaps = map
        .iter()
        .flat_map(|r| r.iter())
        .fold(0, |acc, n| acc + if *n > 1 { 1 } else { 0 });
    format!("{}", count_overlaps)
}

pub fn part2(input: String) -> String {
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let claims = input
        .lines()
        .map(|l| parse_line(l, &re))
        .collect::<Vec<_>>();
    let mut map = [[0u8; 1000]; 1000];
    for claim in &claims {
        for x in claim.left..claim.left + claim.width {
            for y in claim.top..claim.top + claim.height {
                map[x][y] += 1;
            }
        }
    }
    'outer: for claim in claims {
        for x in claim.left..claim.left + claim.width {
            for y in claim.top..claim.top + claim.height {
                if map[x][y] > 1 {
                    continue 'outer;
                }
            }
        }
        return format!("{}", claim.id);
    }
    format!("No answer")
}
