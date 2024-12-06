use aoc::utils::*;
use std::collections::HashMap;

fn roll(g: &Grid<()>, d: Direction) -> Grid<()> {
    let mut g = g.clone();
    loop {
        let ng = g.new_iter(|p, c, _| {
            if c == '.'
                && g.drive(p, d.opposite())
                    .map(|p2| g.read_pt(&p2) == 'O')
                    .unwrap_or(false)
            {
                ('O', ())
            } else if c == 'O'
                && g.drive(p, d)
                    .map(|p2| g.read_pt(&p2) == '.')
                    .unwrap_or(false)
            {
                ('.', ())
            } else {
                (c, ())
            }
        });
        if g == ng {
            break;
        } else {
            g = ng;
        }
    }
    g
}

pub fn part1(input: String) -> String {
    let mut g = Grid::new(&input, ());
    roll(&g, Direction::N)
        .iter_chars()
        .filter(|(_, c)| *c == 'O')
        .map(|(p, _)| 1 + p.y)
        .sum::<isize>()
        .to_string()
}

fn run_cycle(g: &Grid<()>) -> Grid<()> {
    let mut g = roll(&g, Direction::N);
    g = roll(&g, Direction::W);
    g = roll(&g, Direction::S);
    g = roll(&g, Direction::E);
    g
}

pub fn part2(input: String) -> String {
    let mut g = Grid::new(&input, ());
    let mut seen = HashMap::new();
    let mut i = 0;
    while !seen.contains_key(&g.to_string()) {
        seen.insert(g.to_string(), i);
        g = run_cycle(&g);
        i += 1;
        // println!("{}\n", g.to_string())
    }
    let cycle = i - seen.get(&g.to_string()).unwrap();
    println!("Found cycle of len {} after {} iterations", cycle, i);
    let remainder = (1000000000 - i) % cycle;
    for _ in 0..remainder {
        g = run_cycle(&g);
    }
    g.iter_chars()
        .filter(|(_, c)| *c == 'O')
        .map(|(p, _)| 1 + p.y)
        .sum::<isize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const I1: &str = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    #[test]
    fn test_simple() {
        assert_eq!(part1(I1.to_string()), "136".to_string());
    }

    #[test]
    fn test_cycle() {
        assert_eq!(part2(I1.to_string()), "64".to_string());
    }
}
