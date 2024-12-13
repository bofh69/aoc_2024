// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

// use ahash::{HashMap, HashMapExt};
use regex::Regex;
use std::str::FromStr;

type NumType = i64;

#[derive(Debug)]
pub struct InputType {
    a_x: NumType,
    a_y: NumType,
    b_x: NumType,
    b_y: NumType,
    prize_x: NumType,
    prize_y: NumType,
}

type SolutionType = NumType;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    let mut lines = input.lines();

    // Button A: X+94, Y+34
    let button_re = Regex::new(r"Button .: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let prize_re = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();

    let mut result = Vec::new();
    while let Some(line) = lines.next() {
        let (_, [a_x, a_y]) = button_re.captures(line).unwrap().extract();
        let line = lines.next().unwrap();
        let (_, [b_x, b_y]) = button_re.captures(line).unwrap().extract();
        let line = lines.next().unwrap();
        let (_, [prize_x, prize_y]) = prize_re.captures(line).unwrap().extract();
        lines.next();

        let a_x = NumType::from_str(a_x).unwrap();
        let a_y = NumType::from_str(a_y).unwrap();
        let b_x = NumType::from_str(b_x).unwrap();
        let b_y = NumType::from_str(b_y).unwrap();
        let prize_x = NumType::from_str(prize_x).unwrap();
        let prize_y = NumType::from_str(prize_y).unwrap();

        result.push(InputType {
            a_x,
            a_y,
            b_x,
            b_y,
            prize_x,
            prize_y,
        });
    }
    result
}

#[aoc(day13, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let mut tokens = 0;
    for crane in data {
        let mut a = 0;
        loop {
            if a * crane.a_x > crane.prize_x {
                break;
            }
            if a * crane.a_y > crane.prize_y {
                break;
            }
            let b_rem = (crane.prize_x - a * crane.a_x) % crane.b_x;
            if b_rem == 0 {
                let b = (crane.prize_x - a * crane.a_x) / crane.b_x;
                if b >= 0 && crane.a_y * a + crane.b_y * b == crane.prize_y {
                    tokens += a * 3 + b;
                }
            }
            a += 1;
        }
    }
    tokens
}

#[aoc(day13, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let mut tokens = 0;
    for crane in data {
        let total_x = crane.prize_x + 10_000_000_000_000;
        let total_y = crane.prize_y + 10_000_000_000_000;

        let pyax_pxay = total_y * crane.a_x - total_x * crane.a_y;
        let byax_bxay = crane.b_y * crane.a_x - crane.b_x * crane.a_y;

        if byax_bxay == 0 {
            continue;
        }
        if pyax_pxay % byax_bxay != 0 {
            continue;
        }
        let b = pyax_pxay / byax_bxay;

        let tmp = total_x - b * crane.b_x;

        if tmp % crane.a_x != 0 {
            continue;
        }

        let a = tmp / crane.a_x;

        tokens += a * 3 + b;
    }
    tokens
}
