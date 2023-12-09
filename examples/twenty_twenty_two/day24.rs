use aoc::{makeset, utils::*};
use fnv::FnvHashMap;
use Direction::*;

type Blizzards = FnvHashMap<usize, Vec<(Point, Direction)>>;

fn calc_locations<'a>(
    time: usize,
    g: &Grid<()>,
    hist: &'a mut Blizzards,
) {
    if !hist.contains_key(&time) {
        calc_locations(time - 1, g, hist);
        let prev = hist.get(&(time - 1)).unwrap();
        let next = prev
            .iter()
            .map(|(p, d)| {
                let mut r = p.offset_dir(*d);
                if r.x == g.left_bound {
                    r.x = g.right_bound - 1;
                } else if r.x == g.right_bound {
                    r.x = g.left_bound + 1;
                } else if r.y == g.bottom_bound {
                    r.y = g.top_bound - 1;
                } else if r.y == g.top_bound {
                    r.y = g.bottom_bound + 1;
                }
                (r, *d)
            })
            .collect();
        hist.insert(time, next);
    }
}

fn make_journey(start: Point, goal: Point, time: usize, g: &Grid<()>, blizzards_by_time: &mut Blizzards) -> usize {
	let mut seen = makeset!();
    let mut q = vec![(start, time)];
    let mut best = usize::MAX;
    while let Some((np, nt)) = q.pop() {
        if np == goal {
            if nt < best {
                best = nt;
                // println!("Found new best time of {}", nt);
            }
            continue;
        }
        if nt > best {
            continue;
        }
        if seen.contains(&(np, nt)) {
            continue;
        }
        seen.insert((np, nt));
        calc_locations(nt + 1, &g, blizzards_by_time);
        let b_loc = blizzards_by_time.get(&(nt + 1)).unwrap();
        g.neighbors(np)
            .chain(std::iter::once(np))
            .filter(|p| !b_loc.iter().any(|(b, _)| b == p))
            .for_each(|p| q.push((p, nt + 1)));
        q.sort_by_key(|p| isize::MAX - p.0.dist(&goal));
    }
	best
}

fn parse(s: &str) -> (Grid<()>, Blizzards) {
	let g = Grid::new(s, ()).with_wall('#');
    let blizzards: Vec<(Point, Direction)> = g
        .iter_chars()
        .filter_map(|(p, c)| match c {
            '^' => Some((p, N)),
            'v' => Some((p, S)),
            '>' => Some((p, E)),
            '<' => Some((p, W)),
            _ => None,
        })
        .collect();
    let mut blizzards_by_time: FnvHashMap<usize, Vec<(Point, Direction)>> = FnvHashMap::default();
    blizzards_by_time.insert(0, blizzards);
	(g, blizzards_by_time)
}

pub fn part1(input: String) -> String {
    let (g, mut blizzards_by_time) = parse(&input);
    let start = Point::new(g.left_bound + 1, g.top_bound);
    let goal = Point::new(g.right_bound - 1, g.bottom_bound);
    let best = make_journey(start, goal, 0, &g, &mut blizzards_by_time);
    best.to_string()
}

pub fn part2(input: String) -> String {
	let (g, mut blizzards_by_time) = parse(&input);
    let start = Point::new(g.left_bound + 1, g.top_bound);
    let goal = Point::new(g.right_bound - 1, g.bottom_bound);
    let best = make_journey(start, goal, 0, &g, &mut blizzards_by_time);
	let best = make_journey(goal, start, best, &g, &mut blizzards_by_time);
	let best = make_journey(start, goal, best, &g, &mut blizzards_by_time);
    best.to_string()
}

#[test]
fn test() {
    let s = r#"#.######
	#>>.<^<#
	#.<..<<#
	#>v.><>#
	#<^v^^>#
	######.#"#
        .to_owned();
    assert_eq!("18", part1(s.clone()));
}
