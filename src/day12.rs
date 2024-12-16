// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

// use std::collections::HashSet;

use advent_of_tools::*;

type SolutionType = i32;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Map {
    Map::from_string_with_border(input)
}

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

#[aoc(day12, part1)]
pub fn solve_part1(first_map: &Map) -> SolutionType {
    let mut map = first_map.clone();

    let mut price = 0;
    for y in 1..map.get_height() - 1 {
        for x in 1..map.get_width() - 1 {
            let pos = Point { x, y };
            let c = map.get_at_unchecked(pos);
            if (c as char).is_ascii_alphabetic() {
                let (area, perimeter) = flood_cardinal(&mut map, first_map, pos, c, b'*');
                price += area * perimeter;
                /*
                println!(
                    "{}: area={area}, perimeter={perimeter}, price={}",
                    c as char,
                    area * perimeter
                );
                */
            }
        }
    }
    price
}

fn count_sides(map: &Map, start: Point) -> SolutionType {
    let mut sides = 0;
    use Dir::*;
    for &dir in &[North, South] {
        for y in start.y..map.get_height() - 1 {
            let mut was_on_side = false;
            for x in 1..map.get_width() {
                let pos = Point { x, y };
                if map.get_at_unchecked(pos) == b'!' {
                    let pos_outside = pos.walk(dir);
                    if map.get_at_unchecked(pos_outside) != b'!' {
                        if !was_on_side {
                            sides += 1;
                        }
                        was_on_side = true;
                    } else {
                        was_on_side = false;
                    }
                } else {
                    was_on_side = false;
                }
            }
        }
    }
    for &dir in &[East, West] {
        for x in 1..map.get_width() {
            let mut was_on_side = false;
            for y in start.y..map.get_height() - 1 {
                let pos = Point { x, y };
                if map.get_at_unchecked(pos) == b'!' {
                    let pos_outside = pos.walk(dir);
                    if map.get_at_unchecked(pos_outside) != b'!' {
                        if !was_on_side {
                            sides += 1;
                        }
                        was_on_side = true;
                    } else {
                        was_on_side = false;
                    }
                } else {
                    was_on_side = false;
                }
            }
        }
    }
    sides
}

#[aoc(day12, part2)]
pub fn solve_part2(first_map: &Map) -> SolutionType {
    let mut map = first_map.clone();

    let mut price = 0;
    for y in 1..map.get_height() - 1 {
        for x in 1..map.get_width() - 1 {
            let pos = Point { x, y };
            let c = map.get_at_unchecked(pos);
            if (c as char).is_ascii_alphabetic() {
                flood_cardinal(&mut map, first_map, pos, c, b'!');
                let sides = count_sides(&map, pos);
                let (area, _perimeter) = flood_cardinal(&mut map, first_map, pos, b'!', b'*');

                price += area * sides;
                /*
                println!(
                    "{}: area={area}, sides={sides}, price={}",
                    c as char,
                    area * sides
                );
                */
            }
        }
    }
    price
}
