// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

// use advent_of_tools::*;

use std::str::FromStr;
// use ahash::{HashMap, HashMapExt};
// use ahash::{HashSet, HashSetExt};
// use rayon::prelude::*;

type NumType = u64;
type InputType = Vec<NumType>;
type SolutionType = NumType;

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> InputType {
    input
        .lines()
        .map(|s| NumType::from_str(s).unwrap())
        .collect()
}

#[inline(always)]
fn next(n: NumType) -> NumType {
    let n1 = ((n * 64) ^ n) & ((1 << 24) - 1);
    let n2 = ((n1 / 32) ^ n1) & ((1 << 24) - 1);
    ((n2 * 2048) ^ n2) & ((1 << 24) - 1)
}

#[aoc(day22, part1)]
pub fn solve_part1(data: &InputType) -> SolutionType {
    data.iter()
        .map(|&n| {
            let mut n = n;
            for _ in 0..2000 {
                n = next(n);
            }
            n
        })
        .sum()
}

#[aoc(day22, part2)]
pub fn solve_part2(data: &InputType) -> SolutionType {
    let mut prices = [0; 19*19*19*19];

    data.iter().for_each(|&n| {
        let mut n = n;
        let old_price = n % 10;
        n = next(n);
        let price = n % 10;
        let mut idx = (price - old_price + 9) as usize;
        n = next(n);
        let old_price = price;
        let price = n % 10;
        idx = idx * 19 + (price - old_price + 9) as usize;
        n = next(n);
        let old_price = price;
        let mut price = n % 10;
        idx = idx * 19 + (price - old_price + 9) as usize;

        let mut first = [true; 19*19*19*19];
        for _ in 3..2000 {
            n = next(n);
            let old_price = price;
            price = n % 10;
            idx = (idx * 19 + (price - old_price + 9) as usize) % (19*19*19*19);
            if first[idx] {
                prices[idx] += price;
                first[idx] = false;
            }
        }
    });
    prices.iter().map(|n| *n).max().unwrap() as SolutionType
}
