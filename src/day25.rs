// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use advent_of_tools::*;

// use rayon::prelude::*;

type InputType = Vec<Map>;
type SolutionType = usize;

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> InputType {
    input.split("\n\n").map(Map::from_string).collect()
}

#[aoc(day25, part1)]
pub fn solve_part1(data: &InputType) -> SolutionType {
    let locks: Vec<_> = data
        .iter()
        .filter(|m| m.get_at_unchecked(Point { x: 0, y: 0 }) == b'#')
        .map(|m| {
            let mut result = Vec::new();
            'x: for x in 0..m.get_width() {
                for y in 0..m.get_height() {
                    if m.get_at_unchecked(Point { x, y }) == b'.' {
                        result.push(y - 1);
                        continue 'x;
                    }
                }
                panic!("No end to lock: {m:?}");
            }
            result
        })
        .collect();
    let keys: Vec<_> = data
        .iter()
        .filter(|m| m.get_at_unchecked(Point { x: 0, y: 0 }) == b'.')
        .map(|m| {
            let mut result = Vec::new();
            'x: for x in 0..m.get_width() {
                for y in 0..m.get_height() {
                    let y1 = m.get_height() - y - 1;
                    if m.get_at_unchecked(Point { x, y: y1 }) == b'.' {
                        result.push(y1 + 1);
                        continue 'x;
                    }
                }
                panic!("No end to key: {m:?}");
            }
            result
        })
        .collect();
    let mut result = 0;
    for lock in locks {
        'key: for key in &keys {
            let mut max = 0;
            for x in 0..key.len() {
                max = max.max(key[x] + lock[x]);
            }
            for x in 0..key.len() {
                if key[x] <= lock[x] {
                    continue 'key;
                }
            }
            result += 1;
        }
    }
    result
}
