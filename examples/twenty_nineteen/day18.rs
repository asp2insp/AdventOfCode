use aoc::utils::*;
use std::collections::{BinaryHeap};
use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
struct State {
    cost: isize,
    loc: Point,
    keys: VecSet<char>,
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.keys
            .len()
            .cmp(&other.keys.len())
            .then(other.cost.cmp(&self.cost))
        // other.cost
        //     .cmp(&self.cost)
        //     .then(self.keys.len().cmp(&other.keys.len()))
    }
}

impl State {
    fn new(start_loc: Point) -> Self {
        State {
            cost: 0,
            loc: start_loc,
            keys: VecSet::new(),
        }
    }

    fn key(&self) -> (Vec<char>, Point) {
        (self.keys.key(), self.loc)
    }
}

pub fn part1(input: String) -> String {
    let (grid, start_loc, keys) = parse(&input);
    let mut states = BinaryHeap::new();
    states.push(State::new(start_loc));
    let mut gen = 0;
    let mut best = isize::MAX;
    let mut seen = FnvHashMap::default();
    while let Some(s) = states.pop() {
        if s.cost >= best || s.cost >= *seen.get(&s.key()).unwrap_or(&best) {
            continue;
        }
        seen.insert(s.key(), s.cost);
        let currbest = (s.cost, s.keys.clone());
        if s.keys.len() == keys.len() {
            println!("Possible Soln {:?} with cost {}", s.keys, s.cost);
            if s.cost < best {
                best = s.cost;
                // states = states.into_iter().filter(|s| s.cost <= best).collect();
            }
			continue
        }
        gen += 1;
        states.extend(
            reachable(&grid, &keys, s),
        );
        if gen % 1000 == 0 {
            println!(
                "Gen {} with {} current states. Current is {:?} keys at a cost of {}",
                gen,
                states.len(),
                currbest.1,
                currbest.0,
            );
        }
    }
    return best.to_string();
}

fn parse(s: &str) -> (Grid<()>, Point, FnvHashMap<char, Point>) {
    let g = Grid::new(s, ()).with_wall('#');
    let start_loc = g.find('@').unwrap();
    let keys = g
        .iter_chars()
        .filter(|(_, c)| *c <= 'z' && *c >= 'a')
        .map(flip)
        .collect();
    (g, start_loc, keys)
}

fn reachable(g: &Grid<()>, keys: &FnvHashMap<char, Point>, start: State) -> Vec<State> {
    let reachable_keys: FnvHashSet<Point> = g
        .flood_search_by_pred(start.loc, |f, t| {
			let fc = g.read_pt(&f);
			let tc = g.read_pt(&t);
			if keys.contains_key(&fc) && !start.keys.contains(&fc) {
				false
			} else if tc.is_uppercase() && tc.is_alphabetic() && !start.keys.contains(&tc.to_ascii_lowercase()) {
				false
			} else {
				true
			}
		})
        .into_iter()
        .filter_map(|p| {
			let c = &g.read_pt(&p);
            if start.keys.contains(c) || !keys.contains_key(c) {
                None
            } else {
                Some(p)
            }
        })
        .collect();
	// println!("reachable keys {:?} from {:?} with {:?}", reachable_keys.iter().map(|p| g.read_pt(p)).collect_vec(), start.loc, start.keys);
    let ret = g.dfs_path_bulk(
        start.loc,
        reachable_keys,
        Some(|np| {
            let nc = g.get(np).unwrap().0;
            if nc == '.'
                || nc == '@'
                || keys.contains_key(&nc)
                || start.keys.contains(&nc.to_ascii_lowercase())
            {
                1
            } else {
                isize::MAX
            }
        }),
    )
    .into_iter()
    .map(|(p, (ncost, _))| State {
        loc: p,
        cost: start.cost.saturating_add(ncost),
        keys: start.keys.clone_with(g.get(p).unwrap().0),
    })
    .collect();
	// println!("    {:?}", ret);
	ret
}

pub fn part2(input: String) -> String {
    "part2".to_string()
}

#[test]
fn test_simple() {
    let s = r#"#########
#b.A.@.a#
#########
"#;
    let (grid, start_loc, keys) = parse(s);
    assert_eq!(
        "[State { cost: 2, loc: Point { x: 7, y: 1 }, keys: VecSet(['a']) }]",
        format!("{:?}", reachable(&grid, &keys, State::new(start_loc)))
    );
    assert_eq!("8", part1(s.to_owned()));
}

#[test]
fn test_larger() {
    let s = r#"########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################
"#;
        assert_eq!("86", part1(s.to_owned()));

}

#[test]
fn test_2() {
    let s2 = r#"########################
    #...............b.C.D.f#
    #.######################
    #.....@.a.B.c.d.A.e.F.g#
    ########################"#;
        assert_eq!("132", part1(s2.to_owned()));
}

#[test]
fn test_3() {
    let s3 =
    r#"#################
    #i.G..c...e..H.p#
    ########.########
    #j.A..b...f..D.o#
    ########@########
    #k.E..a...g..B.n#
    ########.########
    #l.F..d...h..C.m#
    #################"#;
            assert_eq!("136", part1(s3.to_owned()));
    // assert!(false)
}

#[test]
fn test_4() {
    let s4 = 
    r#"########################
    #@..............ac.GI.b#
    ###d#e#f################
    ###A#B#C################
    ###g#h#i################
    ########################"#;
        assert_eq!("81", part1(s4.to_owned()));
    
        // assert!(false);
}