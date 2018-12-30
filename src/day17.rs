use itertools::*;
use chrono::{NaiveDateTime,Timelike};
use std::collections::HashSet;
use time::Duration;
use rayon::prelude::*;
use regex::*;
use std::mem;
use image::{Rgb, ImageBuffer};
use std::path::Path;

macro_rules! veci{
    ( $( $e:expr, if $b:expr),*$(,)* ) => ({
        vec![
            $(
                if $b {Some($e)} else {None}
            ),*
        ].into_iter()
        .flat_map(|a| a)
        .collect()
    })
}

// x=428, y=86..110
// y=1292, x=430..432
lazy_static! {
    static ref RE: Regex = Regex::new(r"(?P<axis>x|y)=(?P<pcoord>\d+), (x|y)=(?P<sstart>\d+)..(?P<send>\d+)").unwrap();
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

const SPRING: Point = Point {x: 500, y: 0};

struct World {
    clay: HashSet<Point>,
    water: HashSet<Point>,
    top: usize,
    bottom: usize,
}

impl World {
    fn to_string(&self) -> String {
        let min_x = self.clay.iter().min_by_key(|p| p.x).unwrap().x - 1;
        let max_x = self.clay.iter().max_by_key(|p| p.x).unwrap().x + 1;
        (self.top..=self.bottom).map(|y| {
            (min_x..=max_x)
                .map(|x| Point {x,y})
                .map(|p| if self.water.contains(&p) {
                    '~'
                } else if self.clay.contains(&p) {
                    '#'
                } else {
                    '.'
                })
                .collect::<String>()
        })
        .join("\n")
    }

    fn make_image(&self, path: &Path) {
        let min_x = self.clay.iter().chain(self.water.iter()).min_by_key(|p| p.x).unwrap().x - 1;
        let max_x = self.clay.iter().chain(self.water.iter()).max_by_key(|p| p.x).unwrap().x + 2;
        let width = (max_x - min_x + 10) as u32;
        let height = (self.bottom + 10) as u32;
        let mut img = ImageBuffer::from_pixel(width, height, Rgb([220, 220, 180]));
        for wp in &self.water {
            let x = (wp.x - min_x) as u32;
            let y = wp.y as u32;
            img.put_pixel(x, y, Rgb([0, 0, 200]));
        }
        for cp in &self.clay {
            let x = (cp.x - min_x) as u32;
            let y = cp.y as u32;
            img.put_pixel(x, y, Rgb([80, 70, 40]));
        }
        img.save(path);
    }

    fn is_clay_under(&self, p: Point) -> bool {
        self.clay.contains(&Point {
            y: p.y+1,
            ..p
        })
    }

    fn is_water_under(&self, p: Point) -> bool {
        self.water.contains(&Point {
            y: p.y+1,
            ..p
        })
    }

    fn will_fall(&self, p: Point) -> bool {
        !(self.is_clay_under(p) || self.is_water_under(p))
    }

    fn fill_rect(&mut self, tl: Point, br: Point) {
        let top = self.top;
        let bottom = self.bottom;
        self.water.extend(rect(tl, br)
                .filter(|p| p.y >= top && p.y <= bottom)
        );
    }

    // Return the furthest point in the given direction before a wall or fall
    // 2nd is true if the flow will drip
    fn get_fill_until_wall_or_drip(&self, p: Point, dir: isize) -> (Point, bool) {
        let mut next = p;
        loop {
            let next_next = Point { x: if dir > 0 {next.x+1} else {next.x-1}, ..next };
            if self.clay.contains(&next_next) {
                return (next, self.will_fall(next))
            } else if self.will_fall(next_next) && self.is_clay_under(next) {
                return (next_next, true)
            } else if self.will_fall(next_next) && self.is_water_under(next) {
                return (next, true)
            } else {
                next = next_next;
            }
        }
    }

    // p must already be water that is ready to fall
    fn fill_down_until_clay(&mut self, mut p: Point) -> Option<Point> {
        while !self.is_clay_under(p) {
            if p.y > self.bottom {
                return None
            }
            p = Point { y: p.y+1, ..p };
            if p.y >= self.top {
                self.water.insert(p);
            }
        }
        Some(p)
    }

    // Returns left/right spill points after filling bucket
    // p must be existing water
    fn fill_bucket(&mut self, mut p: Point) -> Vec<Point> {
        loop {
            let (left, ldrips) = self.get_fill_until_wall_or_drip(p, -1);
            let (right, rdrips) = self.get_fill_until_wall_or_drip(p, 1);
            self.fill_rect(left, right);
            if ldrips || rdrips {
                return veci![
                    left, if ldrips,
                    right, if rdrips,
                ]
            }
            p = Point {y: p.y-1, ..p};
        }
    }
}

fn parse_lines(s: &str) -> World {
    let mut top = usize::max_value();
    let mut bottom = 0;
    let mut clay = HashSet::new();
    for l in s.lines() {
        if let Some(cap) = RE.captures(l) {
            let pcoord = cap["pcoord"].parse::<usize>().unwrap();
            let sstart = cap["sstart"].parse::<usize>().unwrap();
            let send = cap["send"].parse::<usize>().unwrap();
            clay.extend(
                (sstart..=send)
                    .map(|b| (pcoord, b))
                    .map(|(a,b)| if &cap["axis"] == "x" {(a,b)} else {(b,a)})
                    .map(|(x,y)| Point{x, y})
                    .inspect(|p| top = top.min(p.y))
                    .inspect(|p| bottom = bottom.max(p.y))
            );
        }
    }
    World {
        water: HashSet::new(),
        clay: clay,
        top: top,
        bottom: bottom,
    }
}

fn rect(tl: Point, br: Point) -> impl Iterator<Item=Point> {
    (tl.x..=br.x)
        .cartesian_product((tl.y..=br.y))
        .map(|(x,y)| Point { x, y })
}

fn run_simulation(world: &mut World) {
    let mut filling = HashSet::new();
    let mut falling = HashSet::new();
    falling.insert(SPRING);
    let mut i = 0;
    while filling.len() > 0 || falling.len() > 0 {
        let next_filling = falling.into_iter()
            .flat_map(|tl| world.fill_down_until_clay(tl))
            .collect();
        let next_falling = filling.into_iter()
            .flat_map(|p| world.fill_bucket(p))
            .collect();
        filling = next_filling;
        falling = next_falling;
        i += 1;
    }
    // world.make_image(&Path::new(&format!("target/tmp/{}.png", i)));
}

pub fn part1(input: String) -> String {
    let mut world = parse_lines(&input);
    run_simulation(&mut world);
    world.water.iter()
        .filter(|p| p.y <= world.bottom && p.y >= world.top)
        .count()
        .to_string()
}

pub fn part2(input: String) -> String {
    let mut world = parse_lines(&input);
    run_simulation(&mut world);
    world.water.iter()
        .filter(|p| p.y <= world.bottom && p.y >= world.top)
        .filter(|&p| {
            // Don't take ones which drip
            let (_, ldrips) = world.get_fill_until_wall_or_drip(*p, -1);
            let (_, rdrips) = world.get_fill_until_wall_or_drip(*p, 1);
            !ldrips && !rdrips
        })
        .count()
        .to_string()
}

const EXAMPLE: &str = r#"
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504
"#;