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
const KEYPAD: [[char; 3]; 4] = [
    [' ', '0', 'A'],
    ['1', '2', '3'],
    ['4', '5', '6'],
    ['7', '8', '9'],
];

#[derive(Clone, Debug)]
struct NumBot {
    x: usize,
    y: usize,
}

impl NumBot {
    fn new() -> Self {
        Self { x: 2, y: 0 }
    }

    fn do_moves_to(&mut self, target: char) -> Vec<char> {
        let mut moves = Vec::new();
        let mut dest = (0, 0);
        let mut q = VecDeque::new();
        let mut visited = makeset!();
        q.push_back((self.x, self.y, Vec::new()));
        while let Some((x, y, path)) = q.pop_front() {
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));
            if KEYPAD[y][x] == target {
                moves = path;
                dest = (x, y);
                break;
            }
            for d in DIRECTIONS.iter() {
                let nx = x as isize + d.x_offset();
                let ny = y as isize + d.y_offset();
                if nx < 0 || nx >= 3 || ny < 0 || ny >= 4 || KEYPAD[ny as usize][nx as usize] == ' '
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
                ret.push(KEYPAD[self.y][self.x]);
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
const DIRPAD: [[char; 3]; 2] = [['<', 'v', '>'], [' ', '^', 'A']];
#[derive(Clone, Debug)]
struct DirBot {
    x: usize,
    y: usize,
}

impl DirBot {
    fn new() -> Self {
        Self { x: 2, y: 1 }
    }

    fn do_moves_to(&mut self, target: char) -> Vec<char> {
        let mut moves = Vec::new();
        let mut dest = (0, 0);
        let mut q = VecDeque::new();
        let mut visited = makeset!();
        q.push_back((self.x, self.y, Vec::new()));
        while let Some((x, y, path)) = q.pop_front() {
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));
            if DIRPAD[y][x] == target {
                moves = path;
                dest = (x, y);
                break;
            }
            for d in DIRECTIONS.iter() {
                let nx = x as isize + d.x_offset();
                let ny = y as isize + d.y_offset();
                if nx < 0 || nx >= 3 || ny < 0 || ny >= 2 || DIRPAD[ny as usize][nx as usize] == ' '
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
                ret.push(DIRPAD[self.y][self.x]);
            } else {
                let d = Direction::from_char(c).unwrap();
                self.x = (self.x as isize + d.x_offset()) as usize;
                self.y = (self.y as isize + d.y_offset()) as usize;
            }
        }
        ret
    }
}

fn complexity(code: &str, shortest_moves: usize) -> usize {
    gimme_usizes_once(code)[0] * shortest_moves
}

fn find_shortest_moves(code: &str) -> Vec<char> {
    let mut bot = NumBot::new();
    let num_bot_dirs: Vec<char> = code.chars().flat_map(|c| bot.do_moves_to(c)).collect();
    let mut space_bot = DirBot::new();
    let dir_bot_dirs: Vec<char> = num_bot_dirs
        .into_iter()
        .flat_map(|c| space_bot.do_moves_to(c))
        .collect();
    let mut cold_bot = DirBot::new();
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
        let mut bot = NumBot::new();
        assert_eq!(bot.do_moves_to('0').iter().join(""), "<A");
        assert_eq!(bot.do_moves_to('2').iter().join(""), "^A");
        assert_eq!(bot.do_moves_to('9').iter().join(""), "^^>A");
        assert_eq!(bot.do_moves_to('A').iter().join(""), "vvvA");
    }

    #[test]
    fn test_dir_bot() {
        // 029A first moves is "<A" on the NumBot to get 0
        let mut bot = DirBot::new();
        assert_eq!(bot.do_moves_to('<').iter().join(""), "v<<A");
        assert_eq!(bot.do_moves_to('A').iter().join(""), ">^>A");
        // followed by "^A" to get 2
        assert_eq!(bot.do_moves_to('^').iter().join(""), "<A");
        assert_eq!(bot.do_moves_to('A').iter().join(""), ">A");
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
        let mut bot = NumBot::new();
        assert_eq!(bot.apply_seq("<A^A>^^AvvvA"), vec!['0', '2', '9', 'A']);
        let mut bot = DirBot::new();
        assert_eq!(
            bot.apply_seq("v<<A>>^A<A>AvA<^AA>A<vAAA>^A")
                .iter()
                .join(""),
            "<A^A>^^AvvvA"
        );
        let mut bot = DirBot::new();
        assert_eq!(
            bot.apply_seq("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A")
                .iter()
                .join(""),
            "v<<A>>^A<A>AvA<^AA>A<vAAA>^A"
        );
    }
}
