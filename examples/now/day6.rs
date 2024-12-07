use aoc::makeset;
use aoc::utils::Direction::*;
use aoc::utils::Grid;
use aoc::utils::Point;
use fnv::FnvHashSet;

use rayon::prelude::*;

fn find_visited(g: &Grid<()>) -> FnvHashSet<Point> {
    let mut curr = (g.find('^').unwrap(), N);
    let mut seen = makeset!(curr.0);
    loop {
        match g.drive(curr.0, curr.1) {
            None => break,
            Some(p) if g.read_pt(&p) != '#' => {
                curr = (p, curr.1);
                seen.insert(p);
            }
            Some(_p) => {
                curr = (curr.0, curr.1.turn('R'));
            }
        }
    }
    seen
}

pub fn part1(input: String) -> String {
    let g = Grid::new(&input, ());
    find_visited(&g).len().to_string()
}

fn is_loop(g: &Grid<()>, block: Point) -> bool {
    let mut g = g.clone();
    g.set(block, '#', ());
    let mut curr = (g.find('^').unwrap(), N);
    let mut seen = makeset!(curr);
    loop {
        match g.drive(curr.0, curr.1) {
            None => break,
            Some(p) if g.read_pt(&p) != '#' => {
                curr = (p, curr.1);
                if !seen.insert(curr) {
                    return true;
                }
            }
            Some(_p) => {
                curr = (curr.0, curr.1.turn('R'));
            }
        }
    }
    return false;
}

pub fn part2(input: String) -> String {
    let g = Grid::new(&input, ());
    // let obs_x = g.find_all('#').into_iter().map(|p| p.x).collect::<FnvHashSet<isize>>();
    // let obs_y = g.find_all('#').into_iter().map(|p| p.y).collect::<FnvHashSet<isize>>();
    find_visited(&g)
        .into_par_iter()
        .filter(|p| g.read_pt(p) == '.')
        // .filter(|&p| obs_x.contains(&p.x) || obs_y.contains(&p.y))
        .filter(|&p| is_loop(&g, p))
        .count()
        .to_string()
}

#[test]
fn test() {
    let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
    assert_eq!(part1(input.to_string()), "41");
    assert_eq!(part2(input.to_string()), "6");
}
