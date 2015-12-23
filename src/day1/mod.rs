pub fn part1(input: String) -> String {
    format!("{}", input.chars().fold(0, |floor, p| {
        match p {
            '(' => floor + 1,
            ')' => floor - 1,
            _   => floor,
        }
    }))
}

fn index_if_new(floor: i32, record: usize, this: usize) -> usize {
    if record == usize::max_value() && floor == -1 {
        this
    } else {
        record
    }
}

pub fn part2(input: String) -> String {
    format!("{}", input.chars().enumerate().fold((0, usize::max_value()), |pair, p| {
        match p.1 {
            '(' => (pair.0 + 1, pair.1),
            ')' => (pair.0 - 1, index_if_new(pair.0-1, pair.1, p.0)),
            _   => pair,
        }
    }).1+1)
}
