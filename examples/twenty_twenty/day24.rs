use fnv::FnvHashMap;
use itertools::Itertools;
use aoc::utils::*;
use aoc::dict;

enum Dir {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Dir {
    fn from_str(s: &str) -> (Self, &str) {
        // Match the first one or two chars and return remainder
        match s.chars().next() {
            Some('e') => (Self::East, &s[1..]),
            Some('w') => (Self::West, &s[1..]),
            Some('s') => match s.chars().nth(1) {
                Some('e') => (Self::SouthEast, &s[2..]),
                Some('w') => (Self::SouthWest, &s[2..]),
                _ => panic!("Invalid direction"),
            },
            Some('n') => match s.chars().nth(1) {
                Some('e') => (Self::NorthEast, &s[2..]),
                Some('w') => (Self::NorthWest, &s[2..]),
                _ => panic!("Invalid direction"),
            },
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct H3 {
    q: isize,
    r: isize,
    s: isize,
}

impl H3 {
    fn new(q: isize, r: isize, s: isize) -> Self {
        Self { q, r, s }
    }

    fn drive(&self, d: Dir) -> Self {
        match d {
            Dir::East => Self::new(self.q + 1, self.r, self.s - 1),
            Dir::West => Self::new(self.q - 1, self.r, self.s + 1),
            Dir::SouthEast => Self::new(self.q, self.r + 1, self.s - 1),
            Dir::SouthWest => Self::new(self.q - 1, self.r + 1, self.s),
            Dir::NorthWest => Self::new(self.q, self.r - 1, self.s + 1),
            Dir::NorthEast => Self::new(self.q + 1, self.r - 1, self.s),
        }
    }

    fn neighbors(&self) -> Vec<Self> {
        vec![Dir::East, Dir::West, Dir::SouthEast, Dir::SouthWest, Dir::NorthWest, Dir::NorthEast]
            .into_iter()
            .map(|d| self.drive(d))
            .collect_vec()
    }

    fn count_black_neighbors(&self, m: &FnvHashMap<Self, bool>) -> usize {
        self.neighbors().iter().filter(|p| m.get(p).copied().unwrap_or(false)).count()
    }

    fn flip(&self, m: &FnvHashMap<Self, bool>) -> bool {
        let black_neighbors = self.count_black_neighbors(m);
        if m.get(self).copied().unwrap_or(false) {
            // self is black and 0 or > 2 neighbors, we flip to white otherwise stay
            if black_neighbors == 0 || black_neighbors > 2 {
                false
            } else {
                true
            }
        } else {
            // self is white, if 2 neighbors are black, flip to black
            if black_neighbors == 2 {
                true
            } else {
                false
            }
        }
    }
}

fn build_floor(s: &str) -> FnvHashMap<H3, bool> {
    let tiles = s.lines()
        .map(|mut l| {
            let mut v = vec![];
            while !l.is_empty() {
                let (d, r) = Dir::from_str(l);
                v.push(d);
                l = r;
            }
            v
        })
        .collect_vec();
    let mut m = dict!();
    for tpath in tiles {
        let mut p = H3::new(0, 0, 0);
        for d in tpath {
            p = p.drive(d);
        }
        if !m.contains_key(&p) {
            m.insert(p, true);
        } else {
            m.insert(p, !m[&p]);
        }
    }
    m
}

pub fn part1(s: String) -> String {
    let m = build_floor(&s);
    m.values().filter(|&v| *v).count().to_string()
}


pub fn part2(s: String) -> String {
    let mut m = build_floor(&s);
    for _ in 0..100 {
        let mut m2 = dict!();
        let bounds = m.keys().fold((0, 0, 0, 0, 0, 0), |(minq, maxq, minr, maxr, mins, maxs), p| {
            (minq.min(p.q), maxq.max(p.q), minr.min(p.r), maxr.max(p.r), mins.min(p.s), maxs.max(p.s))
        });
        for q in bounds.0 - 2..bounds.1 + 2 {
            for r in bounds.2 - 2..bounds.3 + 2{
                for s in bounds.4 - 2..bounds.5 + 2{
                    let p = H3::new(q, r, s);
                    if p.flip(&m) {
                        m2.insert(p, true);
                    }
                }
            }
        }
        m = m2;
    }
    m.values().filter(|&v| *v).count().to_string()
}


#[test]
fn test() {
    let s = r"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
    assert_eq!("10", part1(s.to_string()));
    assert_eq!("2208", part2(s.to_string()));
}