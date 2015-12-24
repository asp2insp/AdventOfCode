#!/bin/bash
mkdir src/$1;
echo -e "pub fn part1(input: String) -> String {\n\t\"part1\".to_string()\n}\n\n" > src/$1/mod.rs;
echo -e "pub fn part2(input: String) -> String {\n\t\"part2\".to_string()\n}" >> src/$1/mod.rs;
touch src/$1/input.txt
