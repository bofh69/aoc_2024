// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

// use advent_of_tools::*;

use std::str::FromStr;
// use ahash::{HashMap, HashMapExt};
// use ahash::{HashSet, HashSetExt};
// use rayon::prelude::*;

type NumType = u32;
type InputType = Vec<NumType>;
type SolutionType = u64;

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
    ((n2.overflowing_mul(2048)).0 ^ n2) & ((1 << 24) - 1)
}

#[aoc(day22, part1)]
pub fn solve_part1(data: &InputType) -> SolutionType {
    data.iter()
        .map(|&n| {
            let mut n = n;
            for _ in 0..2000 {
                n = next(n);
            }
            n as SolutionType
        })
        .sum()
}

#[aoc(day22, part2)]
pub fn solve_part2(data: &InputType) -> SolutionType {
    let mut prices = [0u16; 19 * 19 * 19 * 19];

    data.iter().for_each(|&n| {
        let mut n = n;
        let old_price = n % 10;
        n = next(n);
        let price = n % 10;
        let mut old_nums = [0u8; 4];
        old_nums[2] = (price + 9 - old_price) as u8;
        n = next(n);
        let old_price = price;
        let price = n % 10;
        old_nums[1] = (price + 9 - old_price) as u8;
        n = next(n);
        let old_price = price;
        let mut price = n % 10;
        old_nums[0] = (price + 9 - old_price) as u8;

        let mut first = [true; 19 * 19 * 19 * 19];
        for _ in 3..2000 {
            n = next(n);
            let old_price = price;
            price = n % 10;
            let diff = (price + 9 - old_price) as u8;
            let idx = old_nums[2] as usize * 19 * 19 * 19
                + old_nums[1] as usize * 19 * 19
                + old_nums[0] as usize * 19
                + diff as usize;
            if first[idx] {
                prices[idx] += price as u16;
                first[idx] = false;
            }
            old_nums[2] = old_nums[1];
            old_nums[1] = old_nums[0];
            old_nums[0] = diff;
        }
    });
    prices.iter().copied().max().unwrap() as SolutionType
}
