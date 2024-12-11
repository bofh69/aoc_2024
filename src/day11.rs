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
        let mut digits = 1;
        let mut num = n;
        while num >= 10 {
            num /= 10;
            digits += 1;
        }
        if digits % 2 == 0 {
            // Even numbers
            let mut first = n;
            let mut second = 0;
            let mut fac = 1;
            digits /= 2;
            while digits > 0 {
                second += (first % 10) * fac;
                first /= 10;
                fac *= 10;
                digits -= 1;
            }
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
