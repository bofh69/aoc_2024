// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use advent_of_tools::*;

type SolutionType = usize;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Map {
    Map::from_string_with_border(input)
}

fn is_xmas(map: &Map, pos: Point, dir: Dir) -> bool {
    let mut pos = pos;
    for c in [b'M', b'A', b'S'] {
        pos = pos.walk(dir);
        if map.get_at_unchecked(pos) != c {
            return false;
        }
    }
    true
}

#[aoc(day4, part1)]
pub fn solve_part1(map: &Map) -> SolutionType {
    let mut count = 0;
    for y in 1..map.get_height() - 1 {
        for x in 1..map.get_width() - 1 {
            let pos = Point { x, y };
            if map.get_at_unchecked(pos) == b'X' {
                let mut dir = Dir::North;
                for _ in 0..8 {
                    if is_xmas(map, pos, dir) {
                        count += 1;
                    }
                    dir = dir.turn_right();
                }
            }
        }
    }
    count
}

fn is_mas(map: &Map, pos: Point, dir1: Dir, dir2: Dir) -> bool {
    let pos1 = pos.walk(dir1);
    let pos2 = pos.walk(dir2);
    let c1 = map.get_at_unchecked(pos1);
    let c2 = map.get_at_unchecked(pos2);

    if (c1 == b'M' && c2 == b'S') || (c2 == b'M' && c1 == b'S') {
        return true;
    }
    false
}

#[aoc(day4, part2)]
pub fn solve_part2(map: &Map) -> SolutionType {
    let mut count = 0;
    for y in 1..map.get_height() - 1 {
        for x in 1..map.get_width() - 1 {
            let pos = Point { x, y };
            if map.get_at_unchecked(pos) == b'A'
                && is_mas(map, pos, Dir::NorthEast, Dir::SouthWest)
                && is_mas(map, pos, Dir::SouthEast, Dir::NorthWest)
            {
                count += 1;
            }
        }
    }
    count
}
