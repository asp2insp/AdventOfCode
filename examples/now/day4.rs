use aoc::utils::Direction;
use aoc::utils::Grid;

pub fn part1(input: String) -> String {
    let g = Grid::new(&input, ());
    let starts = g.find_all('X');
    starts
        .iter()
        .flat_map(|s| {
            [Direction::N, Direction::S, Direction::W, Direction::E]
                .into_iter()
                .map(|d| {
                    g.drive_iter(*s, d)
                        .take(3)
                        .map(|c| g.read_pt(&c))
                        .collect::<String>()
                })
                .chain(
                    [
                        (Direction::N, Direction::W),
                        (Direction::N, Direction::E),
                        (Direction::S, Direction::W),
                        (Direction::S, Direction::E),
                    ]
                    .into_iter()
                    .map(|ds| {
                        g.drive_iter_diagonal(*s, ds)
                            .take(3)
                            .map(|c| g.read_pt(&c))
                            .collect::<String>()
                    }),
                )
        })
        .filter(|s| s == "MAS")
        .count()
        .to_string()
}

pub fn part2(input: String) -> String {
    let g = Grid::new(&input, ());
    let starts = g.find_all('A');
    starts
        .iter()
        .map(|s| {
            match [
                (Direction::N, Direction::W),
                (Direction::S, Direction::E),
                (Direction::N, Direction::E),
                (Direction::S, Direction::W),
            ]
            .into_iter()
            .filter_map(|(d1, d2)| g.drive2(*s, d1, d2))
            .map(|c| g.read_pt(&c))
            .collect::<String>()
            .as_str()
            {
                "MSSM" => true,
                "MSMS" => true,
                "SMSM" => true,
                "SMMS" => true,
                _ => false,
            }
        })
        .filter(|b| *b)
        .count()
        .to_string()
}
