use aoc::utils::*;
use itertools::Itertools;
use rayon::prelude::*;
use take_until::TakeUntilExt;

pub fn part1(input: String) -> String {
    let mut g = Grid::new(&input, false);
    let (l, b, r, t) = g.get_bounds();

    // Bottom Up
    mark_visible(&mut g, l..=r, Direction::N);
    // Top Down
    mark_visible(&mut g, l..=r, Direction::S);
    // West to East
    mark_visible(&mut g, b..=t, Direction::E);
    // East to West
    mark_visible(&mut g, b..=t, Direction::W);

    // println!("{}", g.to_string());
    // println!("{:?}", g.iter_contents().collect_vec());
    g.iter_contents().filter(|x| *x.1).count().to_string()
}

fn mark_visible(g: &mut Grid<bool>, outer: impl IntoIterator<Item = isize>, dir: Direction) {
    let (l, b, r, t) = g.get_bounds();
    for op in outer {
        let mut p = match dir {
            Direction::N => Point::new(op, b),
            Direction::S => Point::new(op, t),
            Direction::E => Point::new(l, op),
            Direction::W => Point::new(r, op),
        };
        let mut highest = '/';
        loop {
            let c = g.read_pt(&p);
            if c > highest {
                highest = c;
                g.get_mut(p).unwrap().1 = true;
            }
            if let Some(np) = g.drive(p, dir) {
                p = np;
            } else {
                break;
            }
        }
    }
}

pub fn part2(input: String) -> String {
    let g = Grid::new(&input, false);
    g.iter_range(None, None)
        .collect_vec()
        .into_par_iter()
        .map(|(p, c, _)| {
            DIRECTIONS
                .map(|dir| g.drive_iter(p, dir).take_until(|np| g.read_pt(np) >= c).count())
                .into_iter()
                .product::<usize>()
        })
        .max()
        .unwrap()
        .to_string()
}

#[test]
fn test_1() {
    let s = r#"30373
 25512
 65332
 33549
 35390"#
        .to_owned();
    assert_eq!("21", part1(s));
}
