use aoc::{makeset, utils::*};

pub fn part1(input: String) -> String {
    use Direction::*;
    let g = Grid::new(&input, ());
    let mut seen = makeset!();
    let mut total_price = 0;
    for (p, c, _) in g.iter() {
        if seen.contains(&p) {
            continue;
        }
        let region = g.flood_search_by_pred(p, |_, d| g.read_pt(&d) == c);
        let borders = g.borders_of_contiguous_area(&region);
        let border_len = borders
            .into_iter()
            .flat_map(|p| [N, E, S, W].map(|d| g.drive(p, d)))
            .filter(|np| np.is_none_or(|nnp| g.read_pt(&nnp) != c))
            .count();
        total_price += region.len() * border_len;
        // println!("{}: {} * {} = {}", c, region.len(), border_len, region.len() * border_len);
        seen.extend(region);
    }
    total_price.to_string()
}

pub fn part2(input: String) -> String {
    let g = Grid::new(&input, ());
    let mut seen = makeset!();
    let mut total_price = 0;
    for (p, c, _) in g.iter() {
        if seen.contains(&p) {
            continue;
        }
        let region = g.flood_search_by_pred(p, |_, d| g.read_pt(&d) == c);
        let sides = g.sides_of_contiguous_area(&region);
        // println!("{}: {:?}", c, sides);
        total_price += region.len() * sides.len();
        seen.extend(region);
    }
    total_price.to_string()
}

#[test]
fn test_small() {
    let s = r#"AAAA
BBCD
BBCC
EEEC"#;
    assert_eq!("140", part1(s.to_string()));
    assert_eq!("80", part2(s.to_string()));
}

#[test]
fn test_islands() {
    let s = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;
    assert_eq!("772", part1(s.to_string()));
    assert_eq!("436", part2(s.to_string()));
}
