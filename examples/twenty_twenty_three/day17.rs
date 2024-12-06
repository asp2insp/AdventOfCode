use aoc::utils::*;
use itertools::Itertools;

pub fn part1(input: String) -> String {
    use Direction::*;
    let g = Grid::new_with(&input, |c| c.to_digit(10).unwrap() as isize);
    let goal = Point::new(g.right_bound, g.bottom_bound);
    a_star(
        (Point::new(g.left_bound, g.top_bound), [N, N, N]),
        |&(pt, _dirs)| pt.dist(&goal),
        |(pt, dirs)| {
            g.neighbors_with_directions(pt)
                .map(|np| (np, g.get(np.1).map(|v| v.1).unwrap_or(isize::MAX)))
                .filter(|(np, _)| np.0 != dirs[0].opposite())
                .filter(|(np, _)| np.0 != dirs[0] || np.0 != dirs[1] || np.0 != dirs[2])
                .map(|(np, c)| ((np.1, [np.0, dirs[0], dirs[1]]), c))
                .collect_vec()
        },
        |&(pt, _)| pt == goal,
    )
    .unwrap()
    .to_string()
}

pub fn part2(input: String) -> String {
    use Direction::*;
    let g = Grid::new_with(&input, |c| c.to_digit(10).unwrap() as isize);
    let goal = Point::new(g.right_bound, g.bottom_bound);
    a_star(
        (
            Point::new(g.left_bound, g.top_bound),
            [N, N, N, N, N, N, N, N, N, N],
        ),
        |&(pt, _dirs)| pt.dist(&goal),
        |(pt, dirs)| {
            g.neighbors_with_directions(pt)
                .map(|np| (np, g.get(np.1).map(|v| v.1).unwrap_or(isize::MAX)))
                .filter(|(np, _)| np.0 != dirs[0].opposite())
                .filter(|(np, _)| {
                    if np.0 == dirs[0] {
                        // If we're going the same direction, cap at 10
                        dirs.iter().any(|d| *d != np.0)
                    } else {
                        // If we're turning, we need to go at least 4 first
                        dirs.iter().take(4).unique().count() == 1
                    }
                })
                .map(|(np, c)| {
                    (
                        (
                            np.1,
                            [
                                np.0, dirs[0], dirs[1], dirs[2], dirs[3], dirs[4], dirs[5],
                                dirs[6], dirs[7], dirs[8],
                            ],
                        ),
                        c,
                    )
                })
                .collect_vec()
        },
        |&(pt, _)| pt == goal,
    )
    .unwrap()
    .to_string()
}

#[test]
fn test_example() {
    let input = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
        .to_string();
    assert_eq!(part1(input), "102");
}

#[test]
fn test_even_simpler() {
    let input = r"112999
911111"
        .to_string();
    assert_eq!(part1(input), "7");
}
