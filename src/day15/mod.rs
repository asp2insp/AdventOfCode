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

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Ingredient {
	name: String,
	capacity: i64,
	durability: i64,
	flavor: i64,
	texture: i64,
	calories: i64,
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
		// let mut calories = 0i64;

		for ia in &self.ingredients {
			capacity += ia.0.capacity * ia.1;
			durability += ia.0.durability * ia.1;
			flavor += ia.0.flavor * ia.1;
			texture += ia.0.texture * ia.1;
			// calories += ia.0.calories * ia.1;
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

fn best_recipe(r: &Recipe, ing: &Vec<Ingredient>, memo: &mut HashMap<Recipe, i64>) -> i64 {
	if memo.contains_key(r) {
		return *memo.get(r).unwrap()
	}
	let ret = if r.len() == 100 {
		r.score()
	} else {
		let mut best = 0i64;
		for ingredient in ing {
			let mut nr = r.clone();
			*nr.ingredients.get_mut(ingredient).unwrap() += 1;
			let ns = best_recipe(&nr, ing, memo);
			if ns > best {
				best = ns;
			}
		}
		best
	};
	memo.insert(r.clone(), ret);
	ret
}

pub fn part1(input: String) -> String {
	let ing = parse_only(all_ingredients, input.as_bytes()).unwrap();
	let mut starting_ingredients: HashMap<Ingredient, i64> = HashMap::new();
	for i in &ing {
		starting_ingredients.insert(i.clone(), 0);
	}
	let mut memo: HashMap<Recipe, i64> = HashMap::new();
	let m = best_recipe(&Recipe {
			ingredients: starting_ingredients.clone(),
		}, &ing, &mut memo);
	println!("len memo {}", memo.len());
	format!("{:?}", m)
}


pub fn part2(input: String) -> String {
	"part2".to_string()
}
