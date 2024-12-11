// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use ahash::{HashMap, HashMapExt};
use std::str::FromStr;

type InputType = SolutionType;
type SolutionType = u64;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input
        .split(' ')
        .map(|s| SolutionType::from_str(s).unwrap())
        .collect()
}

fn get_digits(n: InputType) -> u32 {
    SolutionType::ilog10(n) + 1
}

fn count(data: &[InputType], blinks: u8) -> SolutionType {
    let mut curr: HashMap<SolutionType, SolutionType> = HashMap::new();
    for &stone in data {
        *curr.entry(stone).or_insert(0) += 1;
    }
    let mut next = HashMap::new();
    for _blink in 0..blinks {
        for (&stone, &n_stones) in &curr {
            if stone == 0 {
                *next.entry(1).or_insert(0) += n_stones;
            } else {
                let digits = get_digits(stone);
                if digits % 2 == 0 {
                    let pow = (10 as SolutionType).pow(digits / 2);
                    let first = stone / pow;
                    let second = stone % pow;
                    *next.entry(first).or_insert(0) += n_stones;
                    *next.entry(second).or_insert(0) += n_stones;
                } else {
                    *next.entry(stone * 2024).or_insert(0) += n_stones;
                }
            }
        }
        (curr, next) = (next, curr);
        next.clear();
    }
    curr.values().map(|v| *v as SolutionType).sum()
}

#[aoc(day11, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    count(data, 25)
}

#[aoc(day11, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    count(data, 75)
}
