use itertools::*;
use std::any::TypeId;
use std::cell::Cell;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::ops::RangeInclusive;
use std::str::FromStr;

pub trait IterUtils: Iterator {
    fn counting_set(self) -> HashMap<Self::Item, usize>
    where
        Self: Sized,
        Self::Item: Clone + Eq + Hash,
    {
        self.fold(HashMap::new(), |mut map, it| {
            *map.entry(it).or_insert(0) += 1;
            map
        })
    }
}

impl<T: ?Sized> IterUtils for T where T: Iterator {}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    pub fn from_char(c: char) -> Result<Direction, ()> {
        use Direction::*;

        match c {
            'N' | 'n' => Ok(N),
            'S' | 's' => Ok(S),
            'E' | 'e' => Ok(E),
            'W' | 'w' => Ok(W),
            _ => Err(()),
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        if s.len() != 1 {
            return Err(());
        }
        Direction::from_char(s.chars().next().ok_or(())?)
    }
}

pub struct Grid<T> {
    map: HashMap<(isize, isize), (char, T)>,
    pub wall_char: char,
    pub floor_char: char,
    pub default_fn: Option<Box<dyn Fn(isize, isize) -> (char, T)>>,
    pub left_bound: isize,
    pub right_bound: isize,
    pub top_bound: isize,
    pub bottom_bound: isize,
}

impl<T> Default for Grid<T> {
    fn default() -> Grid<T> {
        Grid {
            map: HashMap::new(),
            wall_char: '\0',
            floor_char: '\0',
            default_fn: None,
            left_bound: 0,
            right_bound: 0,
            top_bound: 0,
            bottom_bound: 0,
        }
    }
}

impl<T> Grid<T> {
    pub fn new(s: &str, t: T) -> Grid<T>
    where
        T: Clone,
    {
        Self::new_with(s, |_| t.clone())
    }

    pub fn new_with(s: &str, f: impl Fn(char) -> T) -> Grid<T> {
        let mut m = HashMap::new();
        for (li, l) in s.lines().rev().enumerate() {
            for (ci, c) in l.chars().enumerate() {
                m.insert((ci as isize, li as isize), (c, f(c)));
            }
        }
        let mut g = Grid {
            map: m,
            ..Default::default()
        };
        let (l, b, r, t) = g.get_bounds();
        Grid {
            left_bound: l,
            bottom_bound: b,
            right_bound: r,
            top_bound: t,
            ..g
        }
    }

    pub fn new_with_bounds(
        left: isize,
        bottom: isize,
        right: isize,
        top: isize,
        f: impl Fn(isize, isize) -> (char, T),
    ) -> Grid<T> {
        let mut x = left;
        let mut y = bottom;
        let mut m = HashMap::new();
        while x <= right {
            while y <= top {
                m.insert((y, x), f(x, y));
                y += 1;
            }
            x += 1;
            y = bottom;
        }
        Grid {
            map: m,
            left_bound: left,
            right_bound: right,
            top_bound: top,
            bottom_bound: bottom,
            ..Default::default()
        }
    }

    pub fn has_walls(&self) -> bool {
        self.wall_char != '\0'
    }

    pub fn has_bounds(&self) -> bool {
        self.left_bound != self.right_bound || self.top_bound != self.bottom_bound
    }

    pub fn in_bounds(&self, x: isize, y: isize) -> bool {
        if !self.has_bounds() {
            true
        } else {
            self.left_bound <= x
                && self.right_bound >= x
                && self.bottom_bound <= y
                && self.top_bound >= y
        }
    }

    // left, bottom, right, top
    pub fn get_bounds(&self) -> (isize, isize, isize, isize) {
        if self.has_bounds() {
            (
                self.left_bound,
                self.bottom_bound,
                self.right_bound,
                self.top_bound,
            )
        } else {
            let mut left = isize::MAX;
            let mut bottom = isize::MAX;
            let mut right = isize::MIN;
            let mut top = isize::MIN;

            for (x, y) in self.map.keys() {
                left = min(left, *x);
                right = max(right, *x);
                bottom = min(bottom, *y);
                top = max(top, *y);
            }
            (left, bottom, right, top)
        }
    }

    pub fn get(&self, x: isize, y: isize) -> Option<&(char, T)> {
        if !self.in_bounds(x, y) {
            None
        }
        // else if (!self.map.contains_key(&(x, y))) {
        //     self.default_fn.map(|f| f(x, y))
        // }
        else {
            self.map.get(&(x, y))
        }
    }

    pub fn set(&mut self, x: isize, y: isize, c: char, t: T) {
        if self.in_bounds(x, y) {
            self.map.insert((x, y), (c, t));
        }
    }

    pub fn iter_contents<'a>(&'a self) -> impl Iterator<Item = (isize, isize, &'a T)> {
        self.map.iter().map(|(xy, ct)| (xy.0, xy.1, &ct.1))
    }

    pub fn iter_chars(&self) -> impl Iterator<Item = (isize, isize, char)> {
        self.map
            .iter()
            .map(|(xy, ct)| (xy.0, xy.1, ct.0))
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn fill_with(
        &mut self,
        xrange: Option<RangeInclusive<isize>>,
        yrange: Option<RangeInclusive<isize>>,
        f: impl Fn(isize, isize) -> (char, T),
    ) {
        let (l, bt, r, tp) = self.get_bounds();
        let xrange = xrange.unwrap_or(l..=r);
        let yrange = yrange.unwrap_or(bt..=tp);
        xrange.cartesian_product(yrange).for_each(|(x, y)| {
            let (c, t) = f(x, y);
            self.set(x, y, c, t);
        });
    }

    pub fn iter_range(
        &self,
        xrange: Option<RangeInclusive<isize>>,
        yrange: Option<RangeInclusive<isize>>,
    ) -> impl Iterator<Item = (isize, isize, char, &T)> {
        let (l, bt, r, tp) = self.get_bounds();
        let xrange = xrange.unwrap_or(l..=r);
        let yrange = yrange.unwrap_or(bt..=tp);
        xrange
            .cartesian_product(yrange)
            .filter_map(|(x, y)| self.get(x, y).map(|ct| (x, y, ct.0, &ct.1)))
    }

    pub fn is_wall(&self, x: isize, y: isize) -> bool {
        self.has_walls()
            && self
                .get(x, y)
                .map(|ct| ct.0 == self.wall_char)
                .unwrap_or(false)
    }

    pub fn drive(&self, x: isize, y: isize, d: Direction) -> Option<(isize, isize)> {
        use Direction::*;

        let mut xnew = x;
        let mut ynew = y;
        match d {
            N => ynew += 1,
            S => ynew -= 1,
            E => xnew += 1,
            W => xnew -= 1,
        };
        if self.in_bounds(xnew, ynew) && !self.is_wall(xnew, ynew) {
            Some((xnew, ynew))
        } else {
            None
        }
    }

    pub fn neighbors(&self, x: isize, y: isize) -> impl Iterator<Item = (isize, isize)> {
        use Direction::*;

        vec![
            self.drive(x, y, N),
            self.drive(x, y, S),
            self.drive(x, y, E),
            self.drive(x, y, W),
        ]
        .into_iter()
        .filter_map(|n| n)
    }

    pub fn dfs_path(
        &self,
        pt1: (isize, isize),
        pt2: (isize, isize),
        weight: Option<impl Fn(isize, isize) -> isize>,
    ) -> (isize, Vec<(isize, isize)>) {
        let mut results = self.dfs_path_bulk(pt1, makeset! {pt2}, weight);
        results.remove(&pt2).unwrap()
    }

    pub fn dfs_path_bulk(
        &self,
        pt1: (isize, isize),
        dests: HashSet<(isize, isize)>,
        weight: Option<impl Fn(isize, isize) -> isize>,
    ) -> HashMap<(isize, isize), (isize, Vec<(isize, isize)>)> {
        // Maps point => cost, parent_point
        let mut seen: HashMap<(isize, isize), (isize, (isize, isize))> = HashMap::new();
        let mut results = HashMap::new();
        let mut stack = Vec::new();
        stack.push((pt1, 0));
        loop {
            let (curr, cost) = stack.pop().expect(&format!("Dead end! found {} out of {}", results.len(), dests.len()));
            if dests.contains(&curr) && !results.contains_key(&curr) {
                let mut traceback = curr;
                let mut ret_path = vec![traceback];
                while traceback != pt1 {
                    traceback = seen.get(&traceback).unwrap().1;
                    ret_path.push(traceback);
                }
                results.insert(curr, (cost, ret_path));
                if results.len() == dests.len() {
                    return results;
                }
            }
            // enqueue stepwise costs from this to all neighbors
            for (nx, ny) in self.neighbors(curr.0, curr.1) {
                let w = weight.as_ref().map(|f| f(nx, ny)).unwrap_or(1);
                if w == isize::MAX {
                    seen.insert((nx, ny), (isize::MAX, curr));
                    continue;
                }
                // If we've already seen a lower cost, skip this one
                if seen
                    .get(&(nx, ny))
                    .map(|(old, _)| *old <= cost + w)
                    .unwrap_or(false)
                {
                    continue;
                }
                seen.insert((nx, ny), (cost + w, curr));
                stack.push(((nx, ny), cost + w));
            }
        }
    }
}

impl<T> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (l, b, r, t) = self.get_bounds();
        for line_no in (b..=t).rev() {
            for col_no in l..=r {
                write!(
                    f,
                    "{}",
                    self.get(col_no, line_no).map(|ct| ct.0).unwrap_or(' ')
                )?;
            }
            if line_no != b {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EX1: &'static str = r"#############
#.|.|.|.|.|.#
#-#####-###-#
#.#.|.#.#.#.#
#-#-###-#-#-#
#.#.#.|.#.|.#
#-#-#-#####-#
#.#.#.#X|.#.#
#-#-#-###-#-#
#.|.#.|.#.#.#
###-#-###-#-#
#.|.#.|.|.#.#
#############";

    #[test]
    fn test_round_trip() {
        let actual = Grid::new(&EX1, ()).to_string();
        println!("{}", actual);
        assert_eq!(EX1, actual);
    }

    #[test]
    fn test_dfs() {
        let mut g = Grid::new(&EX1, ());
        g.wall_char = '#';
        let cost_fn = |x, y| {
            g.get(x, y)
                .map(|ct| if ct.0 == '|' || ct.0 == '-' { 1 } else { 0 })
                .unwrap_or(0)
        };
        let (path_cost, path) = g.dfs_path((1, 1), (3, 7), Some(&cost_fn));
        assert_eq!(
            vec![
                (3, 7),
                (3, 6),
                (3, 5),
                (3, 4),
                (3, 3),
                (3, 2),
                (3, 1),
                (2, 1),
                (1, 1)
            ],
            path
        );
        assert_eq!(4, path_cost);
        assert_eq!(9, g.dfs_path((1, 11), (9, 9), Some(&cost_fn)).0);
    }
}
