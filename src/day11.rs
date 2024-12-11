// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashMap;
// use std::collections::LinkedList;
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

fn count_(mem: &mut HashMap<(InputType, u8), SolutionType>, n: InputType, gen: u8) -> SolutionType {
    if gen == 0 {
        return 1;
    }
    if let Some(res) = mem.get(&(n, gen)) {
        return *res;
    }
    let res = if n == 0 {
        count_(mem, 1, gen - 1)
    } else {
        let digits = n.ilog10() + 1;
        if digits % 2 == 0 {
            let pow = (10 as SolutionType).pow(digits / 2);
            let first = n / pow;
            let second = n % pow;
            count_(mem, first, gen - 1) + count_(mem, second, gen - 1)
        } else {
            count_(mem, n * 2024, gen - 1)
        }
    };
    mem.insert((n, gen), res);
    res
}

fn count(n: InputType, gen: u8) -> SolutionType {
    let mut mem = HashMap::new();

    count_(&mut mem, n, gen)
}

#[aoc(day11, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    data.iter().map(|n| count(*n, 75)).sum()
}
