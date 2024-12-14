// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use advent_of_tools::*;
use std::collections::HashSet;
use std::str::FromStr;

type InputType = (Point<SolutionType>, Point<SolutionType>);
type SolutionType = i64;

const WIDTH: SolutionType = 101;
const HEIGHT: SolutionType = 103;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    let mut result = vec![];
    for line in input.lines() {
        // p=0,4 v=3,-3
        let mut val_iter = line.split(" ").map(|s| {
            let mut iter = s.split("=");
            iter.next(); // skip ...=
            iter.next().unwrap()
        });

        let mut pitr = val_iter
            .next()
            .unwrap()
            .split(",")
            .map(|s| SolutionType::from_str(s).unwrap());
        let pos = Point {
            x: pitr.next().unwrap(),
            y: pitr.next().unwrap(),
        };

        let mut vitr = val_iter
            .next()
            .unwrap()
            .split(",")
            .map(|s| SolutionType::from_str(s).unwrap());
        let vel = Point {
            x: vitr.next().unwrap(),
            y: vitr.next().unwrap(),
        };

        result.push((pos, vel));
    }
    result
}

#[aoc(day14, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let quads = data
        .iter()
        .map(|robot| Point {
            x: (robot.0.x + robot.1.x * 100).rem_euclid(WIDTH),
            y: (robot.0.y + robot.1.y * 100).rem_euclid(HEIGHT),
        })
        .map(|p| {
            if p.x == WIDTH / 2 || p.y == HEIGHT / 2 {
                None
            } else {
                Some(if p.x < WIDTH / 2 {
                    if p.y < HEIGHT / 2 {
                        1
                    } else {
                        2
                    }
                } else if p.y < HEIGHT / 2 {
                    3
                } else {
                    4
                })
            }
        })
        .fold([0; 5], |mut a, v| {
            if let Some(v) = v {
                a[v] += 1
            }
            a
        });
    quads[1] * quads[2] * quads[3] * quads[4]
}

fn count_quality(robots: &[InputType]) -> u32 {
    let pos: HashSet<_> = robots.iter().map(|robot| robot.0).collect();

    let mut count = 0;
    for robot in robots {
        if pos.contains(&Point {
            x: robot.0.x - 1,
            y: robot.0.y,
        }) || pos.contains(&Point {
            x: robot.0.x + 1,
            y: robot.0.y,
        }) || pos.contains(&Point {
            x: robot.0.x,
            y: robot.0.y - 1,
        }) || pos.contains(&Point {
            x: robot.0.x,
            y: robot.0.y + 1,
        }) {
            count += 1;
        }
    }

    count
}

#[aoc(day14, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let mut robots = data.to_vec();
    let mut max_quality = 0;
    let mut at_gen = 0;
    for gen in 1..=WIDTH * HEIGHT {
        for robot in robots.iter_mut() {
            robot.0.x = (robot.0.x + robot.1.x).rem_euclid(WIDTH);
            robot.0.y = (robot.0.y + robot.1.y).rem_euclid(HEIGHT);
        }
        let quality = count_quality(&robots);
        if quality > max_quality {
            max_quality = quality;
            at_gen = gen;
        }
    }

    println!("Generation {at_gen}, quality: {max_quality}");
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut count = 0;
            for robot in data {
                if (robot.0.x + robot.1.x * at_gen).rem_euclid(WIDTH) == x
                    && (robot.0.y + robot.1.y * at_gen).rem_euclid(HEIGHT) == y
                {
                    count += 1;
                }
            }
            if count == 0 {
                print!(".");
            } else {
                print!("{count}");
            }
        }
        println!();
    }
    println!();
    at_gen
}
