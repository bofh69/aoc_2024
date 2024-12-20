// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use ahash::{HashSet, HashSetExt};

use advent_of_tools::*;

type InputType = Map;
type SolutionType = usize;

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> InputType {
    Map::from_string(input)
}

#[aoc(day20, part1)]
pub fn solve_part1(map: &InputType) -> SolutionType {
    let start = map.find(b'S')[0];
    let end = map.find(b'E')[0];
    let steps_from_start = find_steps_from(map, start);
    let steps_to_end = find_steps_from(map, end);

    let start_idx = get_idx(map, start);
    let max_time = steps_to_end[start_idx].checked_sub(100).unwrap();
    let mut result = 0;
    for (pos, c) in map.iter() {
        let pos_idx = get_idx(map, pos);
        if c == b'#'
            && steps_from_start[pos_idx] as u32 + steps_to_end[pos_idx] as u32 - 1
                <= max_time as u32
        {
            result += 1;
        }
    }
    result
}

fn get_idx(map: &Map, pos: Point) -> usize {
    (pos.x + pos.y * map.get_width()) as usize
}

fn find_steps_from(map: &Map, from: Point) -> Vec<u16> {
    let mut frontier = std::collections::VecDeque::new();

    let mut costs = Vec::new();
    costs.resize((map.get_height() * map.get_width()) as usize, u16::MAX);
    frontier.push_front((from, 0));
    while let Some((pos, steps)) = frontier.pop_back() {
        if !matches!(map.get_at_unchecked(pos), b'.' | b'S' | b'E') {
            continue;
        }
        let idx = get_idx(map, pos);
        if costs[idx] <= steps {
            continue;
        }
        costs[idx] = steps;
        // if map.get_at_unchecked(pos) == b'S' {
        // println!("Start: pos={pos:?}, idx={idx}, steps={steps}");
        // }
        for dir in [Dir::North, Dir::East, Dir::South, Dir::West] {
            let new_pos = pos.walk(dir);
            frontier.push_front((new_pos, steps + 1));
        }
    }
    for (pos, c) in map.iter() {
        if c == b'#'
            && pos.x > 0
            && pos.y > 0
            && pos.x < map.get_width()
            && pos.y < map.get_height()
        {
            let mut cheapest = u16::MAX;
            for (p, d, c) in map.neighbors(pos) {
                if !d.is_cardinal() {
                    continue;
                }
                if matches!(c, b'.' | b'S' | b'E') {
                    let idx = get_idx(map, p);
                    cheapest = cheapest.min(costs[idx])
                }
            }
            let pos_idx = get_idx(map, pos);
            costs[pos_idx] = 1 + cheapest;
        }
    }
    costs
}

#[aoc(day20, part2)]
pub fn solve_part2(map: &InputType) -> SolutionType {
    /*
     * Cheats start and end at 'S' and '.' tiles and end at '.' and 'E' tiles.
     */

    let start = map.find(b'S')[0];
    let end = map.find(b'E')[0];
    let steps_from_start = find_steps_from(map, start);
    let steps_to_end = find_steps_from(map, end);

    let start_idx = get_idx(map, start);
    let max_time = steps_to_end[start_idx] - 100;

    let mut total_cheats = 0;
    let mut cheats = HashSet::new();
    for (pos, c) in map.iter() {
        if matches!(c, b'.' | b'S')
            && pos.x > 0
            && pos.y > 0
            && pos.x < map.get_width()
            && pos.y < map.get_height()
        {
            let pos_idx = get_idx(map, pos);
            let pos_steps = steps_from_start[pos_idx];
            for d in [Dir::North, Dir::South, Dir::East, Dir::West] {
                let new_pos = pos.walk(d);
                let new_pos_idx = get_idx(map, new_pos);
                if map.get_at_unchecked(new_pos) != b'#' {
                    continue;
                }
                let new_steps = steps_from_start[new_pos_idx];

                if new_steps != pos_steps + 1 {
                    continue;
                }

                let m_d = new_pos.manhattan_distance(end);

                if new_steps + (m_d as u16) > max_time {
                    continue;
                }

                for dy in -19..=19i32 {
                    let y = dy + pos.y;
                    if y < 1 {
                        continue;
                    }
                    if y >= map.get_height() - 1 {
                        continue;
                    }
                    for dx in -19..=19i32 {
                        let m_d = (dx.abs() + dy.abs()) as u16;
                        if dx == 0 && dy == 0 || m_d > 20 {
                            continue;
                        }
                        let x = dx + pos.x;
                        if x < 1 {
                            continue;
                        }
                        if x >= map.get_width() - 1 {
                            continue;
                        }
                        let cheat_to_pos = Point { x, y };
                        let c = map.get_at_unchecked(cheat_to_pos);
                        if b'.' == c || b'E' == c {
                            let idx = (x + y * map.get_width()) as usize;
                            let total_steps = pos_steps + m_d + steps_to_end[idx];
                            if total_steps <= max_time {
                                cheats.insert(cheat_to_pos);
                                if pos.x == start.x && pos.y == start.y {
                                    println!("Cheat to {x},{y}, total_steps = {total_steps}");
                                }
                            }
                        }
                    }
                }
            }
            total_cheats += cheats.len();
            cheats.clear();
        }
    }

    assert!(total_cheats > 880417, "{total_cheats} should be > 880417");

    total_cheats
}
