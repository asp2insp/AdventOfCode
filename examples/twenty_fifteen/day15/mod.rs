use std::str;
use chomp::*;
use chomp::ascii::{skip_whitespace,is_alpha,decimal};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

fn alpha_string(i: Input<u8>) -> U8Result<String> {
	parse!{i;
		let s = take_while(is_alpha);
		ret str::from_utf8(s).unwrap().to_string()
	}
}

fn opt_sign(i: Input<u8>) -> U8Result<i64> {
	option(i, |i| parse!{i;
			token(b'-');
			ret -1
		},
		1
	)
}

fn ndecimal(i: Input<u8>) -> U8Result<i64> {
	parse!{i;
		let sign      = opt_sign();
		let num: i64  = decimal();
		ret num * sign
	}
}

fn ingredient(i: Input<u8>) -> U8Result<Ingredient> {
	parse!{i;
		let n  		   = alpha_string();
						 string(b": capacity ");
		let cap        = ndecimal();
						 string(b", durability ");
		let dur        = ndecimal();
						 string(b", flavor ");
		let fla        = ndecimal();
						 string(b", texture ");
		let tex        = ndecimal();
						 string(b", calories ");
		let cal        = ndecimal();
				 		 skip_whitespace();
		ret Ingredient {
			name: n,
			capacity: cap,
			durability: dur,
			flavor: fla,
			texture: tex,
			calories: cal,
		}
	}
}

fn all_ingredients(i: Input<u8>) -> U8Result<Vec<Ingredient>> {
	parse!{i;
		let v = many1(ingredient);
		ret v
	}
}

#[derive(Clone, Debug, Eq)]
struct Ingredient {
	name: String,
	capacity: i64,
	durability: i64,
	flavor: i64,
	texture: i64,
	calories: i64,
}

impl PartialEq for Ingredient {
	fn eq(&self, other: &Ingredient) -> bool {
		self.name == other.name
	}
}

impl Hash for Ingredient {
    fn hash<H: Hasher>(&self, state: &mut H) {
		self.name.chars().next().hash(state);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Recipe {
	ingredients: HashMap<Ingredient, i64>,
}

impl Recipe {
	fn score(&self) -> i64 {
		let mut capacity = 0i64;
		let mut durability = 0i64;
		let mut flavor = 0i64;
		let mut texture = 0i64;

		for ia in &self.ingredients {
			capacity += ia.0.capacity * ia.1;
			durability += ia.0.durability * ia.1;
			flavor += ia.0.flavor * ia.1;
			texture += ia.0.texture * ia.1;
		}
		if capacity < 0 {
			capacity = 0;
		}
		if durability < 0 {
			durability = 0;
		}
		if flavor < 0 {
			flavor = 0;
		}
		if texture < 0 {
			texture = 0;
		}

		capacity * durability * flavor * texture
	}

	fn len(&self) -> i64 {
		self.ingredients.values().fold(0, |sum, i| sum + i)
	}

	fn calories(&self) -> i64 {
		let mut calories = 0i64;
		for ia in &self.ingredients {
			calories += ia.0.calories * ia.1;
		}
		calories
	}
}

impl Hash for Recipe {
    fn hash<H: Hasher>(&self, state: &mut H) {
		let mut key = String::new();
		for kv in &self.ingredients {
			key = format!("{},{}", key, kv.1);
		}
		key.hash(state);
    }
}

fn best_recipe(r: &Recipe, ing: &Vec<Ingredient>) -> i64 {
	if r.len() == 100 {
		r.score()
	} else {
		let mut best = 0i64;
		for ingredient in ing {
			let new_ing:Vec<Ingredient> = ing.iter()
				.filter(|i| i.name != ingredient.name)
				.map(|i| i.clone())
				.collect();
			for inum in 0..(101-r.len()) {
				let mut nr = r.clone();
				*nr.ingredients.get_mut(ingredient).unwrap() += inum;
				let ns = best_recipe(&nr, &new_ing);
				if ns > best {
					best = ns;
				}
			}
		}
		best
	}
}

fn best_recipe_500_cals(r: &Recipe, ing: &Vec<Ingredient>) -> i64 {
	if r.calories() > 500 {
		0
	} else if r.len() == 100 {
		if r.calories() == 500 {
			r.score()
		} else {
			0
		}
	} else {
		let mut best = 0i64;
		for ingredient in ing {
			let new_ing:Vec<Ingredient> = ing.iter()
				.filter(|i| i.name != ingredient.name)
				.map(|i| i.clone())
				.collect();
			for inum in 0..(101-r.len()) {
				let mut nr = r.clone();
				*nr.ingredients.get_mut(ingredient).unwrap() += inum;
				let ns = best_recipe_500_cals(&nr, &new_ing);
				if ns > best {
					best = ns;
				}
			}
		}
		best
	}
}

pub fn part1(input: String) -> String {
	let ing = parse_only(all_ingredients, input.as_bytes()).unwrap();
	let mut starting_ingredients: HashMap<Ingredient, i64> = HashMap::new();
	for i in &ing {
		starting_ingredients.insert(i.clone(), 0);
	}
	starting_ingredients.shrink_to_fit();
	let m = best_recipe(&Recipe {
			ingredients: starting_ingredients.clone(),
		}, &ing);
	format!("{:?}", m)
}


pub fn part2(input: String) -> String {
	let ing = parse_only(all_ingredients, input.as_bytes()).unwrap();
	let mut starting_ingredients: HashMap<Ingredient, i64> = HashMap::new();
	for i in &ing {
		starting_ingredients.insert(i.clone(), 0);
	}
	starting_ingredients.shrink_to_fit();
	let m = best_recipe_500_cals(&Recipe {
			ingredients: starting_ingredients.clone(),
		}, &ing);
	format!("{:?}", m)
}
