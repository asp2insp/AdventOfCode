use itertools::*;
use std::any::TypeId;
use std::cell::Cell;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
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

    fn counting_set_by<T: Hash + Eq, F: Fn(Self::Item) -> T>(self, f: F) -> HashMap<T, usize>
    where
        Self: Sized,
    {
        self.fold(HashMap::new(), |mut map, it| {
            let key = f(it);
            *map.entry(key).or_insert(0) += 1;
            map
        })
    }
}

pub fn add_counting_sets<T: Hash + Eq>(a: HashMap<T, usize>, mut b: HashMap<T, usize>) -> HashMap<T, usize> {
    for (k, v) in a.into_iter() {
        *b.entry(k).or_insert(0) += v;
    }
    b
}

pub fn gimme_nums(s: &str) -> Vec<Vec<isize>> {
    use regex::*;
        let re = Regex::new(r"([-\d]+)([^-\d]*)").unwrap();
        return s.lines().map(|l| {
            re.captures_iter(l.trim()).map(|c| parse!(c[1], isize)).collect::<Vec<isize>>()
        }).collect::<Vec<Vec<isize>>>()
}

fn free_neighbors_bounded(p: Point, bounds: Option<(isize, isize, isize, isize)>) -> Vec<Point> {
    veci![
        Point {x: p.x - 1, y: p.y}, if bounds.map(|b| p.x > b.0).unwrap_or(true),
        Point {x: p.x + 1, y: p.y}, if bounds.map(|b| p.x < b.2).unwrap_or(true),
        Point {x: p.x , y: p.y - 1}, if bounds.map(|b| p.y > b.1).unwrap_or(true),
        Point {x: p.x, y: p.y + 1}, if bounds.map(|b| p.y < b.3).unwrap_or(true),
    ]
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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl From<(isize, isize)> for Point {
    fn from(tup: (isize, isize)) -> Point {
        Point {
            x: tup.0,
            y: tup.1,
        }
    }
}

impl From<(usize, usize)> for Point {
    fn from(tup: (usize, usize)) -> Point {
        Point {
            x: tup.0 as isize,
            y: tup.1 as isize,
        }
    }
}

impl From<(i32, i32)> for Point {
    fn from(tup: (i32, i32)) -> Point {
        Point {
            x: tup.0 as isize,
            y: tup.1 as isize,
        }
    }
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Point {x, y}
    }

    pub fn offset(&self, offsets: (isize, isize)) -> Self {
        Point {
            x: self.x + offsets.0,
            y: self.y + offsets.1,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Grid<T> {
    map: HashMap<Point, (char, T)>,
    pub wall_char: char,
    pub floor_char: char,
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
            for (ci, c) in l.trim().chars().enumerate() {
                m.insert(Point::from((ci, li)), (c, f(c)));
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
        f: impl Fn(Point) -> (char, T),
    ) -> Grid<T> {
        let mut x = left;
        let mut y = bottom;
        let mut m = HashMap::new();
        while x <= right {
            while y <= top {
                let p = Point::from((x, y));
                m.insert(p, f(p));
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
    
    pub fn add_other(&mut self, other: &Grid<T>, d: Direction) where T: Clone {
        let offsets = match d {
            Direction::W => (-(other.right_bound+1) + self.left_bound, 0),
            Direction::N => (0, (self.top_bound + 1) - other.bottom_bound),
            Direction::E => ((self.right_bound+1) - other.left_bound, 0),
            Direction::S => (0, -(other.top_bound+1) + self.bottom_bound),
        };
        self.left_bound = 0;
        self.right_bound = 0;
        self.top_bound = 0;
        self.bottom_bound = 0;
        for (p, c, t) in other.iter_range(None, None) {
            self.set(p.offset(offsets), c, t.clone());
        }
        let bounds = self.calc_bounds();
        self.left_bound = bounds.0;
        self.bottom_bound = bounds.1;
        self.right_bound = bounds.2;
        self.top_bound = bounds.3;
    }

    pub fn width(&self) -> isize {
        1 + self.right_bound - self.left_bound
    }

    pub fn height(&self) -> isize {
        1 + self.top_bound - self.bottom_bound
    }

    pub fn has_walls(&self) -> bool {
        self.wall_char != '\0'
    }

    pub fn has_bounds(&self) -> bool {
        self.left_bound != self.right_bound || self.top_bound != self.bottom_bound
    }

    pub fn in_bounds(&self, p: Point) -> bool {
        if !self.has_bounds() {
            true
        } else {
            let Point{x, y} = p;
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
            self.calc_bounds()
        }
    }

    pub fn calc_bounds(&self) -> (isize, isize, isize, isize) {
        let mut left = isize::MAX;
        let mut bottom = isize::MAX;
        let mut right = isize::MIN;
        let mut top = isize::MIN;

        for Point{x, y} in self.map.keys() {
            left = min(left, *x);
            right = max(right, *x);
            bottom = min(bottom, *y);
            top = max(top, *y);
        }
        (left, bottom, right, top)
    }

    pub fn get(&self, p: Point) -> Option<&(char, T)> {
        if !self.in_bounds(p) {
            None
        }
        // else if (!self.map.contains_key(&(x, y))) {
        //     self.default_fn.map(|f| f(x, y))
        // }
        else {
            self.map.get(&p)
        }
    }

    pub fn get_mut(&mut self, p: Point) -> Option<&mut (char, T)> {
        if !self.in_bounds(p) {
            None
        }
        else {
            self.map.get_mut(&p)
        }
    }

    pub fn set(&mut self, p: Point, c: char, t: T) {
        if self.in_bounds(p) {
            self.map.insert(p, (c, t));
        }
    }

    pub fn iter_contents<'a>(&'a self) -> impl Iterator<Item = (Point, &'a T)> {
        self.map.iter().map(|(xy, ct)| (*xy, &ct.1))
    }

    pub fn iter_chars(&self) -> impl Iterator<Item = (Point, char)> {
        self.map
            .iter()
            .map(|(xy, ct)| (*xy, ct.0))
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn fill_with(
        &mut self,
        xrange: Option<RangeInclusive<isize>>,
        yrange: Option<RangeInclusive<isize>>,
        f: impl Fn(Point) -> (char, T),
    ) {
        let (l, bt, r, tp) = self.get_bounds();
        let xrange = xrange.unwrap_or(l..=r);
        let yrange = yrange.unwrap_or(bt..=tp);
        xrange.cartesian_product(yrange).for_each(|(x, y)| {
            let p = Point::from((x, y));
            let (c, t) = f(p);
            self.set(p, c, t);
        });
    }

    pub fn iter_range(
        &self,
        xrange: Option<RangeInclusive<isize>>,
        yrange: Option<RangeInclusive<isize>>,
    ) -> impl Iterator<Item = (Point, char, &T)> {
        let (l, bt, r, tp) = self.get_bounds();
        let xrange = xrange.unwrap_or(l..=r);
        let yrange = yrange.unwrap_or(bt..=tp);
        xrange
            .cartesian_product(yrange)
            .filter_map(|xy| self.get(Point::from(xy)).map(|ct| (Point::from(xy), ct.0, &ct.1)))
    }

    pub fn for_each_mut(
        &mut self,
        xrange: Option<RangeInclusive<isize>>,
        yrange: Option<RangeInclusive<isize>>,
        f: impl Fn(&mut (char, T)) -> (),
    ) {
        let (l, bt, r, tp) = self.get_bounds();
        let xrange = xrange.unwrap_or(l..=r);
        let yrange = yrange.unwrap_or(bt..=tp);
        xrange
            .cartesian_product(yrange)
            .for_each(|xy| {
                if let Some(ct) = self.map.get_mut(&Point::from(xy)) {
                    f(ct);
                }
            });
    }

    pub fn is_wall(&self, p: Point) -> bool {
        self.has_walls()
            && self
                .get(p)
                .map(|ct| ct.0 == self.wall_char)
                .unwrap_or(false)
    }

    pub fn drive(&self, p: Point, d: Direction) -> Option<Point> {
        use Direction::*;

        let mut pnew = p;
        match d {
            N => pnew.y += 1,
            S => pnew.y -= 1,
            E => pnew.x += 1,
            W => pnew.x -= 1,
        };
        if self.in_bounds(pnew) && !self.is_wall(pnew) {
            Some(pnew)
        } else {
            None
        }
    }

    pub fn neighbors(&self, p: Point) -> impl Iterator<Item = Point> {
        use Direction::*;

        vec![
            self.drive(p, N),
            self.drive(p, S),
            self.drive(p, E),
            self.drive(p, W),
        ]
        .into_iter()
        .filter_map(|n| n)
    }

    fn neighbors_default(&self, p: Point) -> Vec<(Point, isize)> {
        self.neighbors(p).map(|(p2)| (p2, 1)).collect()
    }

    pub fn neighbors_with_diagonals(&self, p: Point) -> impl Iterator<Item = Point> {
        use Direction::*;
        vec![
            self.drive(p, N),
            self.drive(p, S),
            self.drive(p, E),
            self.drive(p, W),
            self.drive(p, N).and_then(|p2| self.drive(p2, E)),
            self.drive(p, N).and_then(|p2| self.drive(p2, W)),
            self.drive(p, S).and_then(|p2| self.drive(p2, E)),
            self.drive(p, S).and_then(|p2| self.drive(p2, W)),
        ]
        .into_iter()
        .filter_map(|n| n)
    }

    // Weights on nodes, looks for specific targets
    pub fn dfs_path(
        &self,
        pt1: Point,
        pt2: Point,
        weight: Option<impl Fn(Point) -> isize>,
    ) -> (isize, Vec<Point>) {
        let mut results = self.dfs_path_bulk(pt1, makeset! {pt2}, weight);
        results.remove(&pt2).unwrap()
    }

    pub fn dfs_path_bulk(
        &self,
        pt1: Point,
        dests: HashSet<Point>,
        weight: Option<impl Fn(Point) -> isize>,
    ) -> HashMap<Point, (isize, Vec<Point>)> {
        // Maps point => cost, parent_point
        let mut seen: HashMap<Point, (isize, Point)> = HashMap::new();
        let mut results = HashMap::new();
        let mut stack = Vec::new();
        stack.push((pt1, 0));
        loop {
            let (curr, cost) = stack.pop().expect(&format!(
                "Dead end! found {} out of {}",
                results.len(),
                dests.len()
            ));
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
            for np in self.neighbors(curr) {
                let w = weight.as_ref().map(|f| f(np)).unwrap_or(1);
                if w == isize::MAX {
                    seen.insert(np, (isize::MAX, curr));
                    continue;
                }
                // If we've already seen a lower cost, skip this one
                if seen
                    .get(&np)
                    .map(|(old, _)| *old <= cost + w)
                    .unwrap_or(false)
                {
                    continue;
                }
                seen.insert(np, (cost + w, curr));
                stack.push((np, cost + w));
            }
        }
    }

    // Returns flooded area matched by predicate((from), (to))
    pub fn flood_search_by_pred(
        &self,
        start: Point,
        pred: impl Fn(Point, Point) -> bool,
    ) -> HashSet<Point> {
        let mut q = vec![start];
        let mut res = makeset![start];
        while let Some(p) = q.pop() {
            for n in self.neighbors(p) {
                if !res.contains(&n) && pred(p, n) {
                    q.push(n);
                    res.insert(n);
                }
            }
        }
        res
    }

    // Expand needs to return an list of (point, edge_cost)
    // Returns map of point => (min_cost_sum, parent)
    pub fn bfs_generic(
        &self,
        starts: HashSet<Point>,
        expand: Option<& dyn Fn(Point) -> Vec<(Point, isize)>>,
        is_done: Option<& dyn Fn(&HashMap<Point, (isize, Point)>) -> bool>,
    ) -> HashMap<Point, (isize, Point)> {
        let mut res: HashMap<Point, (isize, Point)> = starts.iter().map(|s| (*s, (0, *s))).collect();
        let mut q = starts.into_iter().collect::<VecDeque<_>>();
        while let Some(p) = q.pop_front() {
            let curr_min_cost = res.get(&p).map(|tup| tup.0).unwrap_or(isize::MAX);
            for n in expand.as_ref().map(|f| f(p)).unwrap_or(self.neighbors_default(p)) {
                let np = n.0;
                let next_cost = curr_min_cost.saturating_add(n.1);
                let should_include = res.get(&np).map(|tup| tup.0).unwrap_or(isize::MAX) > next_cost;
                if should_include {
                    res.insert(np, (next_cost, p));
                    if is_done.as_ref().map(|f| f(&res)).unwrap_or(false) {
                        // Allow short circuiting
                        return res;
                    }
                    q.push_back(np);
                }
            }
        }
        res
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
                    self.get(Point::from((col_no, line_no))).map(|ct| ct.0).unwrap_or(' ')
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
        let cost_fn = |p| {
            g.get(p)
                .map(|ct| if ct.0 == '|' || ct.0 == '-' { 1 } else { 0 })
                .unwrap_or(0)
        };
        let (path_cost, path) = g.dfs_path((1, 1).into(), (3, 7).into(), Some(&cost_fn));
        assert_eq!(
            vec![
                Point::from((3, 7)),
                Point::from((3, 6)),
                Point::from((3, 5)),
                Point::from((3, 4)),
                Point::from((3, 3)),
                Point::from((3, 2)),
                Point::from((3, 1)),
                Point::from((2, 1)),
                Point::from((1, 1))
            ],
            path
        );
        assert_eq!(4, path_cost);
        assert_eq!(9, g.dfs_path((1, 11).into(), (9, 9).into(), Some(&cost_fn)).0);
    }

    #[test]
    fn test_height_width_add() {
        let s = "123\n456\n789";
        let mut g = Grid::new(s, ());
        println!(">\n{}", g.to_string());

        assert_eq!(3, g.width());
        assert_eq!(3, g.height());
        g.add_other(&Grid::new("1101", ()), Direction::S);
        println!(">\n{}", g.to_string());

        assert_eq!(4, g.height());
        assert_eq!(4, g.width());
        g.add_other(&Grid::new("a\nb\nc\nd", ()), Direction::E);
        println!(">\n{}", g.to_string());
        // assert_eq!("", g.to_string());
    }
}
