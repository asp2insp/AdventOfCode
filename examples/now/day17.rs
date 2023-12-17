use aoc::utils::*;
use itertools::Itertools;
use std::collections::HashMap;

fn bfs(start: Point, end: Point, g: &Grid<u32>) -> Option<u32> {
	use Direction::*;
    let mut states: HashMap<(Point, [Direction; 3]), u32> = HashMap::new();
    let mut pts = vec![(start, [N, N, N], 0)];
    while pts.len() > 0 {
        pts.iter().for_each(|s| {
			if s.2 < *states.get(&(s.0, s.1)).unwrap_or(&u32::MAX) {
            	states.insert((s.0, s.1), s.2);
			}
        });
		// println!("{}: {:?}", i, pts);
        pts = pts
            .into_iter()
            .flat_map(|(pt, dirs, cost)| {
                g.neighbors_with_directions(pt)
                    .map(|np| (np, cost + g.get(np.1).map(|v| v.1).unwrap_or(u32::MAX)))
                    .filter(|(np, _)| np.0 != dirs[0].opposite())
                    .filter(|(np, _)| np.0 != dirs[0] || np.0 != dirs[1] || np.0 != dirs[2])
                    .map(|(np, c)| (np.1, [np.0, dirs[0], dirs[1]], c))
                    .collect_vec()
                    .into_iter()
            })
            .filter(|s| s.2 < *states.get(&(s.0, s.1)).unwrap_or(&u32::MAX))
            .collect_vec();
    }
    states
        .into_iter()
        .filter(|(pt, _)| pt.0 == end)
        .map(|(_pts, cost)| {
            println!("{:?} {}", _pts, cost);
            cost
        })
        .min()
}

pub fn part1(input: String) -> String {
    let g = Grid::new_with(&input, |c| c.to_digit(10).unwrap_or(u32::MAX));
    bfs(
        Point::new(g.left_bound, g.top_bound),
        Point::new(g.right_bound, g.bottom_bound),
        &g,
    )
    .unwrap()
    .to_string()
}

pub fn part2(input: String) -> String {
    "part2".to_string()
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
911111".to_string();
	assert_eq!(part1(input), "6");
}