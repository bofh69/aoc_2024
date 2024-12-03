// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::aoc;

use regex::Regex;
use std::str::FromStr;

type SolutionType = i64;

#[aoc(day3, part1)]
pub fn solve_part1(data: &str) -> SolutionType {
    // Match in xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").expect("Compilable regex");

    re.captures_iter(data)
        .map(|m| {
            let (_, [f1, f2]) = m.extract();
            let f1 = SolutionType::from_str(f1).unwrap();
            let f2 = SolutionType::from_str(f2).unwrap();
            f1 * f2
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(data: &str) -> SolutionType {
    let re =
        Regex::new(r"(don't\(\))|(do\(\))|(mul\(([0-9]+),([0-9]+)\))").expect("Compilable regex");

    let mut sum = 0;
    let mut do_it = true;
    for c in re.captures_iter(data) {
        if c.get(1).is_some() {
            do_it = false;
        } else if c.get(2).is_some() {
            do_it = true;
        } else if do_it {
            let f1 = SolutionType::from_str(c.get(4).unwrap().as_str()).unwrap();
            let f2 = SolutionType::from_str(c.get(5).unwrap().as_str()).unwrap();
            sum += f1 * f2;
        }
    }

    sum
}
