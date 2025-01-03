use aoc::utils::*;
use itertools::Itertools;
use regex::Regex;

fn measure_area(g: &Grid<()>) -> usize {
    let mut by_y = HashMap::new();
    for (p, _) in g.iter_chars().filter(|(_, c)| *c == '#') {
        by_y.entry(p.y).or_insert_with(Vec::new).push(p.x);
    }
    by_y.into_iter()
        .map(|(_, mut v)| {
            v.sort();
            let mut sum = 0;
            let mut inside = true;
            for (l, r) in v.iter().tuple_windows() {
                if r - l == 1 {
                    continue;
                } else if inside {
                    sum += r - l - 1;
                }
                inside = !inside;
            }
            sum += v.len() as isize; // Add the walls themselves
            println!("{:?} {}", v, sum);
            sum
        })
        .sum::<isize>() as usize
}

pub fn part1(input: String) -> String {
    let mut g = Grid::new_with_bounds(0, 0, 0, 0, |_| ('.', ()));
    let mut v = vec![];
    g.clear_bounds();
    let mut p = Point::new(0, 0);
    for l in input.lines() {
        let parts = l.split_whitespace().collect_vec();
        let dir = match parts[0] {
            "U" => Direction::N,
            "D" => Direction::S,
            "L" => Direction::W,
            "R" => Direction::E,
            _ => panic!("Unknown direction"),
        };
        let dist = parts[1].parse::<i32>().unwrap();
        v.push(p);
        for _ in 0..dist {
            p = g.drive(p, dir).unwrap();
            g.set(p, '#', ());
        }
    }
    v.push(p);
    let ans = v
        .into_iter()
        .tuple_windows()
        .map(|(p1, p2)| (p1.x * p2.y - p1.y * p2.x) - p1.dist(&p2))
        .sum::<isize>()
        .abs()
        / 2
        + 1;
    ans.to_string()
    // let inside = g.flood_search_by_pred(Point::new(1, 1), |pf, pt| g.get(pf).map(|c| c.0).unwrap_or('.') != '#' && g.get(pt).map(|c| c.0).unwrap_or('.') != '#').len();
    // let outside = g.iter_chars().filter(|&(_, c)| c == '#').count();
    // (inside + outside).to_string()
}

pub fn part2(input: String) -> String {
    let mut v = vec![];
    let mut p = Point::new(0, 0);
    let re = Regex::new(r"([UDLR]) (\d+) \(#(\w+)\)").unwrap();
    for l in input.lines() {
        let caps = re.captures(l).unwrap();
        let parts = caps.get(3).unwrap().as_str().chars().collect_vec();
        let dir = match parts[5] {
            '3' => Direction::N,
            '1' => Direction::S,
            '2' => Direction::W,
            '0' => Direction::E,
            _ => panic!("Unknown direction"),
        };
        let dist = u32::from_str_radix(&parts.into_iter().take(5).join(""), 16).unwrap();
        v.push(p);
        p = p.offset_dir_long(dir, dist as isize);
    }
    v.push(p);
    let ans = v
        .into_iter()
        .tuple_windows()
        .map(|(p1, p2)| (p1.x * p2.y - p1.y * p2.x) - p1.dist(&p2))
        .sum::<isize>()
        .abs()
        / 2
        + 1;
    ans.to_string()
}

#[test]
fn test2() {
    let input = r"R 6 (#70c710)
	D 5 (#0dc571)
	L 2 (#5713f0)
	D 2 (#d2c081)
	R 2 (#59c680)
	D 2 (#411b91)
	L 5 (#8ceee2)
	U 2 (#caa173)
	L 1 (#1b58a2)
	U 2 (#caa171)
	R 2 (#7807d2)
	U 3 (#a77fa3)
	L 2 (#015232)
	U 2 (#7a21e3)"
        .to_string();
    assert_eq!(part1(input.clone()), "62".to_string());
    assert_eq!(part2(input), "952408144115".to_string());
}
