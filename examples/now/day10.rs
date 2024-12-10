use aoc::makeset;
use aoc::utils::*;

pub fn part1(input: String) -> String {
    let g = Grid::new_with(&input, |c| c.to_digit(10).unwrap() as usize);
    let trailheads = g.find_all('0');
    trailheads
        .into_iter()
        .map(|th| {
            g.bfs_generic(
                makeset!(th),
                Some(&|n| {
                    g.neighbors(n)
                        .filter(|np| g.get(*np).unwrap().1 == g.get(n).unwrap().1 + 1)
                        .map(|p| (p, 1))
                        .collect::<Vec<_>>()
                }),
                None,
            )
            .keys()
            .filter(|&p| g.get(*p).unwrap().1 == 9)
            .count()
        })
        .sum::<usize>()
        .to_string()
}

fn count_paths(g: &Grid<usize>, start: Point) -> usize {
    if g.get(start).unwrap().1 == 9 {
        1
    } else {
        g.neighbors(start)
            .filter(|np| g.get(*np).unwrap().1 == g.get(start).unwrap().1 + 1)
            .map(|p| count_paths(g, p))
            .sum()
    }
}

pub fn part2(input: String) -> String {
	let g = Grid::new_with(&input, |c| c.to_digit(10).unwrap() as usize);
    let trailheads = g.find_all('0');
    trailheads
        .into_iter()
        .map(|th| count_paths(&g, th))
		.sum::<usize>()
		.to_string()
}

#[cfg(test)]
const EX: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

#[test]
fn test() {
    assert_eq!(part1(EX.to_string()), "36");
}
