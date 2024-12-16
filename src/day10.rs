// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use ahash::{HashSet, HashSetExt};

use advent_of_tools::*;

type SolutionType = i32;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Map {
    Map::from_string_with_border(input)
}

fn count_reachable_goals(map: &Map, start: Point) -> SolutionType {
    let mut goals = HashSet::new();

    let mut visited = HashSet::new();
    let mut to_expand = Vec::new();
    to_expand.push((start, b'0'));

    while let Some((pos, height)) = to_expand.pop() {
        use Dir::*;
        for dir in [North, South, East, West] {
            let pos = pos.walk(dir);
            let c = map.get_at_unchecked(pos);
            if c == height + 1 && visited.insert(pos) {
                if c == b'9' {
                    goals.insert(pos);
                } else {
                    to_expand.push((pos, c));
                }
            }
        }
    }

    goals.len() as SolutionType
}

#[aoc(day10, part1)]
pub fn solve_part1(map: &Map) -> SolutionType {
    let starts = map.find(b'0');
    starts
        .iter()
        .map(|&pos| count_reachable_goals(map, pos))
        .sum()
}

fn count_rate_of_trails(map: &Map, start: Point) -> SolutionType {
    let mut to_expand = Vec::new();
    to_expand.push((start, b'0'));

    let mut rating = 0;
    while let Some((pos, height)) = to_expand.pop() {
        use Dir::*;
        for dir in [North, South, East, West] {
            let pos = pos.walk(dir);
            let c = map.get_at_unchecked(pos);
            if c == height + 1 {
                if c == b'9' {
                    rating += 1;
                } else {
                    to_expand.push((pos, c));
                }
            }
        }
    }

    rating
}

#[aoc(day10, part2)]
pub fn solve_part2(map: &Map) -> SolutionType {
    let starts = map.find(b'0');
    starts
        .iter()
        .map(|&pos| count_rate_of_trails(map, pos))
        .sum()
}
