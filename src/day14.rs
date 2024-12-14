// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use advent_of_tools::*;
use ahash::{HashSet, HashSetExt};
use std::str::FromStr;

type PointType = i16;
type InputType = (Point<PointType>, Point<PointType>);
type SolutionType = i64;

const WIDTH: PointType = 101;
const HEIGHT: PointType = 103;

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
            .map(|s| PointType::from_str(s).unwrap());
        let pos = Point {
            x: pitr.next().unwrap(),
            y: pitr.next().unwrap(),
        };

        let mut vitr = val_iter
            .next()
            .unwrap()
            .split(",")
            .map(|s| PointType::from_str(s).unwrap());
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

//////////////////////

#[allow(unused)]
fn print_map(data: &[InputType], at_gen: SolutionType) {
    println!("Generation {at_gen}");
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut count = 0;
            for robot in data {
                if (robot.0.x as SolutionType + robot.1.x as SolutionType * at_gen)
                    .rem_euclid(WIDTH as SolutionType) as PointType
                    == x
                    && (robot.0.y as SolutionType + robot.1.y as SolutionType * at_gen)
                        .rem_euclid(HEIGHT as SolutionType) as PointType
                        == y
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
}

fn count_quality(pos: &HashSet<Point<PointType>>) -> u32 {
    let mut count = 0;
    for robot in pos {
        if pos.contains(&Point {
            x: robot.x - 1,
            y: robot.y,
        }) {
            count += 1;
        }
        if pos.contains(&Point {
            x: robot.x + 1,
            y: robot.y,
        }) {
            count += 1;
        }
        if pos.contains(&Point {
            x: robot.x,
            y: robot.y - 1,
        }) {
            count += 1;
        }
        if pos.contains(&Point {
            x: robot.x,
            y: robot.y + 1,
        }) {
            count += 1;
        }
        if pos.contains(&Point {
            x: robot.x - 1,
            y: robot.y - 1,
        }) {
            count += 1;
        }
        if pos.contains(&Point {
            x: robot.x - 1,
            y: robot.y + 1,
        }) {
            count += 1;
        }
        if pos.contains(&Point {
            x: robot.x + 1,
            y: robot.y - 1,
        }) {
            count += 1;
        }
        if pos.contains(&Point {
            x: robot.x + 1,
            y: robot.y + 1,
        }) {
            count += 1;
        }
    }

    count
}

fn get_quality_at(
    robots: &mut HashSet<Point<PointType>>,
    data: &[InputType],
    gen: SolutionType,
) -> u32 {
    robots.clear();
    for robot in data {
        robots.insert(Point::<PointType> {
            x: (robot.0.x as SolutionType + robot.1.x as SolutionType * gen)
                .rem_euclid(WIDTH as SolutionType) as PointType,
            y: (robot.0.y as SolutionType + robot.1.y as SolutionType * gen)
                .rem_euclid(HEIGHT as SolutionType) as PointType,
        });
    }
    count_quality(robots)
}

#[aoc(day14, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let mut max_quality = 0;
    let mut at_gen = 0;
    let mut pos = HashSet::new();
    for gen in 0..WIDTH as SolutionType {
        let quality = get_quality_at(&mut pos, data, gen);
        if quality > max_quality {
            max_quality = quality;
            at_gen = gen;
        }
    }

    max_quality = 0;
    let mut gen = at_gen;
    loop {
        gen += WIDTH as SolutionType;
        let quality = get_quality_at(&mut pos, data, gen);
        if quality > max_quality {
            max_quality = quality;
            at_gen = gen;
        }
        if gen >= WIDTH as SolutionType * HEIGHT as SolutionType {
            break;
        }
    }

    print_map(data, at_gen);
    at_gen
}
