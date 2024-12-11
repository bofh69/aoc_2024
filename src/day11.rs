// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use ahash::{HashMap, HashMapExt};
use memoize::memoize;
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

#[aoc(day11, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    data.iter().map(|n| count(*n, 25)).sum()
}

#[memoize(CustomHasher: HashMap)]
fn count(n: InputType, gen: u8) -> SolutionType {
    if gen == 0 {
        return 1;
    }

    if n == 0 {
        count(1, gen - 1)
    } else {
        let digits = n.ilog10() + 1;
        if digits % 2 == 0 {
            let pow = (10 as SolutionType).pow(digits / 2);
            let first = n / pow;
            let second = n % pow;
            count(first, gen - 1) + count(second, gen - 1)
        } else {
            count(n * 2024, gen - 1)
        }
    }
}

#[aoc(day11, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    data.iter().map(|n| count(*n, 75)).sum()
}
