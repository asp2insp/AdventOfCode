#!/bin/bash
echo -e "pub fn part1(input: String) -> String {\n\t\"part1\".to_string()\n}\n\n" > src/day$1.rs;
echo -e "pub fn part2(input: String) -> String {\n\t\"part2\".to_string()\n}" >> src/day$1.rs;
touch input/day$1;
