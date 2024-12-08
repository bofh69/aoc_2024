// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashSet;

use advent_of_tools::*;
// use rayon::prelude::*;

type SolutionType = i32;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Map {
    Map::from_string(input)
}

#[aoc(day8, part1)]
pub fn solve_part1(map: &Map) -> SolutionType {
    let map = map.clone();
    let types = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut antinodes = HashSet::new();
    for &antenna_type in types {
        let antennas = map.find(antenna_type);
        if antennas.len() > 1 {
            for (i, antenna1) in antennas.iter().enumerate() {
                for antenna2 in antennas.iter().skip(i + 1) {
                    let diff = Point {
                        x: antenna2.x - antenna1.x,
                        y: antenna2.y - antenna1.y,
                    };
                    let antinode1 = Point {
                        x: antenna1.x - diff.x,
                        y: antenna1.y - diff.y,
                    };
                    let antinode2 = Point {
                        x: antenna2.x + diff.x,
                        y: antenna2.y + diff.y,
                    };
                    if map.is_inside_map(antinode1) {
                        antinodes.insert(antinode1);
                    }
                    if map.is_inside_map(antinode2) {
                        antinodes.insert(antinode2);
                    }
                }
            }
        }
    }
    map.print_with_overlay(|pos, c| {
        if c == b'.' && antinodes.contains(&pos) {
            Some(b'#')
        } else {
            None
        }
    });
    antinodes.len() as SolutionType
}

#[aoc(day8, part2)]
pub fn solve_part2(map: &Map) -> SolutionType {
    let map = map.clone();
    let types = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut antinodes = HashSet::new();
    for &antenna_type in types {
        let antennas = map.find(antenna_type);
        if antennas.len() > 1 {
            for (i, antenna1) in antennas.iter().enumerate() {
                for antenna2 in antennas.iter().skip(i + 1) {
                    let diff = Point {
                        x: antenna2.x - antenna1.x,
                        y: antenna2.y - antenna1.y,
                    };
                    let mut antinode = *antenna2;
                    loop {
                        antinode = Point {
                            x: antinode.x - diff.x,
                            y: antinode.y - diff.y,
                        };
                        if map.is_inside_map(antinode) {
                            antinodes.insert(antinode);
                        } else {
                            break;
                        }
                    }
                    let mut antinode = *antenna1;
                    loop {
                        antinode = Point {
                            x: antinode.x + diff.x,
                            y: antinode.y + diff.y,
                        };
                        if map.is_inside_map(antinode) {
                            antinodes.insert(antinode);
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    map.print_with_overlay(|pos, c| {
        if c == b'.' && antinodes.contains(&pos) {
            Some(b'#')
        } else {
            None
        }
    });
    antinodes.len() as SolutionType
}
