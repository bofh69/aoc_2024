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
    let prices: Vec<([i8; 2000], [i8; 2000])> = data
        .iter()
        .map(|&n| {
            let mut n = n;
            let mut prices = [0i8; 2000];
            let mut price_diffs = [0i8; 2000];
            for i in 0..2000 {
                let old_price = (n % 10) as i8;
                n = next(n);
                prices[i] = (n % 10) as i8;
                price_diffs[i] = (n % 10) as i8 - old_price;
            }
            (prices, price_diffs)
        })
        .collect();
    let mut max_bananas = 0;
    for i in -9..=9 {
        println!("Checking {i}");
        for j in -9..=9 {
            for k in -9..=9 {
                for l in -9..=9 {
                    let mut bananas = 0;
                    for monkey_prices in &prices {
                        for m in 3..2000 {
                            if monkey_prices.1[m - 3] == i
                                && monkey_prices.1[m - 2] == j
                                && monkey_prices.1[m - 1] == k
                                && monkey_prices.1[m] == l
                            {
                                bananas += monkey_prices.0[m] as NumType;
                                break;
                            }
                        }
                    }
                    if bananas > max_bananas {
                        println!("For {i} {j} {k} {l}, got {bananas}");
                    }
                    max_bananas = max_bananas.max(bananas);
                }
            }
        }
    }
    max_bananas
}
