// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use ahash::{HashMap, HashMapExt};
use std::str::FromStr;

type InputType = (SolutionType, SolutionType);
type SolutionType = i32;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    let mut result = vec![];
    for line in input.lines() {
        let line: Vec<_> = line.split("   ").collect();
        result.push((
            SolutionType::from_str(line[0]).unwrap(),
            SolutionType::from_str(line[1]).unwrap(),
        ))
    }
    result
}

#[aoc(day1, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let mut left: Vec<_> = data.iter().map(|(a, _b)| a).collect();
    let mut right: Vec<_> = data.iter().map(|(_a, b)| b).collect();
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .map(|(&a, &b)| SolutionType::abs(a - b))
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let mut right_count = HashMap::new();
    for number in data.iter().map(|(_a, b)| b) {
        right_count
            .entry(number)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    data.iter().map(|(a, _b)| a).fold(0, |count, number| {
        if let Some(right_count) = right_count.get(number) {
            count + number * right_count
        } else {
            count
        }
    })
}
