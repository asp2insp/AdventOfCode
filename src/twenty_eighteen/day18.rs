use std::collections::HashMap;

const SIZE: usize = 50;

struct World {
    map: [[char; SIZE]; SIZE],
}

impl World {
    fn from_str(s: &str) -> World {
        let mut m = [['.'; SIZE]; SIZE];
        s.chars()
            .filter(|c| !c.is_whitespace())
            .enumerate()
            .for_each(|(i, c)| m[i/SIZE][i%SIZE] = c);
        World {map: m}
    }

    fn to_string(&self) -> String {
        self.map.iter()
        .flat_map(|r| r.iter())
        .collect()
    }

    // returns (trees, lumberyards, clearground)
    fn adjacents(&self, x: usize, y: usize) -> (usize, usize, usize) {
        // 0 1 2
        // 3 4 5
        // 6 7 8
        let v: Vec<char> = veci![
            // 0 1 2
            self.map[y-1][x-1], if x > 0 && y > 0,
            self.map[y-1][x],   if y > 0,
            self.map[y-1][x+1], if x < SIZE-1 && y > 0,

            // 3 4 5 
            self.map[y][x-1], if x > 0,
            // self is not adjacent to self
            self.map[y][x+1], if x < SIZE-1,
            
            // 6 7 8
            self.map[y+1][x-1], if x > 0 && y < SIZE-1,
            self.map[y+1][x],   if y < SIZE-1,
            self.map[y+1][x+1], if x < SIZE-1 && y < SIZE-1,
        ];
        v.into_iter()
        .fold((0,0,0), |mut acc, n| {
            match n {
                '|' => acc.0 += 1,
                '#' => acc.1 += 1,
                '.' => acc.2 += 1,
                _ => panic!("Unknown char {}", n),
            };
            acc
        })
    }

    fn step(self) -> World {
        let mut m = [['0'; SIZE]; SIZE];
        for y in 0..SIZE {
            for x in 0..SIZE {
                m[y][x] = match (self.map[y][x], self.adjacents(x, y)) {
                    ('.', (t, _, _)) if t >= 3 => '|',
                    ('|', (_, l, _)) if l >= 3 => '#',
                    ('#', (t, l, _)) if t >= 1 && l >= 1 => '#',
                    ('#', (_, _, _)) => '.',
                    (c, _) => c,
                }
            }
        }
        World {
            map: m,
        }
    }
}

pub fn part1(input: String) -> String {
    let mut map = World::from_str(&input);
    for i in 0..10 {
        map = map.step();
    }
    let counts = map.map.iter()
        .flat_map(|r| r.iter())
        .fold((0, 0), |acc, n| match n {
            '|' => (acc.0 + 1, acc.1),
            '#' => (acc.0, acc.1+1),
            _ => acc,
        });
    (counts.0 * counts.1).to_string()
}

pub fn part2(input: String) -> String {
    let mut seen = HashMap::new();
    let mut map = World::from_str(&input);
    let mut cycle = 0;
    while !seen.contains_key(&map.to_string()) {
        seen.insert(map.to_string(), cycle);
        map = map.step();
        cycle += 1;
    }
    let len = cycle - seen.get(&map.to_string()).unwrap();
    println!("Found cycle after {} minutes of length {}", cycle, len);
    for _ in 0..((1_000_000_000-cycle) % len) {
        map = map.step();
    }
    let counts = map.map.iter()
        .flat_map(|r| r.iter())
        .fold((0, 0), |acc, n| match n {
            '|' => (acc.0 + 1, acc.1),
            '#' => (acc.0, acc.1+1),
            _ => acc,
        });
    (counts.0 * counts.1).to_string() 
}