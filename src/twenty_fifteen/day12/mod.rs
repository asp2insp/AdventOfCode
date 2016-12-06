use serde_json::{Value,from_str};
use std::collections::BTreeMap;

fn count_nums(data: &Value) -> i64 {
	match data {
		&Value::U64(i) => i as i64,
	    &Value::I64(i) => i,
	    &Value::Array(ref v)  => v.iter().map(count_nums).fold(0, |s, n| s + n),
	    &Value::Object(ref m) => m.values().map(count_nums).fold(0, |s, n| s + n),
		_ => 0,
	}
}

fn has_red_value(m: &BTreeMap<String, Value>) -> bool {
	m.values()
		.fold(false, |acc, v| {
			acc | match v {
				&Value::String(ref s) => s.eq(&"red".to_string()),
				_ => false,
			}
		})
}

fn count_nums_no_red(data: &Value) -> i64 {
	match data {
		&Value::U64(i) => i as i64,
	    &Value::I64(i) => i,
	    &Value::Array(ref v)  => v.iter().map(count_nums_no_red).fold(0, |s, n| s + n),
	    &Value::Object(ref m) => {
			if has_red_value(m) {
				0
			} else {
				m.values()
				.map(count_nums_no_red)
				.fold(0, |s, n| s + n)
			}
		},
		_ => 0,
	}
}

pub fn part1(input: String) -> String {
	let data: Value = from_str(input.as_ref()).unwrap();
	format!("{}", count_nums(&data))
}


pub fn part2(input: String) -> String {
	let data: Value = from_str(input.as_ref()).unwrap();
	format!("{}", count_nums_no_red(&data))
}
