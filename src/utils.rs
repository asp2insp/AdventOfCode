use fnv::{FnvHashMap, FnvHashSet};
use itertools::*;
use std::any::TypeId;
use std::cell::Cell;
use std::cmp::{max, min};
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::fmt;
use std::hash::Hash;
use std::ops::RangeInclusive;
use std::str::FromStr;


/// Run an A* algorithm over the given search space.
/// Takes a start state, a heuristic function to rank search candidates,
/// an expand function to find neighbors, and finally a done predicate.
/// TODO expand to track path and reconstruct.
pub fn a_star<S, D, H, E>(start: S, heuristic: H, expand: E, done: D) -> Option<isize> //(isize, Vec<S>) 
    where S: Clone + Hash + Eq, D: Fn(&S) -> bool, H: Fn(&S) -> isize, E: Fn(S) -> Vec<(S, isize)> {
        let mut cost = FnvHashMap::default();
        cost.insert(start.clone(), 0isize);
        let mut q = vec![start];
        while let Some(next) = q.pop() {
            let curr_cost = *cost.get(&next).unwrap();
            if done(&next) {
                return Some(curr_cost)
            }
            for (neighbor, step_cost) in expand(next) {
                let cost_e = cost.entry(neighbor.clone()).or_insert(isize::MAX);
                if step_cost + curr_cost < *cost_e {
                    *cost_e = step_cost + curr_cost;
                    q.push(neighbor);
                }
            }
            q.sort_by_cached_key(|e| cost.get(&e).unwrap_or(&isize::MAX).saturating_add(heuristic(e)));
            q.reverse();
        }
        // We can't find a path
        return None;
}

pub fn flatten<T, Outer, Inner>(a: Outer) -> impl Iterator<Item=T> 
    where Outer: IntoIterator<Item=Inner>, Inner: IntoIterator<Item=T> {
        a.into_iter().flat_map(IntoIterator::into_iter)
    }

pub trait IterUtils: Iterator {
    fn counting_set(self) -> FnvHashMap<Self::Item, usize>
    where
        Self: Sized,
        Self::Item: Clone + Eq + Hash,
    {
        self.fold(FnvHashMap::default(), |mut map, it| {
            *map.entry(it).or_insert(0) += 1;
            map
        })
    }

    fn counting_set_by<T: Hash + Eq, F: Fn(Self::Item) -> T>(self, f: F) -> FnvHashMap<T, usize>
    where
        Self: Sized,
    {
        self.fold(FnvHashMap::default(), |mut map, it| {
            let key = f(it);
            *map.entry(key).or_insert(0) += 1;
            map
        })
    }
}

pub fn flip<T, U>((a, b): (T, U)) -> (U, T) {
    (b, a)
}

// Take an MxN matrix and return an NxM one with transposed contents
pub fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> where T: Copy + Default {
    let m = v.len();
    let n = v[0].len();
    let mut ret = vec![vec![Default::default(); m]; n];
    for x in 0..m {
        for y in 0..n {
            ret[y][x] = v[x][y];
        }
    }
    ret
}

#[derive(Clone, Debug)]
pub struct VecSet<T>(Vec<T>);

impl<T> VecSet<T>
where
    T: PartialEq + Clone + Ord,
{
    pub fn new() -> VecSet<T> {
        Self(vec![])
    }

    pub fn contains(&self, t: &T) -> bool {
        self.0.iter().contains(t)
    }

    pub fn insert(&mut self, t: T) -> bool {
        if self.contains(&t) {
            false
        } else {
            self.0.push(t);
            true
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn clone_with(&self, t: T) -> Self {
        let mut nv = VecSet(self.0.clone());
        nv.insert(t);
        nv
    }

    pub fn starts_with(&self, v: &[T]) -> bool {
        self.0.starts_with(&v)
    }

    pub fn key(&self) -> Vec<T> {
        let mut selfsorted = self.0.clone();
        selfsorted.sort();
        selfsorted
    }
}

impl<T> VecSet<T>
where
    T: std::fmt::Debug,
{
    pub fn stringify(&self) -> String {
        format!("{:?}", self)
    }
}

impl<T> PartialEq for VecSet<T>
where
    T: PartialEq + Clone + Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().all(|c| other.contains(c)) && other.0.iter().all(|c| self.contains(c))
    }
}

impl<T> Eq for VecSet<T> where T: PartialEq + Clone + Ord {}

impl<T> Hash for VecSet<T>
where
    T: Clone + Hash + Ord,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key().hash(state);
    }
}

pub struct CharSet {
    // 1 bit per char, 1-27 are upper, followed by lower
    // 0 is null, high bits are metadata
    inner: u64,
}

impl CharSet {
    pub fn new() -> Self {
        CharSet {inner: 0}
    }

    fn conv(c: char) -> u64 {
        if !c.is_ascii_alphabetic() {
            1 << 63
        } else if c.is_lowercase() {
            1 << ((c as usize - 'a' as usize) + 27)
        } else {
            1 << (c as usize - 'A' as usize)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub fn contains(&self, c: char) -> bool {
        self.inner & Self::conv(c) != 0
    }

    // Return true if the given insertion changed the set
    pub fn insert(&mut self, c: char) -> bool {
        let ret = !self.contains(c);
        self.inner |= Self::conv(c);
        ret
    }

    // Return true if the given removal changed the set
    pub fn remove(&mut self, c: char) -> bool {
        let ret = self.contains(c);
        self.inner &= !Self::conv(c);
        ret
    }

    pub fn union(&self, other: &CharSet) -> CharSet {
        CharSet {
            inner: self.inner | other.inner,
        }
    }

    pub fn intersection(&self, other: &CharSet) -> CharSet {
        CharSet {
            inner: self.inner & other.inner,
        }
    }

    // Calculates this - other
    pub fn difference(&self, other: &CharSet) -> CharSet {
        CharSet {
            inner: self.inner & (self.inner ^ other.inner)
        }
    }
}

#[test]
fn char_set() {
    let mut s = CharSet::new();
    s.insert('A');
    s.insert('B');
    s.insert('c');

    assert_eq!(true, true);
    todo!()
}

pub fn div_up(a: usize, b: usize) -> usize {
    (a + (b - 1)) / b
}

pub fn from_bits(bits: &[u8]) -> usize {
    bits.iter().fold(0, |n, b| (n << 1) + *b as usize)
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct P3 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl P3 {
    pub fn origin() -> P3 {
        P3::new(0, 0, 0)
    }

    pub fn new(x: isize, y: isize, z: isize) -> P3 {
        P3 { x, y, z }
    }

    pub fn from(xyz: &(isize, isize, isize)) -> P3 {
        P3::new(xyz.0, xyz.1, xyz.2)
    }

    pub fn dist(&self, other: &P3) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    pub fn euclidian_dist_squared(&self, other: &P3) -> isize {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }

    pub fn rotate_90_around_axis(&self, axis: char) -> P3 {
        match axis {
            'x' => P3 {
                x: self.x,
                y: self.z * -1,
                z: self.y,
            },
            'y' => P3 {
                x: self.z,
                y: self.y,
                z: self.x * -1,
            },
            'z' => P3 {
                x: self.y * -1,
                y: self.x,
                z: self.z,
            },
            _ => unreachable!(),
        }
    }

    pub fn rotate_around_axis(&self, axis: char, times: isize) -> P3 {
        let mut ret = *self;
        for _ in 0..times {
            ret = ret.rotate_90_around_axis(axis);
        }
        ret
    }

    pub fn rotate(&self, rots: P3) -> P3 {
        let mut ret = self.rotate_around_axis('x', rots.x.abs());
        ret = ret.rotate_around_axis('y', rots.y.abs());
        ret.rotate_around_axis('z', rots.z.abs())
    }

    pub fn flip(&self, flip: P3) -> P3 {
        P3 {
            x: self.x * signum_one(flip.x),
            y: self.y * signum_one(flip.y),
            z: self.z * signum_one(flip.z),
        }
    }

    pub fn flip_and_rotate(&self, frots: P3) -> P3 {
        let ret = self.flip(frots);
        ret.rotate(frots)
    }
}

fn signum_one(i: isize) -> isize {
    if i >= 0 {
        1
    } else {
        -1
    }
}

impl std::ops::Add for &P3 {
    type Output = P3;
    fn add(self, rhs: Self) -> Self::Output {
        P3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Add for P3 {
    type Output = P3;
    fn add(self, rhs: Self) -> Self::Output {
        P3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for &P3 {
    type Output = P3;
    fn sub(self, rhs: Self) -> Self::Output {
        P3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Sub for P3 {
    type Output = P3;
    fn sub(self, rhs: Self) -> Self::Output {
        P3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl From<&str> for P3 {
    fn from(s: &str) -> Self {
        let (x, y, z) = s
            .split(',')
            .map(|c| parse!(c, isize))
            .collect_tuple::<_>()
            .unwrap();
        P3 { x, y, z }
    }
}

pub fn toggle<T: Eq>(curr: T, a: T, b: T) -> T {
    if curr == a {
        b
    } else {
        a
    }
}

pub fn wrap(v: isize, low: isize, high: isize) -> isize {
    if v > high {
        low
    } else if v < low {
        high
    } else {
        v
    }
}

#[test]
fn test_bits() {
    assert_eq!(vec![1, 1, 0, 1], to_bits(13, None));
    assert_eq!(13, from_bits(&to_bits(13, None)));
}

pub fn to_bits(mut n: usize, len: Option<usize>) -> Vec<u8> {
    let mut res = Vec::new();
    let mut len = len.unwrap_or(0);
    while n > 0 || len > 0 {
        res.insert(0, (n & 1) as u8);
        n >>= 1;
        len = len.saturating_sub(1);
    }
    res
}

pub fn add_counting_sets<T: Hash + Eq>(
    a: FnvHashMap<T, usize>,
    mut b: FnvHashMap<T, usize>,
) -> FnvHashMap<T, usize> {
    for (k, v) in a.into_iter() {
        *b.entry(k).or_insert(0) += v;
    }
    b
}

pub fn gimme_nums(s: &str) -> Vec<Vec<isize>> {
    parse_nums_from_lines(s.lines())
}

pub fn parse_nums_from_lines<'a>(lines: impl Iterator<Item=&'a str>) -> Vec<Vec<isize>> {
    use regex::*;
    let re = Regex::new(r"([-\d]+)([^-\d]*)").unwrap();
    lines.map(|l| {
            re.captures_iter(l.trim())
                .map(|c| parse!(c[1], isize))
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<Vec<isize>>>()
}

pub fn gimme_usize_nums(s: &str) -> Vec<Vec<usize>> {
    use regex::*;
    let re = Regex::new(r"([\d]+)([^\d]*)").unwrap();
    return s
        .lines()
        .map(|l| {
            re.captures_iter(l.trim())
                .map(|c| parse!(c[1], usize))
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
}

pub fn gimme_chunks(s: &str) -> Vec<Vec<&str>> {
    let mut ret = vec![];
    let mut curr = vec![];
    for line in s.lines() {
        if line.is_empty() {
            ret.push(curr);
            curr = vec![];
        } else {
            curr.push(line);
        }
    }
    ret
}


fn free_neighbors_bounded(p: Point, bounds: Option<(isize, isize, isize, isize)>) -> Vec<Point> {
    veci![
        Point {x: p.x - 1, y: p.y}, if bounds.map(|b| p.x > b.0).unwrap_or(true),
        Point {x: p.x + 1, y: p.y}, if bounds.map(|b| p.x < b.2).unwrap_or(true),
        Point {x: p.x , y: p.y - 1}, if bounds.map(|b| p.y > b.1).unwrap_or(true),
        Point {x: p.x, y: p.y + 1}, if bounds.map(|b| p.y < b.3).unwrap_or(true),
    ]
}

fn default_map<K, V>() -> BTreeMap<K, V> {
    Default::default()
}

impl<T: ?Sized> IterUtils for T where T: Iterator {}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Direction {
    N,
    S,
    E,
    W,
}

pub const DIRECTIONS: [Direction; 4] = [Direction::N, Direction::S, Direction::E, Direction::W];

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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl From<(isize, isize)> for Point {
    fn from(tup: (isize, isize)) -> Point {
        Point { x: tup.0, y: tup.1 }
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
        Point { x, y }
    }

    pub fn offset(&self, offsets: (isize, isize)) -> Self {
        Point {
            x: self.x + offsets.0,
            y: self.y + offsets.1,
        }
    }

    // manhattan distance
    pub fn dist(&self, other: &Point) -> isize {
        self.x.abs_diff(other.x) as isize + self.y.abs_diff(other.y) as isize
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Grid<T> {
    // map: FnvHashMap<Point, (char, T)>,
    map: BTreeMap<Point, (char, T)>,
    pub wall_char: char,
    pub floor_char: char,
    pub left_bound: isize,
    pub right_bound: isize,
    pub top_bound: isize,
    pub bottom_bound: isize,
    wall_cache: Option<Vec<Vec<bool>>>,
}

impl<T> std::hash::Hash for Grid<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

impl<T> Default for Grid<T> {
    fn default() -> Grid<T> {
        Grid {
            map: default_map(),
            wall_char: '\0',
            floor_char: '\0',
            left_bound: 0,
            right_bound: 0,
            top_bound: 0,
            bottom_bound: 0,
            wall_cache: None,
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
        let mut m = default_map();
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
        let mut m = default_map();
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

    pub fn new_iter(&self, f: impl Fn(Point, char, &T) -> (char, T)) -> Grid<T> {
        let mut map = default_map();
        self.iter_range(None, None).for_each(|(p, c, t)| {
            map.insert(p.clone(), f(p, c, t));
        });
        Grid {
            map,
            left_bound: self.left_bound,
            right_bound: self.right_bound,
            top_bound: self.top_bound,
            bottom_bound: self.top_bound,
            wall_char: self.wall_char,
            floor_char: self.floor_char,
            wall_cache: None,
        }
    }

    pub fn with_wall(self, w: char) -> Self {
        let (l, b, r, t) = self.calc_bounds();
        let mut wc = vec![vec![false; t as usize + 1]; r as usize + 1];
        self.iter_chars().for_each(|(p, c)| {
            if c == w {
                wc[p.x as usize][p.y as usize] = true;
            }
        });
        Grid {
            wall_char: w,
            wall_cache: Some(wc),
            ..self
        }
    }

    pub fn add_other(&mut self, other: &Grid<T>, d: Direction)
    where
        T: Clone,
    {
        let offsets = match d {
            Direction::W => (-(other.right_bound + 1) + self.left_bound, 0),
            Direction::N => (0, (self.top_bound + 1) - other.bottom_bound),
            Direction::E => ((self.right_bound + 1) - other.left_bound, 0),
            Direction::S => (0, -(other.top_bound + 1) + self.bottom_bound),
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
            let Point { x, y } = p;
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

        for Point { x, y } in self.map.keys() {
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

    pub fn read(&self, x: isize, y: isize) -> char {
        self.map.get(&Point::from((x, y))).unwrap().0
    }

    pub fn read_pt(&self, p: &Point) -> char {
        self.map.get(p).unwrap().0
    }

    pub fn get_mut(&mut self, p: Point) -> Option<&mut (char, T)> {
        if !self.in_bounds(p) {
            None
        } else {
            self.map.get_mut(&p)
        }
    }

    pub fn set(&mut self, p: Point, c: char, t: T) {
        if self.in_bounds(p) {
            self.map.insert(p, (c, t));
        }
    }

    pub fn swap(&mut self, pfrom: Point, pto: Point) {
        if self.in_bounds(pfrom) && self.in_bounds(pto) {
            let from = self.map.remove(&pfrom).unwrap();
            let to = self.map.remove(&pto).unwrap();
            self.map.insert(pfrom, to);
            self.map.insert(pto, from);
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

    pub fn find(&self, needle: char) -> Option<Point> {
        self.iter_chars()
            .find(|(p, c)| *c == needle)
            .map(|(p, _)| p)
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
        xrange.cartesian_product(yrange).filter_map(|xy| {
            self.get(Point::from(xy))
                .map(|ct| (Point::from(xy), ct.0, &ct.1))
        })
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
        xrange.cartesian_product(yrange).for_each(|xy| {
            if let Some(ct) = self.map.get_mut(&Point::from(xy)) {
                f(ct);
            }
        });
    }

    pub fn is_wall(&self, p: Point) -> bool {
        self.has_walls()
            && self.wall_cache.as_ref().map_or_else(
                || {
                    self.get(p)
                        .map(|ct| ct.0 == self.wall_char)
                        .unwrap_or(false)
                },
                |wc| wc[p.x as usize][p.y as usize],
            )
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

    pub fn drive_iter<'a>(&'a self, p: Point, d: Direction) -> impl Iterator<Item=Point> + 'a {
        DriveIter {
            dir: d,
            curr: Some(p),
            g: self,
        }
    }

    pub fn drive_wrap(&self, p: Point, d: Direction) -> Point {
        use Direction::*;
        let mut pnew = p;
        match d {
            N => pnew.y = wrap(p.y + 1, self.bottom_bound, self.top_bound),
            S => pnew.y = wrap(p.y - 1, self.bottom_bound, self.top_bound),
            E => pnew.x = wrap(p.x + 1, self.left_bound, self.right_bound),
            W => pnew.x = wrap(p.x - 1, self.left_bound, self.right_bound),
        };
        pnew
    }

    pub fn drive2(&self, p: Point, d: Direction, d2: Direction) -> Option<Point> {
        use Direction::*;

        let mut pnew = p;
        match d {
            N => pnew.y += 1,
            S => pnew.y -= 1,
            E => pnew.x += 1,
            W => pnew.x -= 1,
        };
        match d2 {
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
            self.drive2(p, N, E),
            self.drive2(p, N, W),
            self.drive2(p, S, E),
            self.drive2(p, S, W),
        ]
        .into_iter()
        .filter_map(|n| n)
    }

    pub fn three_by_three(&self, p: Point, default: char) -> impl Iterator<Item = char> {
        use Direction::*;
        vec![
            self.drive2(p, N, W)
                .and_then(|p3| self.get(p3))
                .map(|tup| tup.0),
            self.drive(p, N)
                .and_then(|p3| self.get(p3))
                .map(|tup| tup.0),
            self.drive2(p, N, E)
                .and_then(|p3| self.get(p3))
                .map(|tup| tup.0),
            self.drive(p, W)
                .and_then(|p3| self.get(p3))
                .map(|tup| tup.0),
            self.get(p).map(|tup| tup.0),
            self.drive(p, E)
                .and_then(|p3| self.get(p3))
                .map(|tup| tup.0),
            self.drive2(p, S, W)
                .and_then(|p3| self.get(p3))
                .map(|tup| tup.0),
            self.drive(p, S)
                .and_then(|p3| self.get(p3))
                .map(|tup| tup.0),
            self.drive2(p, S, E)
                .and_then(|p3| self.get(p3))
                .map(|tup| tup.0),
        ]
        .into_iter()
        .map(move |o| o.unwrap_or(default))
    }

    // Weights on nodes, looks for specific targets
    pub fn dfs_path(
        &self,
        pt1: Point,
        pt2: Point,
        weight: Option<impl Fn(Point) -> isize>,
    ) -> (isize, Vec<Point>) {
        self.try_dfs_path(pt1, pt2, weight).unwrap()
    }

    pub fn try_dfs_path(
        &self,
        pt1: Point,
        pt2: Point,
        weight: Option<impl Fn(Point) -> isize>,
    ) -> Option<(isize, Vec<Point>)> {
        let mut results = self.dfs_path_bulk(pt1, makeset! {pt2}, weight);
        results.remove(&pt2)
    }

    pub fn dfs_path_bulk(
        &self,
        pt1: Point,
        dests: FnvHashSet<Point>,
        weight: Option<impl Fn(Point) -> isize>,
    ) -> FnvHashMap<Point, (isize, Vec<Point>)> {
        // Maps point => cost, parent_point
        let mut seen: FnvHashMap<Point, (isize, Point)> =
            FnvHashMap::with_capacity_and_hasher(dests.len() * 10, Default::default());
        let mut results = FnvHashMap::with_capacity_and_hasher(dests.len(), Default::default());
        let mut stack = Vec::new();
        stack.push((pt1, 0));
        loop {
            if stack.is_empty() {
                return results;
            }
            let (curr, cost) = stack.pop().expect(&format!(
                "Dead end! found {} out of {}",
                results.len(),
                dests.len()
            ));
            if dests.contains(&curr)
                && cost < results.get(&curr).map(|(c, _)| *c).unwrap_or(isize::MAX)
            {
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
                seen.insert(np, (cost.saturating_add(w), curr));
                stack.push((np, cost.saturating_add(w)));
            }
        }
    }

    // Returns flooded area matched by predicate((from), (to))
    pub fn flood_search_by_pred(
        &self,
        start: Point,
        pred: impl Fn(Point, Point) -> bool,
    ) -> FnvHashSet<Point> {
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
        starts: FnvHashSet<Point>,
        expand: Option<&dyn Fn(Point) -> Vec<(Point, isize)>>,
        is_done: Option<&dyn Fn(&FnvHashMap<Point, (isize, Point)>) -> bool>,
    ) -> FnvHashMap<Point, (isize, Point)> {
        let mut res: FnvHashMap<Point, (isize, Point)> =
            starts.iter().map(|s| (*s, (0, *s))).collect();
        let mut q = starts.into_iter().collect::<VecDeque<_>>();
        while let Some(p) = q.pop_front() {
            let curr_min_cost = res.get(&p).map(|tup| tup.0).unwrap_or(isize::MAX);
            for n in expand
                .as_ref()
                .map(|f| f(p))
                .unwrap_or(self.neighbors_default(p))
            {
                let np = n.0;
                let next_cost = curr_min_cost.saturating_add(n.1);
                let should_include =
                    res.get(&np).map(|tup| tup.0).unwrap_or(isize::MAX) > next_cost;
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
            write!(
                f,
                "{}",
                (l..=r)
                    .map(|col_no| self
                        .get(Point::from((col_no, line_no)))
                        .map(|ct| ct.0)
                        .unwrap_or(' '))
                    .collect::<String>()
            )?;
            if line_no != b {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

struct DriveIter<'a, T> {
    curr: Option<Point>,
    dir: Direction,
    g: &'a Grid<T>,
}

impl <'a, T> Iterator for DriveIter<'a, T> {
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
        if let Some(p) = self.curr {
            self.curr = self.g.drive(p, self.dir);
            self.curr
        } else {
            None
        }
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
        assert_eq!(
            9,
            g.dfs_path((1, 11).into(), (9, 9).into(), Some(&cost_fn)).0
        );
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
