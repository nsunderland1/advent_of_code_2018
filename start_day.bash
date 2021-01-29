#!/bin/bash

DAY=$1
DAY_PADDED=$(printf "%02d" $DAY)

firefox https://adventofcode.com/2018/day/${DAY}

cargo new day${DAY_PADDED}

code day${DAY_PADDED}/src/main.rs

curl "https://adventofcode.com/2018/day/${DAY}/input" -H "Cookie: session=${ADVENT_SESSION}" > "day${DAY_PADDED}/input"

