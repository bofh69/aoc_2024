// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use std::str::FromStr;

type InputType = Vec<SolutionType>;
type SolutionType = i32;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|s| SolutionType::from_str(s).unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn is_increasing(report: &[SolutionType]) -> bool {
    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    return true;
}

fn is_decreasing(report: &[SolutionType]) -> bool {
    for i in 1..report.len() {
        let diff = report[i - 1] - report[i];
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    return true;
}

#[aoc(day2, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    data.iter()
        .filter(|report| is_increasing(report) || is_decreasing(report))
        .count() as SolutionType
}

// ------------------------------------------------------------

fn is_increasing2(report: &[SolutionType], skip: usize) -> bool {
    let mut last = 0;
    if skip == 0 {
        last = 1;
    }
    for i in last + 1..report.len() {
        if i == skip {
            continue;
        }
        let diff = report[i] - report[last];
        if diff < 1 || diff > 3 {
            return false;
        }
        last = i;
    }
    return true;
}

fn is_decreasing2(report: &[SolutionType], skip: usize) -> bool {
    let mut last = 0;
    if skip == 0 {
        last = 1;
    }
    for i in last + 1..report.len() {
        if i == skip {
            continue;
        }
        let diff = report[last] - report[i];
        if diff < 1 || diff > 3 {
            return false;
        }
        last = i;
    }
    return true;
}

#[aoc(day2, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    data.iter()
        .filter(|report| {
            for i in 0..report.len() {
                if is_increasing2(report, i) {
                    return true;
                }
                if is_decreasing2(report, i) {
                    return true;
                }
            }
            false
        })
        .count() as SolutionType
}
