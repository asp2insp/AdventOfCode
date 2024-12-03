use aoc::utils::{gimme_usizes_once, ToDebugString, BetterRange};

#[derive(Debug)]
struct RangeMap {
    source_start: usize,
    dest_start: usize,
    length: usize,
}

impl RangeMap {
    fn new(source_start: usize, dest_start: usize, length: usize) -> Self {
        Self {
            source_start,
            dest_start,
            length,
        }
    }

    fn get(&self, lookup: usize) -> Option<usize> {
        if lookup >= self.source_start && lookup < self.source_start + self.length {
            Some(self.dest_start + (lookup - self.source_start))
        } else {
            None
        }
    }

    fn lookup_range(&self, r: &BetterRange) -> Option<BetterRange> {
        BetterRange::new_from_length(self.source_start, self.length)
			.intersection(r)
			.map(|si| BetterRange::new_from_length(self.dest_start + (si.bottom - self.source_start), si.len()))
    }
}

fn lookup_group(lookup: usize, maps: &[RangeMap]) -> Option<usize> {
    maps.into_iter().flat_map(|m| m.get(lookup)).next()
}

fn lookup_group_range(r: BetterRange, maps: &[RangeMap]) -> Vec<BetterRange> {
    maps.into_iter()
        .flat_map(|m| m.lookup_range(&r))
        .collect()
}

fn parse_maps<'a>(l: impl Iterator<Item = &'a str>) -> Vec<Vec<RangeMap>> {
    let mut maps = Vec::new();
    let mut map: Vec<RangeMap> = vec![];
    for s in l {
        if s.is_empty() {
            map.sort_by(|a, b| a.source_start.cmp(&b.source_start));
            maps.push(map);
            map = vec![];
            continue;
        } else if s.contains("map:") {
            continue; // skip headers
        }
        // Otherwise grab numbers from the line and add them to the map
        let nums = gimme_usizes_once(s);
        map.push(RangeMap::new(nums[1], nums[0], nums[2]));
    }
    map.sort_by(|a, b| a.source_start.cmp(&b.source_start));
    maps.push(map);
    maps
}

pub fn part1(input: String) -> String {
    let mut l = input.lines();
    let seeds = gimme_usizes_once(l.next().expect("No lines in input"));
    let maps = parse_maps(l.skip(1));
    seeds
        .into_iter()
        .map(|s| maps.iter().fold(s, |s, m| lookup_group(s, m).unwrap_or(s)))
        .min()
        .to_debug_string()
}

pub fn part2(input: String) -> String {
    let mut l = input.lines();
    let seeds = gimme_usizes_once(l.next().expect("No lines in input"));
    let maps = parse_maps(l.skip(1));
    let all_seeds = seeds
        .chunks(2)
        .map(|c| BetterRange::new_from_length(c[0], c[1]))
        .collect::<Vec<_>>();
    maps.into_iter()
        .fold(all_seeds, |seedranges, map| {
            seedranges
                .into_iter()
                .flat_map(|r| lookup_group_range(r, &map))
                .collect::<Vec<_>>()
        })
        .into_iter()
        .map(|r| r.bottom)
        .min()
        .to_debug_string()
}
