// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashSet;

use advent_of_tools::*;

type SolutionType = i32;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Map {
    Map::from_string_with_border(input)
}

fn dir_legend(dir: Dir) -> u8 {
    match dir {
        Dir::North => b'^',
        Dir::East => b'>',
        Dir::West => b'<',
        Dir::South => b'v',
        _ => b'*',
    }
}

#[aoc(day6, part1)]
pub fn solve_part1(map: &Map) -> SolutionType {
    let mut map = map.clone();
    let mut pos = map.find(b'^')[0];
    let mut count = 1;
    let mut dir = Dir::North;
    loop {
        let new_pos = pos.walk(dir);
        match map.get_at_unchecked(new_pos) {
            b'>' | b'<' | b'v' | b'^' | b'.' => {
                if map.get_at_unchecked(new_pos) == b'.' {
                    count += 1;
                }
                map.set_at(pos, dir_legend(dir));
                pos = new_pos;
            }
            b'#' => {
                dir = dir.turn_cardinal_right();
                continue;
            }
            _ => {
                break;
            }
        }
    }
    // map.print();
    count
}

fn does_loop(map: &mut Map, pos: Point) -> bool {
    map.set_at(pos, b'O');

    let pos = map.find(b'^');
    if pos.is_empty() {
        return false;
    }
    let mut pos = pos[0];
    let mut dir = Dir::North;

    let mut visited = HashSet::new();

    loop {
        let new_pos = pos.walk(dir);
        if visited.contains(&(pos, dir)) {
            // map.print();
            return true;
        }
        visited.insert((pos, dir));
        // println!("{:?}", pos);
        // map.print();
        match map.get_at_unchecked(new_pos) {
            b'>' | b'<' | b'v' | b'^' | b'.' => {
                let legend = dir_legend(dir);
                map.set_at(pos, legend);
                pos = new_pos;
            }
            b'O' | b'#' => dir = dir.turn_cardinal_right(),
            _ => {
                break;
            }
        }
    }
    false
}

#[aoc(day6, part2)]
pub fn solve_part2(map: &Map) -> SolutionType {
    let mut original_map = map.clone();
    does_loop(&mut original_map, Point { x: 0, y: 0 });

    let mut count = 0;
    for y in 1..map.get_height() - 1 {
        for x in 1..map.get_width() - 1 {
            let pos = Point { x, y };
            match original_map.get_at_unchecked(pos) {
                b'^' | b'v' | b'<' | b'>' => {
                    let mut map = map.clone();
                    if does_loop(&mut map, pos) {
                        count += 1
                    }
                }
                _ => (),
            }
        }
    }
    count
}
