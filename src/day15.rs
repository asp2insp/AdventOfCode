use chrono::{NaiveDateTime, Timelike};
use itertools::*;
use rayon::prelude::*;
use regex::*;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::mem;
use time::Duration;

const SIZE: usize = 33;
const AP: isize = 3;
const START_HP: isize = 200;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn dist(&self, rhs: &Point) -> usize {
        (self.x.max(rhs.x) - self.x.min(rhs.x)) + (self.y.max(rhs.y) - self.y.min(rhs.y))
    }

    fn adjacents(&self) -> impl Iterator<Item = Point> {
        let x = self.x as isize;
        let y = self.y as isize;
        vec![(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)]
            .into_iter()
            .filter(|(x1, y1)| *x1 >= 0 && *y1 >= 0)
            .filter(|(x1, y1)| *x1 < SIZE as isize && *y1 < SIZE as isize)
            .map(|(x1, y1)| Point {
                x: x1 as usize,
                y: y1 as usize,
            })
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Team {
    Goblin,
    Elf,
}
use self::Team::*;

#[derive(Debug, Clone)]
struct Unit {
    hp: isize,
    team: Team,
}

struct Map {
    real_size: usize,
    grid: [[bool; SIZE]; SIZE],
    units: HashMap<Point, Unit>,
    elf_ap: isize,
    dead_elf_count: usize,
}

impl Map {
    fn print_cell(&self, p: &Point) -> char {
        if self.grid[p.y][p.x] {
            '.'
        } else {
            '#'
        }
    }

    fn print(&self) -> String {
        (0..self.real_size)
            .map(|y| {
                let mut s = (0..self.real_size)
                    .map(|x| Point { x: x, y: y })
                    .map(|p| {
                        self.units
                            .get(&p)
                            .map(|u| u.print())
                            .unwrap_or(self.print_cell(&p))
                    })
                    .join("");
                s.push_str("  ");
                s.push_str(
                    &self
                        .units
                        .iter()
                        .filter(|(p, _)| p.y == y)
                        .sorted_by(|a, b| a.0.cmp(&b.0))
                        .into_iter()
                        .map(|(_, u)| u.print_hp())
                        .join(" "),
                );
                s
            })
            .join("\n")
    }

    fn dist(&self, from: Point, to: Point) -> usize {
        let mut mins = HashMap::new();
        let mut q: VecDeque<Point> = VecDeque::new();
        q.push_back(from);
        mins.insert(from, 0);
        'outer: while !q.is_empty() {
            if let Some(next) = q.pop_front() {
                let offer = mins.get(&next).map(|d| d + 1).unwrap_or(usize::max_value());
                for p in next
                    .adjacents()
                    .filter(|p| !self.units.contains_key(p))
                    .filter(|p| self.grid[p.y][p.x])
                {
                    let mut m = mins.entry(p).or_insert(usize::max_value());
                    if offer < *m {
                        *m = offer;
                        if p == to {
                            break 'outer;
                        }
                        q.push_back(p);
                        // println!("{:?}=>{:?} {:?}=>{:?}", from, mins.get(&p), p, to);
                    }
                }
            }
        }
        mins.get(&to).cloned().unwrap_or(usize::max_value())
    }

    fn find_attack_for_unit_at(&self, p: Point) -> Option<Point> {
        self.units
            .get(&p)
            .into_iter()
            .flat_map(|u| {
                p.adjacents()
                    .filter(|p2| self.units.get(p2).map(|u2| u2.is_foe(u)).unwrap_or(false))
                    .sorted()
            })
            .min_by_key(|p2| {
                self.units
                    .get(p2)
                    .map(|u2| u2.hp)
                    .unwrap_or(isize::max_value())
            })
    }

    fn find_target_for_unit_at(&self, p: Point) -> Option<Point> {
        self.units
            .get(&p)
            .into_iter()
            .flat_map(|u| {
                self.units.iter()
                    .filter(|(_, u2)| u.is_foe(u2))
                    .flat_map(|(l,_)| l.adjacents())
                    .sorted()
            })
            .filter(|p2| self.grid[p2.y][p2.x] && !self.units.contains_key(&p2))
            .filter(|pa| self.dist(p, *pa) < usize::max_value())
            .min_by(|pa, pb| self.dist(p, *pa).cmp(&self.dist(p, *pb)).then(pa.cmp(&pb)))
    }

    fn attack(&mut self, ap: Point, dp: Point) {
        let attack_amt = match self.units.get(&ap) {
            Some(u) if u.team == Goblin => AP,
            Some(u) if u.team == Elf => self.elf_ap,
            _ => unreachable!(),
        };
        self.units.get_mut(&dp).map(|u| u.hp -= attack_amt);
        if self.units.get(&dp).map(|u| u.hp <= 0).unwrap_or(false) {
            if self.units.get(&dp).map(|u| u.team == Elf).unwrap_or(false) {
                self.dead_elf_count += 1;
            }
            self.units.remove(&dp);
        }
    }

    fn move_unit(&mut self, from: Point, to: Point) {
        if let Some(u) = self.units.remove(&from) {
            self.units.insert(to, u);
        }
    }

    fn step_unit_at(&mut self, p: Point) {
        if let Some(att) = self.find_attack_for_unit_at(p) {
            self.attack(p, att);
        } else if let Some(tg) = self.find_target_for_unit_at(p) {
            let maybe_mv =  p.adjacents()
                .filter(|p2| self.grid[p2.y][p2.x] && !self.units.contains_key(&p2))
                .min_by(|ma, mb| self.dist(*ma, tg).cmp(&self.dist(*mb, tg)).then(ma.cmp(mb)));
            if let Some(mv) = maybe_mv {
                self.move_unit(p, mv);
                if let Some(att) = self.find_attack_for_unit_at(mv) {
                    self.attack(mv, att);
                }
            }
        }
    }

    fn if_done_get_ending_hp(&self) -> Option<isize> {
        let (gob_hp, elf_hp) = self.units.values()
            .fold((0, 0), |mut acc, u|
                if u.team == Elf {
                    (acc.0 + u.hp, acc.1)
                } else {
                    (acc.0, acc.1 + u.hp)
                });
        match (gob_hp, elf_hp) {
            (0, n) => Some(n),
            (n, 0) => Some(n),
            _ => None,
        }
    }

    fn step_units(&mut self) -> Option<isize> {
        let unit_locs = self.units.keys().cloned().sorted();
        for ul in unit_locs {
            self.step_unit_at(ul);
            if let Some(i) = self.if_done_get_ending_hp() {
                return Some(i)
            }
        }
        None
    }
}

impl Unit {
    fn print(&self) -> char {
        match self.team {
            Goblin => 'G',
            Elf => 'E',
        }
    }

    fn print_hp(&self) -> String {
        format!("{}({})", self.print(), self.hp)
    }

    fn is_foe(&self, other: &Unit) -> bool {
        self.team != other.team
    }
}

fn parse_grid(s: &str) -> Map {
    let mut r = [[false; SIZE]; SIZE];
    let mut units = HashMap::new();
    let mut len = SIZE;
    for (y, l) in s.lines().enumerate() {
        if l.len() > 0 {
            // Record the min width that is non-zero encountered
            len = len.min(l.len());
        }
        for (x, c) in l.chars().enumerate() {
            let p = Point { x: x, y: y };
            r[y][x] = match c {
                'G' => {
                    units.insert(
                        p,
                        Unit {
                            hp: START_HP,
                            team: Goblin,
                        },
                    );
                    true
                }
                'E' => {
                    units.insert(
                        p,
                        Unit {
                            hp: START_HP,
                            team: Elf,
                        },
                    );
                    true
                }
                '.' => true,
                _ => false,
            };
        }
    }
    Map {
        grid: r,
        units: units,
        real_size: len.max(s.lines().count()),
        elf_ap: AP,
        dead_elf_count: 0,
    }
}

fn run_to_completion(map: &mut Map) -> (isize, isize) {
    for i in 0.. {
        // println!("{}", map.print());
        if let Some(j) = map.step_units() {
            return (i, j)
        }
    }
    unreachable!()
}

pub fn part1(input: String) -> String {
    let mut map = parse_grid(&input);
    let (round, hp) = run_to_completion(&mut map);
    format!("{} * {} = {}", round, hp, round * hp)
}

pub fn part2(input: String) -> String {
    for ap in 4.. {
        let mut map = parse_grid(&input);
        map.elf_ap = ap;
        let (round, hp) = run_to_completion(&mut map);
        if map.dead_elf_count == 0 {
            return format!("{} * {} = {}", round, hp, round * hp)
        }
    }
    unreachable!()
}

const EXAMPLE: &str = r#"
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######
"#;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn dist_path() {
        let s = vec![".#.", "...", "#..", ".#.", ".#."]
            .into_iter()
            .join("\n");
        let map = parse_grid(&s);
        assert_eq!(4, map.dist(Point { x: 0, y: 0 }, Point { x: 2, y: 0 }));
        assert_eq!(
            usize::max_value(),
            map.dist(Point { x: 0, y: 0 }, Point { x: 0, y: 3 })
        );
        assert_eq!(6, map.dist(Point { x: 0, y: 0 }, Point { x: 2, y: 4 }))
    }

    #[test]
    pub fn attack_adjacent() {
        let s = vec!["G#G", "E.G"].into_iter().join("\n");
        let map = parse_grid(&s);
        assert_eq!(
            Some(Point { x: 0, y: 1 }),
            map.find_attack_for_unit_at(Point { x: 0, y: 0 })
        );
        assert_eq!(
            Some(Point { x: 0, y: 0 }),
            map.find_attack_for_unit_at(Point { x: 0, y: 1 })
        );

        assert_eq!(None, map.find_attack_for_unit_at(Point { x: 2, y: 1 }));
        assert_eq!(None, map.find_attack_for_unit_at(Point { x: 2, y: 0 }));
    }

    #[test]
    pub fn target_order() {
        let s = vec![
            "E.G.E", 
            "E...G",
        ].into_iter().join("\n");
        let map = parse_grid(&s);
        assert_eq!(
            Some(Point { x: 1, y: 0 }),
            map.find_target_for_unit_at(Point { x: 0, y: 0 })
        );

        // Reading order -- go left before we go right
        assert_eq!(
            Some(Point { x: 1, y: 0 }),
            map.find_target_for_unit_at(Point { x: 2, y: 0 })
        );

        // First in reading order so we go up before right
        assert_eq!(
            Some(Point { x: 1, y: 0 }),
            map.find_target_for_unit_at(Point { x: 0, y: 1 })
        );
    }
 
    #[test]
    pub fn step_simple() {
        let s = "#E..GE#";
        let mut map = parse_grid(&s);
        map.step_units();
        assert_eq!(
            "#.E.GE#  E(200) G(197) E(197)",
            map.print().lines().next().unwrap(),
        );
        map.step_units();
        assert_eq!(
            "#..EGE#  E(200) G(191) E(194)",
            map.print().lines().next().unwrap(),
        );
    }

    #[test]
    pub fn decisions() {
         let s = vec![
            "...G", 
            "E...",
        ].into_iter().join("\n");
        let mut map = parse_grid(&s);
        map.step_units();
        assert_eq!(
            vec![
                "E.G.  E(200) G(200)",
                "....  ",
                "####  ",
                "####  ",
            ].into_iter().join("\n"),
            map.print(),
        );
    }

const E1: &str = r#"
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######
"#;

const E2: &str = r#"
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######
"#;

const E3: &str = r#"
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########
"#;

    #[test]
    pub fn givens() {
        let mut map = parse_grid(E1);
        assert_eq!((35, 793), run_to_completion(&mut map));

        let mut map = parse_grid(E2);
        assert_eq!((37, 982), run_to_completion(&mut map));

        let mut map = parse_grid(E3);
        assert_eq!((20, 937), run_to_completion(&mut map));
    }
}