// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use advent_of_tools::*;
use std::str::FromStr;

type NumType = i32;
type InputType = Vec<(NumType, NumType)>;
type SolutionType = NumType;

const WIDTH: NumType = 71;

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> InputType {
    input
        .lines()
        .map(|line| {
            let mut line = line.split(",");
            (
                NumType::from_str(line.next().unwrap()).unwrap(),
                NumType::from_str(line.next().unwrap()).unwrap(),
            )
        })
        .collect()
}

#[aoc(day18, part1)]
pub fn solve_part1(data: &InputType) -> SolutionType {
    let mut map = Map::new(WIDTH, WIDTH);

    for (t, &(x, y)) in data.iter().enumerate() {
        if t < 1024 {
            let pos = Point { x, y };
            map.set_at(pos, b'#');
        }
    }
    map.print();
    let start = Point { x: 0, y: 0 };
    let end = Point {
        x: WIDTH - 1,
        y: WIDTH - 1,
    };

    map.bfs(start, end, &mut |_map, _p, d, c| {
        use Dir::{East, North, South, West};
        if matches!(d, North | South | East | West) && c == b'.' {
            Some(1)
        } else {
            None
        }
    })
}

///////////////////////////////////

#[aoc(day18, part2)]
pub fn solve_part2(data: &InputType) -> SolutionType {
    let mut map = Map::new(WIDTH, WIDTH);

    let mut byte_iter = data.iter();
    for _time in 0..1024 {
        let &(x, y) = data.iter().next().unwrap();
        let pos = Point { x, y };
        map.set_at(pos, b'#');
    }

    let start = Point { x: 0, y: 0 };
    let end = Point {
        x: WIDTH - 1,
        y: WIDTH - 1,
    };

    while let Some(&(x, y)) = byte_iter.next() {
        let last_pos = Point { x, y };
        map.set_at(last_pos, b'#');

        if map.bfs(start, end, &mut |_map, _p, d, c| {
            if d.is_cardinal() && c == b'.' {
                Some(1)
            } else {
                None
            }
        }) == 0
        {
            println!("Answer: {},{}", last_pos.x, last_pos.y);
            return 0;
        }
    }
    0
}
