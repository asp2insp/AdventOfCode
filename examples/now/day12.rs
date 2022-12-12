use aoc::{makeset, utils::*};
use itertools::Itertools;

fn ptval(p: &Point, g: &Grid<()>) -> usize {
    (match g.read_pt(p) {
        'E' => 'z',
        'S' => 'a',
        c => c,
    }) as usize
}

pub fn part1(input: String) -> String {
    let g = Grid::new(&input, ());
    let start = g.find('S').unwrap();
    let end = g.find('E').unwrap();
    g.bfs_generic(
        makeset! {start},
        Some(&|p| {
            g.neighbors(p)
                .filter(|p2| {
                    let nchar = ptval(p2, &g);
                    let currval = ptval(&p, &g);
                    let nval = nchar as usize;
                    nval <= currval + 1
                })
                .map(|p2| (p2, 1))
                .collect_vec()
        }),
        None,
    )
    // .to_debug_string()
    .get(&end)
    .unwrap()
    .0
    .to_string()
}

pub fn part2(input: String) -> String {
    let g = Grid::new(&input, ());
    let starts = g
        .iter_chars()
        .filter(|(_, c)| *c == 'a')
        .map(|(p, _)| p)
        .collect_vec();
    let end = g.find('E').unwrap();
    g.bfs_generic(
        makeset! {end},
        Some(&|p| {
            g.neighbors(p)
                .filter(|p2| {
                    let nchar = ptval(p2, &g);
                    let currval = ptval(&p, &g);
                    let nval = nchar as usize;
                    nval >= currval - 1
                })
                .map(|p2| (p2, 1))
                .collect_vec()
        }),
        None,
    )
    // .to_debug_string()
    .into_iter()
    .filter(|(p, _)| starts.contains(p))
    .min_by_key(|(_, (c, _))| *c)
    .unwrap()
    .1
     .0
    .to_string()
}
