use aoc::{makeset, utils::Direction, utils::Grid};

pub fn part1(input: String) -> String {
    let mut g = Grid::new(&input, true);
    g.wall_char = '#';
    let mut states = makeset!(g.find('S').unwrap());
    for _ in 0..64 {
        states = states.into_iter().flat_map(|p| g.neighbors(p)).collect();
    }
    states.len().to_string()
}

// Here's the breakdown
// 26501365 = 202300 * 131 + 65
// Here's the samplings
// 65: 3944
// 196: 35082
// 327: 97230
// 458: 190388
// Fits the polynomial
// (15505 x^2)/17161 + (32273 x)/17161 + 76614/17161
// Evaluated at 26501365 yields 634549784009844
pub fn part2(input: String) -> String {
    let mut g = Grid::new(&input, true);
    let start = g.find('S').unwrap();
    let mut full = g.clone();
    g.set(start, '.', true);
    let mut row = g.clone();
    for _ in 0..5 {
        full.add_other(&g, Direction::E);
        full.add_other(&g, Direction::W);
        row.add_other(&g, Direction::E);
        row.add_other(&g, Direction::W);
    }
    for _ in 0..5 {
        full.add_other(&row, Direction::N);
        full.add_other(&row, Direction::S);
    }
    full.wall_char = '#';
    let mut states = makeset!(start);
    for i in 0..460 {
        if [65, 196, 327, 458].contains(&i) {
            println!("{}: {}", i, states.len());
        }
        states = states.into_iter().flat_map(|p| full.neighbors(p)).collect();
    }
    states.len().to_string()
}
