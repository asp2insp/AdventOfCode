#!/bin/bash

if [ $# -eq 0 ]
  then
    DAY=$(python3 -c "import datetime; print(datetime.date.today().day + 1)")
else
    DAY=$1
fi



echo -e "pub fn part1(input: String) -> String {\n\t\"part1\".to_string()\n}\n\n" > examples/now/day$DAY.rs;
echo -e "pub fn part2(input: String) -> String {\n\t\"part2\".to_string()\n}" >> examples/now/day$DAY.rs;
touch input/now/day$DAY;
sed -i "s/(day..\?)/(day$DAY)/" examples/now/main.rs

#code examples/now/day$DAY.rs
#code input/now/day$DAY
