use aoc::utils::*;
use itertools::Itertools;
use std::collections::HashSet;

fn expand(g: &Grid<()>) -> Grid<()> {
    let mut gnew = g.clone();
    // Expand by rows first
    let row_targets = (g.bottom_bound..=g.top_bound)
        .filter(|r| {
            g.iter_range(Some(g.left_bound..=g.right_bound), Some(*r..=*r))
                .all(|(_p, c, _t)| c == '.')
        })
        .collect_vec();
    for row in row_targets.into_iter().rev() {
        gnew.insert_row(row, |_p| ('.', ()));
    }
    // Then columns
    let col_targets = (g.left_bound..=g.right_bound)
        .filter(|c| {
            g.iter_range(Some(*c..=*c), Some(g.bottom_bound..=g.top_bound))
                .all(|(_p, c, _t)| c == '.')
        })
        .collect_vec();
    for col in col_targets.into_iter().rev() {
        gnew.insert_col(col, |_p| ('.', ()));
    }
    return gnew;
}

pub fn part1(input: String) -> String {
    let g = Grid::new(&input, ());
    let g2 = expand(&g);
    let stars = g2
        .iter_chars()
        .filter(|&(_, c)| c == '#')
        .map(|(p, _)| p)
        .collect_vec();
    stars
        .into_iter()
        .combinations(2)
        .map(|v| v[0].dist(&v[1]))
        .sum::<isize>()
        .to_string()
}

fn dist_with_interspersion(
    a: &Point,
    b: &Point,
    rows: &HashSet<isize>,
    cols: &HashSet<isize>,
    interspersion: isize,
) -> isize {
    (a.x.min(b.x)..a.x.max(b.x))
        .map(|x| {
            if cols.contains(&x) {
                interspersion
            } else {
                1
            }
        })
        .sum::<isize>()
        + (a.y.min(b.y)..a.y.max(b.y))
            .map(|y| {
                if rows.contains(&y) {
                    interspersion
                } else {
                    1
                }
            })
            .sum::<isize>()
}

pub fn part2(input: String) -> String {
    let g = Grid::new(&input, ());
    let stars = g
        .iter_chars()
        .filter(|&(_, c)| c == '#')
        .map(|(p, _)| p)
        .collect_vec();
    let col_targets = (g.left_bound..=g.right_bound)
        .filter(|c| {
            g.iter_range(Some(*c..=*c), Some(g.bottom_bound..=g.top_bound))
                .all(|(_p, c, _t)| c == '.')
        })
        .collect::<HashSet<isize>>();
    let row_targets = (g.bottom_bound..=g.top_bound)
        .filter(|r| {
            g.iter_range(Some(g.left_bound..=g.right_bound), Some(*r..=*r))
                .all(|(_p, c, _t)| c == '.')
        })
        .collect::<HashSet<isize>>();
    stars
        .into_iter()
        .combinations(2)
        .map(|v| dist_with_interspersion(&v[0], &v[1], &row_targets, &col_targets, 1000000))
        .sum::<isize>()
        .to_string()
}
