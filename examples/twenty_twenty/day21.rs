use aoc::dict;
use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;
use regex::Regex;

fn get_recipes(s: String) -> Vec<(FnvHashSet<String>, FnvHashSet<String>)> {
    let re = Regex::new(r"([\w ]+)\(contains ([\w, ]+)\)?").unwrap();
    s.lines()
        .flat_map(|l| re.captures(l))
        .map(|cap| {
            (
                cap[1]
                    .split_ascii_whitespace()
                    .map(|s| s.to_string())
                    .collect(),
                cap[2].split(", ").map(|s| s.to_string()).collect(),
            )
        })
        .collect_vec()
}

fn find_possibles(
    recipes: &Vec<(FnvHashSet<String>, FnvHashSet<String>)>,
) -> FnvHashMap<String, FnvHashSet<String>> {
    let mut allergen_possibilities: FnvHashMap<String, FnvHashSet<String>> = dict!();
    for (ingredients, allergens) in recipes {
        for allergen in allergens {
            if !allergen_possibilities.contains_key(allergen) {
                // if allergen == "fish" {println!("Setting fish {:?}", ingredients);}
                allergen_possibilities.insert(allergen.clone(), ingredients.clone());
            } else {
                let narrowed = allergen_possibilities
                    .get(allergen)
                    .unwrap()
                    .intersection(&ingredients)
                    .cloned()
                    .collect();
                if allergen == "fish" {
                    // println!("Narrowing fish {:?}", narrowed);
                }
                allergen_possibilities.insert(allergen.clone(), narrowed);
            }
        }
    }
    let mut changed = true;
    while changed {
        changed = false;
        for allergen in &allergen_possibilities.keys().cloned().collect_vec() {
            if allergen_possibilities.get(allergen).unwrap().len() == 1 {
                let aller_ingredient = allergen_possibilities
                    .get(allergen)
                    .unwrap()
                    .iter()
                    .next()
                    .unwrap()
                    .clone();
                allergen_possibilities.iter_mut().for_each(|(a, poss)| {
                    if a != allergen {
                        changed |= poss.remove(&aller_ingredient);
                    }
                });
            }
        }
    }
    allergen_possibilities
}

pub fn part1(s: String) -> String {
    let recipes = get_recipes(s);
    let all_ingredients = recipes
        .iter()
        .flat_map(|(i, _)| i.iter())
        .cloned()
        .collect::<FnvHashSet<String>>();
    let allergen_possibilities = find_possibles(&recipes);
    let all_possible_ingredients = allergen_possibilities
        .values()
        .flat_map(|v| v)
        .cloned()
        .collect::<FnvHashSet<_>>();
    // println!("Possibles: {:?}", allergen_possibilities);
    all_ingredients
        .difference(&all_possible_ingredients)
        .map(|i| recipes.iter().filter(|(is, _)| is.contains(i)).count())
        .sum::<usize>()
        .to_string()
}

pub fn part2(s: String) -> String {
    let recipes = get_recipes(s);
    let allergen_possibilities = find_possibles(&recipes);
    allergen_possibilities.into_iter()
        .map(|(a, i)| (a, i.into_iter().next().unwrap()))
        .sorted_by_key(|(a, i)| a.clone())
        .map(|(a, i)| i)
        .join(",")
}

#[test]
fn test() {
    let s = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
    assert_eq!("5", part1(s.to_string()));
}
