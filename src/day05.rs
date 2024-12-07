// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashSet;
use std::str::FromStr;

type NumType = u32;
type InputType = (HashSet<(NumType, NumType)>, Vec<Vec<NumType>>);
type SolutionType = NumType;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> InputType {
    let mut iter = input.lines();
    let mut result1 = HashSet::new();
    let mut result2 = Vec::new();
    for line in iter.by_ref() {
        if line.is_empty() {
            break;
        }
        let mut line = line.split("|").map(|s| NumType::from_str(s).unwrap());
        result1.insert((line.next().unwrap(), line.next().unwrap()));
    }
    for line in iter {
        let line: Vec<_> = line
            .split(",")
            .map(|s| NumType::from_str(s).unwrap())
            .collect();
        result2.push(line);
    }
    (result1, result2)
}

fn is_in_order(order: &HashSet<(NumType, NumType)>, update: &[NumType]) -> bool {
    for i in 0..update.len() {
        let first = update[i];
        for &second in update.iter().skip(i) {
            if order.contains(&(second, first)) {
                return false;
            }
        }
    }
    true
}

#[aoc(day5, part1)]
pub fn solve_part1(data: &InputType) -> SolutionType {
    data.1
        .iter()
        .filter(|update| is_in_order(&data.0, update))
        .map(|update| {
            let middle = update.len() / 2;
            update[middle]
        })
        .sum()
}

fn is_path(order: &HashSet<(NumType, NumType)>, a: NumType, b: NumType) -> bool {
    if order.contains(&(a, b)) {
        return true;
    }
    if order.contains(&(b, a)) {
        return false;
    }
    for rule in order {
        if rule.0 == a {
            let x = rule.1;
            if is_path(order, x, b) {
                return true;
            }
        }
    }
    false
}

#[aoc(day5, part2)]
pub fn solve_part2(data: &InputType) -> SolutionType {
    data.1
        .iter()
        .filter(|update| !is_in_order(&data.0, update))
        .map(|update| {
            let mut update = update.clone();
            update.sort_by(|&a, &b| {
                use std::cmp::Ordering;
                if is_path(&data.0, a, b) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            let middle = update.len() / 2;
            update[middle] as SolutionType
        })
        .sum()
}
