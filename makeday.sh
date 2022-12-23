#!/bin/bash

if [ $# -eq 0 ]
  then
    echo "No arguments supplied"
    exit 1
fi

echo -e "pub fn part1(input: String) -> String {\n\t\"part1\".to_string()\n}\n\n" >> examples/now/day$1.rs;
echo -e "pub fn part2(input: String) -> String {\n\t\"part2\".to_string()\n}" >> examples/now/day$1.rs;
touch input/now/day$1;
sed -i "s/(day..\?)/(day$1)/" examples/now/main.rs

#code examples/now/day$1.rs
#code input/now/day$1