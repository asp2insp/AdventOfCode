use aoc::utils::*;
use fnv::FnvHashSet;

#[derive(Debug)]
struct Bot {
    p: Point,
    v: Point,
}

impl Bot {
    fn from(nums: Vec<isize>) -> Bot {
        Bot {
            p: Point::new(nums[0], nums[1]),
            v: Point::new(nums[2], nums[3]),
        }
    }

    fn step(&mut self, (width, height): (isize, isize)) {
        self.p.x = (self.p.x + self.v.x).rem_euclid(width);
        self.p.y = (self.p.y + self.v.y).rem_euclid(height);
    }

    fn step_n(&mut self, n: isize, (width, height): (isize, isize)) {
        self.p.x = (self.p.x + self.v.x * n).rem_euclid(width);
        self.p.y = (self.p.y + self.v.y * n).rem_euclid(height);
    }
}

pub fn part1(input: String) -> String {
    const WIDTH: isize = 101;
    const HEIGHT: isize = 103;
    let mut bots = gimme_nums(&input)
        .into_iter()
        .map(Bot::from)
        .collect::<Vec<_>>();
    for b in &mut bots {
        b.step_n(100, (WIDTH, HEIGHT));
    }
    safety_factor(&bots, (WIDTH, HEIGHT)).to_string()
}

fn safety_factor(bots: &Vec<Bot>, (width, height): (isize, isize)) -> usize {
    let mut quad_counts = vec![0; 4];
    for b in bots {
        match (b.p.x, b.p.y) {
            (x, y) if x < width / 2 && y < height / 2 => quad_counts[0] += 1,
            (x, y) if x > width / 2 && y < height / 2 => quad_counts[1] += 1,
            (x, y) if x < width / 2 && y > height / 2 => quad_counts[2] += 1,
            (x, y) if x > width / 2 && y > height / 2 => quad_counts[3] += 1,
            _ => (),
        }
    }
    quad_counts.into_iter().product()
}

pub fn part2(input: String) -> String {
    const WIDTH: isize = 101;
    const HEIGHT: isize = 103;
    let mut bots = gimme_nums(&input)
        .into_iter()
        .map(Bot::from)
        .collect::<Vec<_>>();
    let mut steps = 0;
    loop {
        let locs = bots.iter().map(|b| b.p).collect::<FnvHashSet<_>>();
        if locs.len() == bots.len() {
            println!(
                "Found it! Done after {} steps, when {} bots are in {} locs",
                steps,
                bots.len(),
                locs.len()
            );
            break;
        }
        for b in &mut bots {
            b.step((WIDTH, HEIGHT));
        }
        steps += 1;
        // print_progress(steps, 1000);
    }
    steps.to_string()
}

#[test]
fn test_bot() {
    let mut b = Bot {
        p: Point::new(0, 0),
        v: Point::new(1, 1),
    };
    b.step((11, 7));
    assert_eq!(b.p, Point::new(1, 1));
    b.v = Point::new(-1, -1);
    b.step((11, 7));
    assert_eq!(b.p, Point::new(0, 0));
    b.step((11, 7));
    assert_eq!(b.p, Point::new(10, 6));
}

#[test]
fn test_small() {
    const WIDTH: isize = 11;
    const HEIGHT: isize = 7;
    let s = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    let mut bots = gimme_nums(s).into_iter().map(Bot::from).collect::<Vec<_>>();
    for _ in 0..100 {
        for b in &mut bots {
            b.step((WIDTH, HEIGHT));
        }
    }
    assert_eq!(12, safety_factor(&bots, (WIDTH, HEIGHT)));
}
