use aoc::dict;
use aoc::utils::*;
use fnv::FnvHashMap;
use itertools::Itertools;

fn is_antinode(pts: &[Point], test: Point) -> bool {
    pts.iter().combinations(2).any(|pts| {
        test.is_on_line(pts[0], pts[1])
            && (test.dist(pts[0]) == 2 * test.dist(pts[1])
                || test.dist(pts[1]) == 2 * test.dist(pts[0]))
    })
}

fn find_antennas(g: &Grid<()>) -> FnvHashMap<char, Vec<Point>> {
    let mut map: FnvHashMap<char, Vec<Point>> = dict!();
    g.iter_chars()
        .for_each(|(p, c)| map.entry(c).or_default().push(p));
    map.remove(&'.');
    map
}

pub fn part1(input: String) -> String {
    let g = Grid::new(&input, ());
    let antennas = find_antennas(&g);
    g.iter()
        .filter(|(p, _, _)| antennas.values().any(|pts| is_antinode(pts, *p)))
        .count()
        .to_string()
}

fn is_antinode2(pts: &[Point], test: Point) -> bool {
	pts.len() > 1 && pts.contains(&test) ||
    pts.iter().combinations(2).any(|pts| {
        test.is_on_line(pts[0], pts[1])
    })
}

pub fn part2(input: String) -> String {
    let g = Grid::new(&input, ());
    let antennas = find_antennas(&g);
    g.iter()
        .filter(|(p, _, _)| antennas.values().any(|pts| is_antinode2(pts, *p)))
        .count()
        .to_string()
}

#[test]
fn test() {
    let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
    assert_eq!("14", part1(input.to_string()));
	assert_eq!("34", part2(input.to_string()));
}
