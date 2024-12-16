// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use ahash::{HashSet, HashSetExt};

use advent_of_tools::*;

type SolutionType = i32;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Map {
    Map::from_string(input)
}

/*
fn print_map(map: &Map, antinodes: &HashSet<Point>) {
    map.print_with_overlay(|pos, c| {
        if c == b'.' && antinodes.contains(&pos) {
            Some(b'#')
        } else {
            None
        }
    });
}
*/

#[aoc(day8, part1)]
pub fn solve_part1(map: &Map) -> SolutionType {
    let mut antinodes = HashSet::new();
    for &antenna_type in ALPHANUMS {
        let antennas = map.find(antenna_type);
        if antennas.len() > 1 {
            for (i, &antenna1) in antennas.iter().enumerate() {
                for &antenna2 in antennas.iter().skip(i + 1) {
                    let diff = antenna2 - antenna1;
                    let antinode1 = antenna1 - diff;
                    if map.is_inside_map(antinode1) {
                        antinodes.insert(antinode1);
                    }
                    let antinode2 = antenna2 + diff;
                    if map.is_inside_map(antinode2) {
                        antinodes.insert(antinode2);
                    }
                }
            }
        }
    }
    // print_map(map, &antinodes);
    antinodes.len() as SolutionType
}

#[aoc(day8, part2)]
pub fn solve_part2(map: &Map) -> SolutionType {
    let mut antinodes = HashSet::new();
    for &antenna_type in ALPHANUMS {
        let antennas = map.find(antenna_type);
        if antennas.len() > 1 {
            for (i, &antenna1) in antennas.iter().enumerate() {
                for &antenna2 in antennas.iter().skip(i + 1) {
                    let diff = antenna2 - antenna1;
                    let mut antinode = antenna2;
                    loop {
                        antinode = antinode - diff;
                        if map.is_inside_map(antinode) {
                            antinodes.insert(antinode);
                        } else {
                            break;
                        }
                    }
                    let mut antinode = antenna1;
                    loop {
                        antinode = antinode + diff;
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
    // print_map(map, &antinodes);
    antinodes.len() as SolutionType
}
