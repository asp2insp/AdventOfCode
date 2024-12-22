use aoc::{
    dict,
    utils::{gimme_usizes_once, CloneWith, Direction, DIRECTIONS},
};
use cached::proc_macro::cached;
use fnv::FnvHashMap;
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

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
const DIRPAD: &[&[char]] = &[&['<', 'v', '>'], &[' ', '^', 'A']];

#[derive(Clone, Debug)]
struct Bot {
    x: usize,
    y: usize,
    pad: &'static [&'static [char]],
}

fn reps(v: &[char]) -> usize {
    if v.is_empty() {
        return 0;
    }
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

fn do_further_first(seq: &[char]) -> usize {
    let mut count = 0;
    for i in 0..seq.len() {
        if seq[i] == '<' || seq[i] == 'v' {
            for j in i + 1..seq.len() {
                if seq[j] == '^' || seq[j] == '>' {
                    count += 1;
                }
            }
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
                if path.len() > moves.len() {
                    continue;
                }
                let path_reps = reps(&path);
                let moves_reps = reps(&moves);
                if path.len() == moves.len() && path_reps < moves_reps {
                    continue;
                }
                if path.len() == moves.len()
                    && path_reps == moves_reps
                    && do_further_first(&path) < do_further_first(&moves)
                {
                    continue;
                }
                moves = path;
                dest = (x, y);
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

    fn all_moves_to(&mut self, target: char) -> Vec<Vec<char>> {
        let mut moves = vec![];
        let mut dest = (0, 0);
        let mut q = VecDeque::new();
        q.push_back((self.x, self.y, Vec::new()));
        while let Some((x, y, path)) = q.pop_front() {
            if self.pad[y][x] == target {
                if moves.is_empty() {
                    moves = vec![path];
                } else if path.len() > moves[0].len() {
                    continue;
                } else if path.len() < moves[0].len() {
                    moves = vec![path];
                } else if path.len() == moves[0].len() {
                    moves.push(path);
                }
                dest = (x, y);
                continue;
            }
            if !moves.is_empty() && path.len() > moves[0].len() {
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
        moves.iter_mut().for_each(|m| m.push('A'));
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

fn find_shortest_moves_25(code: &str) -> usize {
    let mut bot = Bot::new_num_bot();
    let d: Vec<char> = code.chars().flat_map(|c| bot.do_moves_to(c)).collect();
    d.split_inclusive(|c| *c == 'A')
        .map(|c| shortest_rec(c.iter().join(""), 25))
        .sum()
}

#[cached]
fn shortest_rec(code: String, rec: usize) -> usize {
    if rec == 0 {
        return code.len();
    }
    let mut bot = Bot::new_dir_bot();
    let d = code
        .chars()
        .flat_map(|c| bot.do_moves_to(c))
        .collect::<String>();
    d.split_inclusive('A')
        .map(|s| shortest_rec(s.to_string(), rec - 1))
        .sum()
}

fn dp() -> FnvHashMap<(usize, char, char), usize> {
    let mut map = dict!();
    for (a, b) in "^v<>A".chars().cartesian_product("^v<>A".chars()) {
        // At level 0 (me), it only costs 1 press for any button in any seq of buttons
        map.insert((0, a, b), 1);
    }
    for layer in 1..25 + 1 {
        for (a, b) in "^v<>A".chars().cartesian_product("^v<>A".chars()) {
            let mut bot = Bot::new_dir_bot();
            let _ = bot.do_moves_to(a);
            let all_dirs = bot.all_moves_to(b);
            let total_cost = all_dirs
                .into_iter()
                .map(|mut dirs| {
                    dirs.insert(0, 'A');
                    dirs.windows(2)
                        .map(|w| {
                            map.get(&(layer - 1, w[0], w[1]))
                                .expect(&format!("{}, {:?}", layer, w))
                        })
                        .sum()
                })
                .min()
                .unwrap();

            // println!("{}, {:?}, {:?}, {}", layer, a, b, total_cost);
            map.insert((layer, a, b), total_cost);
        }
    }
    for (a, b) in "A0123456789"
        .chars()
        .cartesian_product("A0123456789".chars())
    {
        let mut bot = Bot::new_num_bot();
        let layer = 26;
        let _ = bot.do_moves_to(a);
        let all_dirs = bot.all_moves_to(b);
        let total_cost = all_dirs
            .into_iter()
            .map(|mut dirs| {
                dirs.insert(0, 'A');
                dirs.windows(2)
                    .map(|w| {
                        map.get(&(layer - 1, w[0], w[1]))
                            .expect(&format!("{}, {:?}", layer, w))
                    })
                    .sum()
            })
            .min()
            .unwrap();

        // println!("{}, {:?}, {:?}, {}", layer, a, b, total_cost);
        map.insert((layer, a, b), total_cost);
    }
    map
}

pub fn part1(input: String) -> String {
    input
        .lines()
        .map(|line| complexity(line, find_shortest_moves(line).len()))
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    // Should be 223285811665866
    // instead i 223920874588746, only 480A is wrong
    // println!("{:?}", input.lines().map(|line| dp(line)).collect_vec());
    let map = dp();
    input
        .lines()
        .map(|line| {
            let fstr = format!("A{}", line).chars().collect_vec();
            let shortest_moves = fstr
                .windows(2)
                .filter_map(|w| map.get(&(26, w[0], w[1])))
                .sum::<usize>();
            println!("{}, {:?}", line, shortest_moves);
            complexity(line, shortest_moves)
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_num_bot() {
        // 029A
        let mut bot = Bot::new_num_bot();
        assert_eq!(bot.do_moves_to('0').iter().join(""), "<A");
        assert_eq!(bot.do_moves_to('2').iter().join(""), "^A");
        assert_eq!(bot.do_moves_to('9').iter().join(""), ">^^A");
        assert_eq!(bot.do_moves_to('A').iter().join(""), "vvvA");
        bot.do_moves_to('A');
        assert_eq!("^^<<A", bot.do_moves_to('4').iter().join(""));
        assert_eq!(">^A", bot.do_moves_to('8').iter().join(""));
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
        // println!("{:?}", find_shortest_moves("379A").into_iter().join(""));
        println!(
            "{:?}",
            Bot::new_dir_bot().apply_seq(
                &Bot::new_dir_bot()
                    .apply_seq(
                        "v<<A>>^AvA^Av<<A>>^AAv<A<A>>^AAvAA^<A>Av<A^>AA<A>Av<A<A>>^AAA<Av>A^A"
                    )
                    .into_iter()
                    .join("")
            )
        );
        println!(
            "{:?}",
            Bot::new_dir_bot().apply_seq(
                &Bot::new_dir_bot()
                    .apply_seq("<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A")
                    .into_iter()
                    .join("")
            )
        );
        assert_eq!(
            find_shortest_moves("379A").len(),
            "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
        );
    }

    #[test]
    fn test_part2_rec() {
        let mut b = Bot::new_num_bot();
        let dirs = "379A"
            .chars()
            .flat_map(|c| b.do_moves_to(c))
            .collect::<String>();
        assert_eq!(
            64,
            dirs.split_inclusive('A')
                .map(|s| shortest_rec(s.to_string(), 2))
                .sum::<usize>()
        );
        let dirs = "456A"
            .chars()
            .flat_map(|c| b.do_moves_to(c))
            .collect::<String>();
        assert_eq!(
            64,
            dirs.split_inclusive('A')
                .map(|s| shortest_rec(s.to_string(), 2))
                .sum::<usize>()
        );
        let dirs = "179A"
            .chars()
            .flat_map(|c| b.do_moves_to(c))
            .collect::<String>();
        assert_eq!(
            68,
            dirs.split_inclusive('A')
                .map(|s| shortest_rec(s.to_string(), 2))
                .sum::<usize>()
        );
        let dirs = "029A"
            .chars()
            .flat_map(|c| b.do_moves_to(c))
            .collect::<String>();
        assert_eq!(
            68,
            dirs.split_inclusive('A')
                .map(|s| shortest_rec(s.to_string(), 2))
                .sum::<usize>()
        );
    }

    #[test]
    fn test_slowly_increase() {
        // First level (seed)
        let d = "<A^A>^^AvvvA".to_string();
        let mut b = Bot::new_dir_bot();
        // Second level
        let d2 = d.chars().flat_map(|c| b.do_moves_to(c)).collect::<String>();
        let rec2 = d
            .split_inclusive('A')
            .map(|s| shortest_rec(s.to_string(), 1))
            .sum::<usize>();
        assert_eq!(d2.len(), rec2);
        // Third level
        let d3 = d2
            .chars()
            .flat_map(|c| b.do_moves_to(c))
            .collect::<String>();
        let rec3 = d2
            .split_inclusive('A')
            .map(|s| shortest_rec(s.to_string(), 1))
            .sum::<usize>();
        let rec32 = d
            .split_inclusive('A')
            .map(|s| shortest_rec(s.to_string(), 2))
            .sum::<usize>();
        assert_eq!(d3.len(), rec3);
        assert_eq!(d3.len(), rec32);
        // Fourth level
        let d4 = d3
            .chars()
            .flat_map(|c| b.do_moves_to(c))
            .collect::<String>();
        let rec4 = d3
            .split_inclusive('A')
            .map(|s| shortest_rec(s.to_string(), 1))
            .sum::<usize>();
        let rec43 = d
            .split_inclusive('A')
            .map(|s| shortest_rec(s.to_string(), 3))
            .sum::<usize>();
        let rec42 = d2
            .split_inclusive('A')
            .map(|s| shortest_rec(s.to_string(), 2))
            .sum::<usize>();
        assert_eq!(d4.len(), rec4);
        assert_eq!(d4.len(), rec43);
        assert_eq!(d4.len(), rec42);
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
