use aoc::{
    makeset,
    utils::{gimme_usizes_once, CloneWith, Direction, DIRECTIONS},
};
use itertools::Itertools;
use std::collections::VecDeque;

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
const KEYPAD: &[&[char]] = &[
    &[' ', '0', 'A'],
    &['1', '2', '3'],
    &['4', '5', '6'],
    &['7', '8', '9'],
];

#[derive(Clone, Debug)]
struct Bot {
    x: usize,
    y: usize,
    pad: &'static [&'static [char]],
}

fn reps(v: &[char]) -> usize {
    let mut count = 0;
    let mut last = v[0];
    for c in v.iter().skip(1) {
        if *c == last {
            count += 1;
        } else {
            last = *c;
        }
    }
    count
}

impl Bot {
    fn new_num_bot() -> Self {
        Self {
            x: 2,
            y: 0,
            pad: &KEYPAD,
        }
    }

    fn new_dir_bot() -> Self {
        Self {
            x: 2,
            y: 1,
            pad: &DIRPAD,
        }
    }

    fn do_moves_to(&mut self, target: char) -> Vec<char> {
        let mut moves = vec![' '; 10];
        let mut dest = (0, 0);
        let mut q = VecDeque::new();
        q.push_back((self.x, self.y, Vec::new()));
        while let Some((x, y, path)) = q.pop_front() {
            if path.len() > moves.len() {
                continue;
            }
            if self.pad[y][x] == target {
                let is_new_shortest = path.len() < moves.len()
                    || (path.len() == moves.len() && reps(&path) > reps(&moves));
                if is_new_shortest {
                    moves = path;
                    dest = (x, y);
                }
                continue;
            }
            for d in DIRECTIONS.iter() {
                let nx = x as isize + d.x_offset();
                let ny = y as isize + d.y_offset();
                if nx < 0
                    || nx >= self.pad[0].len() as isize
                    || ny < 0
                    || ny >= self.pad.len() as isize
                    || self.pad[ny as usize][nx as usize] == ' '
                {
                    continue;
                }
                let new_path = path.clone_with(d.as_arrow_char());
                q.push_back((nx as usize, ny as usize, new_path));
            }
        }
        (self.x, self.y) = dest;
        moves.push('A');
        moves
    }

    fn apply_seq(&mut self, seq: &str) -> Vec<char> {
        let mut ret = Vec::new();
        for c in seq.chars() {
            if c == 'A' {
                ret.push(self.pad[self.y][self.x]);
            } else {
                let d = Direction::from_char(c).unwrap();
                self.x = (self.x as isize + d.x_offset()) as usize;
                self.y = (self.y as isize + d.y_offset()) as usize;
            }
        }
        ret
    }
}

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
const DIRPAD: &[&[char]] = &[&['<', 'v', '>'], &[' ', '^', 'A']];

fn complexity(code: &str, shortest_moves: usize) -> usize {
    gimme_usizes_once(code)[0] * shortest_moves
}

fn find_shortest_moves(code: &str) -> Vec<char> {
    let mut bot = Bot::new_num_bot();
    let num_bot_dirs: Vec<char> = code.chars().flat_map(|c| bot.do_moves_to(c)).collect();
    let mut space_bot = Bot::new_dir_bot();
    let dir_bot_dirs: Vec<char> = num_bot_dirs
        .into_iter()
        .flat_map(|c| space_bot.do_moves_to(c))
        .collect();
    let mut cold_bot = Bot::new_dir_bot();
    dir_bot_dirs
        .into_iter()
        .flat_map(|c| cold_bot.do_moves_to(c))
        .collect()
}

pub fn part1(input: String) -> String {
    input
        .lines()
        .map(|line| complexity(line, find_shortest_moves(line).len()))
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    "part2".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_bot() {
        // 029A
        let mut bot = Bot::new_num_bot();
        assert_eq!(bot.do_moves_to('0').iter().join(""), "<A");
        assert_eq!(bot.do_moves_to('2').iter().join(""), "^A");
        assert_eq!(bot.do_moves_to('9').iter().join(""), "^^>A");
        assert_eq!(bot.do_moves_to('A').iter().join(""), "vvvA");
    }

    #[test]
    fn test_dir_bot() {
        // 029A first moves is "<A" on the NumBot to get 0
        let mut bot = Bot::new_dir_bot();
        assert_eq!(bot.do_moves_to('<').iter().join(""), "v<<A");
        assert_eq!(bot.do_moves_to('A').iter().join(""), ">>^A");
        // followed by "^A" to get 2
        assert_eq!(bot.do_moves_to('^').iter().join(""), "<A");
        assert_eq!(bot.do_moves_to('A').iter().join(""), ">A");

        let mut bot = Bot::new_dir_bot();
        assert_eq!(
            "<A^A>^^AvvvA"
                .chars()
                .flat_map(|c| bot.do_moves_to(c))
                .count(),
            "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len()
        );

        assert_eq!(
            "v<<A>>^A<A>AvA<^AA>A<vAAA>^A"
                .chars()
                .flat_map(|c| bot.do_moves_to(c))
                .count(),
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            find_shortest_moves("029A").len(),
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
        );
        assert_eq!(
            find_shortest_moves("980A").len(),
            "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".len()
        );
        assert_eq!(
            find_shortest_moves("179A").len(),
            "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
        );
        assert_eq!(
            find_shortest_moves("456A").len(),
            "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".len()
        );
        assert_eq!(
            find_shortest_moves("379A").len(),
            "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
        );
    }

    #[test]
    fn test_apply() {
        let mut bot = Bot::new_num_bot();
        assert_eq!(bot.apply_seq("<A^A>^^AvvvA"), vec!['0', '2', '9', 'A']);
        let mut bot = Bot::new_dir_bot();
        assert_eq!(
            bot.apply_seq("v<<A>>^A<A>AvA<^AA>A<vAAA>^A")
                .iter()
                .join(""),
            "<A^A>^^AvvvA"
        );
        assert_eq!(
            bot.apply_seq("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A")
                .iter()
                .join(""),
            "v<<A>>^A<A>AvA<^AA>A<vAAA>^A"
        );
    }
}
