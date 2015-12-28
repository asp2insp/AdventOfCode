use std::str;
use chomp::*;
use chomp::ascii::{skip_whitespace,signed,is_alpha,decimal};

fn alpha_string(i: Input<u8>) -> U8Result<String> {
	parse!{i;
		let s = take_while(is_alpha);
		ret str::from_utf8(s).unwrap().to_string()
	}
}

fn ingredient(i: Input<u8>) -> U8Result<Ingredient> {
	// Sprinkles: capacity 2, durability 0, flavor -2, texture 0, calories 3
	parse!{i;
		let n  		   = alpha_string();
						 string(b": capacity ");
		let cap: i64   = signed(decimal);
						 string(b", durability ");
		let dur: i64   = signed(decimal);
						 string(b", flavor ");
		let fla: i64   = signed(decimal);
						 string(b", texture ");
		let tex: i64   = signed(decimal);
						 string(b", calories ");
		let cal: i64   = signed(decimal);
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

#[derive(Clone, Debug)]
struct Ingredient {
	name: String,
	capacity: i64,
	durability: i64,
	flavor: i64,
	texture: i64,
	calories: i64,
}

#[derive(Debug, Clone)]
struct Recipe {
	ingredients: Vec<Ingredient>,
}

impl Recipe {
	fn score(&self) -> i64 {
		let mut capacity = 0i64;
		let mut durability = 0i64;
		let mut flavor = 0i64;
		let mut texture = 0i64;
		let mut calories = 0i64;

		for ia in &self.ingredients {
			capacity += ia.capacity;
			durability += ia.durability;
			flavor += ia.flavor;
			texture += ia.texture;
			calories += ia.calories;
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

	fn len(&self) -> usize {
		self.ingredients.len()
	}
}

fn recipies(r: &Recipe, ing: &Vec<Ingredient>) -> Vec<Recipe> {
	if r.len() == 10 {
		vec![r.clone()]
	} else {
		let mut v: Vec<Recipe> = Vec::new();
		for ingredient in ing {
			let mut nr = r.clone();
			nr.ingredients.push(ingredient.clone());
			v.append(&mut recipies(&nr, ing));
		}
		v
	}
}

fn max(a: i64, b: i64) -> i64 {
	if a > b {
		b
	} else {
		a
	}
}

pub fn part1(input: String) -> String {
	let ing = parse_only(all_ingredients, input.as_bytes()).unwrap();
	let m = recipies(&Recipe {
			ingredients: vec![],
		}, &ing)
		.iter()
		.fold(0, |best, r| max(best, r.score()));
	format!("{}", m)
}


pub fn part2(input: String) -> String {
	"part2".to_string()
}
