// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

// use ahash::{HashSet, HashSetExt};

use advent_of_tools::*;

type InputType = Map;
type SolutionType = i32;

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> InputType {
    Map::from_string_with_border(input)
}

#[aoc(day20, part1)]
pub fn solve_part1(map: &InputType) -> SolutionType {
    let start = map.find(b'S')[0];
    let end = map.find(b'E')[0];
    let time = map.bfs(start, end, &mut |_m, _p, d, c| {
        if d.is_cardinal() && matches!(c, b'E' | b'.') {
            Some(1)
        } else {
            None
        }
    });
    let mut result = 0;
    let max_time = time - 100;
    for cheat_pos in map.find(b'#') {
        if cheat_pos.x == 0
            || cheat_pos.y == 0
            || cheat_pos.x == map.get_width() - 1
            || cheat_pos.y == map.get_height() - 1
        {
            continue;
        }
        let new_time = map.bfs(start, end, &mut |_m, p, d, c| {
            if !d.is_cardinal() {
                return None;
            }
            if matches!(c, b'E' | b'.') {
                return Some(1);
            }
            if p == cheat_pos {
                return Some(1);
            }
            None
        });
        if new_time <= max_time {
            result += 1
        }
    }
    result
}

/*
fn print_type_of<T>(_: &T)
{
    println!("{}", std::any::type_name::<T>());
}
*/

#[aoc(day20, part2)]
pub fn solve_part2(map: &InputType) -> SolutionType {
    /*
    let mut steps_from_start = Vec::new();
    steps_from_start.resize((map.get_width() * map.get_height()) as usize, 0);
    let start = map.find(b'S')[0];
    let end = map.find(b'E')[0];
    */

    // TODO: Dijkstra from end

    map.get_width() as SolutionType
}
