// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

// use std::collections::HashSet;
use std::str::FromStr;

type NumType = u64;
type InputType = Vec<(NumType, Vec<NumType>)>;
type SolutionType = NumType;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> InputType {
    input
        .lines()
        .map(|line| {
            let mut line = line.split(": ");
            let num = NumType::from_str(line.next().unwrap()).unwrap();
            (
                num,
                line.next()
                    .unwrap()
                    .split(" ")
                    .map(|s| NumType::from_str(s).unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn solvable(result: NumType, acc: NumType, nums: &[NumType]) -> bool {
    if nums.is_empty() {
        return acc == result;
    }

    let n = nums[0];
    let nums = &nums[1..];

    if solvable(result, acc * n, nums) || solvable(result, acc + n, nums) {
        return true;
    }

    false
}

#[aoc(day7, part1)]
pub fn solve_part1(data: &InputType) -> SolutionType {
    data.iter()
        .filter(|&(result, ref nums)| solvable(*result, nums[0], &nums[1..]))
        .map(|&(result, _)| result)
        .sum()
}

//////////////////////////////////////////

fn solvable2(result: NumType, acc: NumType, nums: &[NumType]) -> bool {
    if nums.is_empty() {
        return acc == result;
    }
    // Assumes no zeros in nums:
    if acc > result {
        return false;
    }

    let n = nums[0];
    let nums = &nums[1..];

    if solvable2(result, acc * n, nums) {
        return true;
    }
    if solvable2(result, acc + n, nums) {
        return true;
    }
    let mut acc2 = acc;
    let mut n2 = n;
    while n2 > 0 {
        acc2 *= 10;
        n2 /= 10;
    }
    acc2 += n;
    if solvable2(result, acc2, nums) {
        return true;
    }

    false
}

#[aoc(day7, part2)]
pub fn solve_part2(data: &InputType) -> SolutionType {
    data.iter()
        .filter(|&(result, ref nums)| solvable2(*result, nums[0], &nums[1..]))
        .map(|&(result, _)| result)
        .sum()
}
