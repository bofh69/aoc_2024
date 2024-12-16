// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use ahash::{HashMap, HashMapExt};
use ahash::{HashSet, HashSetExt};
use std::collections::BTreeSet;

use advent_of_tools::*;

type SolutionType = u32;

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Map {
    Map::from_string_with_border(input)
}

#[derive(Eq, PartialEq)]
struct ReindeerPos {
    cost: SolutionType,
    pos: Point,
    dir: Dir,
}

impl Ord for ReindeerPos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let to_num = |d| {
            use Dir::*;
            match d {
                North => 0,
                NorthEast => 1,
                East => 2,
                SouthEast => 3,
                South => 4,
                SouthWest => 5,
                West => 6,
                NorthWest => 7,
                _ => panic!("Unknown dir"),
            }
        };
        self.cost
            .cmp(&other.cost)
            .then(self.pos.cmp(&other.pos))
            .then(to_num(self.dir).cmp(&to_num(other.dir)))
    }
}

impl PartialOrd for ReindeerPos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(map: &Map) -> SolutionType {
    let start = map.find(b'S')[0];
    let end = map.find(b'E')[0];

    let mut visited = HashSet::new();
    let mut to_expand = BTreeSet::new();
    to_expand.insert(ReindeerPos {
        pos: start,
        dir: Dir::East,
        cost: 0,
    });

    while let Some(pos) = to_expand.pop_first() {
        if pos.pos == end {
            return pos.cost;
        }
        if visited.contains(&(pos.pos, pos.dir)) {
            continue;
        }
        to_expand.insert(ReindeerPos {
            pos: pos.pos,
            dir: pos.dir.turn_cardinal_left(),
            cost: pos.cost + 1000,
        });
        to_expand.insert(ReindeerPos {
            pos: pos.pos,
            dir: pos.dir.turn_cardinal_right(),
            cost: pos.cost + 1000,
        });
        let new_pos = pos.pos.walk(pos.dir);
        let c = map.get_at_unchecked(new_pos);
        if c == b'E' || c == b'.' {
            to_expand.insert(ReindeerPos {
                pos: new_pos,
                dir: pos.dir,
                cost: pos.cost + 1,
            });
        }
        visited.insert((pos.pos, pos.dir));
        /*
        map.print_with_overlay(|p, c| if p == pos.pos {
            Some(b'@')
        } else {
            Some(c)
        });
        */
    }
    0
}

fn count_paths(
    map: &Map,
    pos: ReindeerPos,
    max_cost: u32,
    points: &mut HashSet<Point>,
    visited: &mut HashMap<(Point, Dir), u32>,
) -> bool {
    if pos.cost > max_cost {
        return false;
    }

    let new_pos = pos.pos.walk(pos.dir);
    let c = map.get_at_unchecked(new_pos);

    if c == b'E' {
        points.insert(pos.pos);
        return true;
    }

    let old_cost = visited.entry((pos.pos, pos.dir)).or_insert(pos.cost);
    if *old_cost < pos.cost {
        return false;
    }

    let mut possible = false;
    if c != b'#' {
        possible = count_paths(
            map,
            ReindeerPos {
                pos: new_pos,
                dir: pos.dir,
                cost: pos.cost + 1,
            },
            max_cost,
            points,
            visited,
        ) || possible;
    }
    possible = count_paths(
        map,
        ReindeerPos {
            pos: pos.pos,
            dir: pos.dir.turn_cardinal_left(),
            cost: pos.cost + 1000,
        },
        max_cost,
        points,
        visited,
    ) || possible;
    possible = count_paths(
        map,
        ReindeerPos {
            pos: pos.pos,
            dir: pos.dir.turn_cardinal_right(),
            cost: pos.cost + 1000,
        },
        max_cost,
        points,
        visited,
    ) || possible;

    if possible {
        points.insert(pos.pos);
    }

    possible
}

#[aoc(day16, part2)]
pub fn solve_part2(map: &Map) -> SolutionType {
    let max_cost = solve_part1(map);
    let start = map.find(b'S')[0];

    let mut points = HashSet::new();
    let mut visited = HashMap::new();

    count_paths(
        map,
        ReindeerPos {
            pos: start,
            dir: Dir::East,
            cost: 0,
        },
        max_cost,
        &mut points,
        &mut visited,
    );

    map.print_with_overlay(|p, c| {
        if points.contains(&p) {
            Some(b'O')
        } else {
            Some(c)
        }
    });

    points.len() as SolutionType + 1
}
