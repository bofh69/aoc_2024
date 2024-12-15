// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

// use std::collections::HashSet;

use advent_of_tools::*;

type SolutionType = i32;
type InputType = (Map, Vec<Dir>);

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> InputType {
    let mut iter = input.split("\n\n");
    let map = Map::from_string(iter.next().unwrap());
    let path = iter
        .next()
        .unwrap()
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| match c {
            '^' => Dir::North,
            'v' => Dir::South,
            '>' => Dir::East,
            '<' => Dir::West,
            _ => panic!("Unknown char: ''{c}''"),
        })
        .collect();
    (map, path)
}

/*
pub fn count_perimeters(map: &Map, pos: Point, c: u8) -> SolutionType {
    use Dir::*;
    let mut perimeters = 0;
    for &dir in &[North, South, East, West] {
        let pos = pos.walk(dir);
        let c2 = map.get_at_unchecked(pos);
        if c2 != c {
            perimeters += 1;
        }
    }
    perimeters
}

pub fn flood_cardinal(
    map: &mut Map,
    original: &Map,
    pos: Point,
    empty: u8,
    val: u8,
) -> (SolutionType, SolutionType) {
    if map.get_at_unchecked(pos) != empty {
        // Nothing to fill here
        return (0, 0);
    }
    let mut area = 0;
    let mut perimeter = 0;
    let min_pos = map.walk_until(pos, Dir::West, |_, c| c != empty);
    let max_pos = map.walk_until(pos, Dir::East, |_, c| c != empty);

    let mut pos = min_pos;
    while pos.x <= max_pos.x {
        map.set_at(pos, val);
        area += 1;
        perimeter += count_perimeters(original, pos, empty);
        pos = pos.walk(Dir::East);
    }
    pos = min_pos;
    while pos.x <= max_pos.x {
        pos.y -= 1;
        if pos.y > 0 {
            let (a, p) = flood_cardinal(map, original, pos, empty, val);
            area += a;
            perimeter += p;
        }
        pos.y += 2;
        if pos.y < map.get_height() {
            let (a, p) = flood_cardinal(map, original, pos, empty, val);
            area += a;
            perimeter += p;
        }
        pos.y -= 1;
        pos = pos.walk(Dir::East);
    }
    (area, perimeter)
}
*/

fn push_crate(map: &mut Map, pos: Point, dir: Dir) -> bool {
    let mut next_pos = pos;
    loop {
        match map.get_at_unchecked(next_pos) {
            b'O' => (),
            b'.' => {
                map.set_at(pos, b'.');
                map.set_at(next_pos, b'O');
                return true;
            }
            _ => return false,
        }
        next_pos = next_pos.walk(dir);
    }
}

#[aoc(day15, part1)]
pub fn solve_part1(data: &InputType) -> SolutionType {
    let mut map = data.0.clone();
    let mut pos = map.find(b'@')[0];
    map.set_at(pos, b'.');
    for &dir in &data.1 {
        let new_pos = pos.walk(dir);
        match map.get_at_unchecked(new_pos) {
            b'.' => pos = new_pos,
            b'#' => (),
            b'O' => {
                if push_crate(&mut map, new_pos, dir) {
                    pos = new_pos
                }
            }
            c => panic!("Unknown char ''{c}''"),
        }
    }
    map.print_with_overlay(|p, c| if p == pos { Some(b'@') } else { Some(c) });
    map.find(b'O').iter().map(|p| p.y * 100 + p.x).sum()
}

/* ////////////////////////// */

fn can_move_crates(map: &mut Map, pos: Point, dir: Dir) -> bool {
    // Only handles North/South, pos is the crate to move.

    let mut pos = pos;
    if map.get_at_unchecked(pos) == b']' {
        // Align pos:
        pos = pos.walk(Dir::West);
    }

    let pos2 = pos.walk(Dir::East);
    let pos3 = pos.walk(dir);
    let pos4 = pos2.walk(dir);
    let c3 = map.get_at_unchecked(pos3);
    let c4 = map.get_at_unchecked(pos4);

    if c3 == b'#' || c4 == b'#' {
        return false;
    }

    if (c3 == b']' || c3 == b'[') && !can_move_crates(map, pos3, dir) {
        return false;
    }
    if c4 == b'[' && !can_move_crates(map, pos4, dir) {
        return false;
    }

    true
}

fn move_crates(map: &mut Map, pos: Point, dir: Dir) {
    // Only handles North/South, pos is the crate to move.
    // The crates is already verified that they can move.

    let mut pos = pos;
    if map.get_at_unchecked(pos) == b']' {
        // Align pos:
        pos = pos.walk(Dir::West);
    }

    let pos2 = pos.walk(Dir::East);
    let pos3 = pos.walk(dir);
    let pos4 = pos2.walk(dir);
    let c3 = map.get_at_unchecked(pos3);
    let c4 = map.get_at_unchecked(pos4);

    if c3 == b'#' || c4 == b'#' {
        return;
    }
    if c3 == b'[' || c3 == b']' {
        move_crates(map, pos3, dir);
    }
    if c4 == b'[' {
        // If c4 == ], it was moved above
        move_crates(map, pos4, dir);
    }
    map.set_at(pos3, b'[');
    map.set_at(pos4, b']');
    map.set_at(pos, b'.');
    map.set_at(pos2, b'.');
}

fn push_big_crate(map: &mut Map, pos: Point, dir: Dir) -> bool {
    if dir == Dir::East || dir == Dir::West {
        let mut next_pos = pos;
        loop {
            match map.get_at_unchecked(next_pos) {
                b'[' | b']' => (),
                b'.' => {
                    while next_pos != pos {
                        let from_pos = next_pos.walk(dir.turn_cardinal_left().turn_cardinal_left());
                        let c = map.get_at_unchecked(from_pos);
                        map.set_at(next_pos, c);
                        next_pos = from_pos;
                    }
                    map.set_at(pos, b'.');
                    return true;
                }
                _ => return false,
            }
            next_pos = next_pos.walk(dir);
        }
    } else if can_move_crates(map, pos, dir) {
        move_crates(map, pos, dir);
        return true;
    }
    false
}

#[aoc(day15, part2)]
pub fn solve_part2(data: &InputType) -> SolutionType {
    let mut map = Map::new(data.0.get_width() * 2, data.0.get_height());
    for (pos, tile) in data.0.iter() {
        let pos1 = Point {
            x: pos.x * 2,
            y: pos.y,
        };
        let pos2 = Point {
            x: pos.x * 2 + 1,
            y: pos.y,
        };
        match tile {
            b'@' | b'.' => {
                map.set_at(pos1, b'.');
                map.set_at(pos2, b'.');
            }
            b'O' => {
                map.set_at(pos1, b'[');
                map.set_at(pos2, b']');
            }
            b'#' => {
                map.set_at(pos1, b'#');
                map.set_at(pos2, b'#');
            }
            _ => panic!("Unknown char"),
        }
    }
    let mut pos = data.0.find(b'@')[0];
    pos.x *= 2;
    for &dir in &data.1 {
        let new_pos = pos.walk(dir);
        match map.get_at_unchecked(new_pos) {
            b'.' => pos = new_pos,
            b'#' => (),
            b'[' | b']' => {
                if push_big_crate(&mut map, new_pos, dir) {
                    pos = new_pos
                }
            }
            c => panic!("Unknown char ''{c}''"),
        }
    }
    map.print_with_overlay(|p, c| if p == pos { Some(b'@') } else { Some(c) });
    map.find(b'[').iter().map(|p| p.y * 100 + p.x).sum()
}
