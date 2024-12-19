// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use ahash::{HashMap, HashMapExt};
use regex::*;

type InputType = (Vec<String>, Vec<String>);
type SolutionType = usize;

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> InputType {
    let mut lines = input.lines();
    let towels = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    lines.next();
    let patterns = lines.map(|s| s.to_string()).collect();
    (towels, patterns)
}

#[aoc(day19, part1)]
pub fn solve_part1(data: &InputType) -> SolutionType {
    let mut re_str = "^(".to_string();
    let mut first = true;
    for towel in &data.0 {
        if !first {
            re_str.push('|');
        }
        first = false;
        re_str.push_str(towel);
    }
    re_str.push_str(")+$");
    // println!("{}", re_str);
    let re = Regex::new(&re_str).unwrap();

    data.1.iter().filter(|s| re.find(s).is_some()).count()
}

///////////////////////////////////

fn count<'a>(mem: &mut HashMap<&'a str, usize>, towels: &Vec<String>, pattern: &'a str) -> usize {
    if let Some(res) = mem.get(pattern) {
        return *res;
    }
    let mut result = 0;
    for towel in towels {
        if pattern.starts_with(towel) {
            result += count(mem, towels, &pattern[towel.len()..]);
        }
    }
    mem.insert(pattern, result);
    result
}

#[aoc(day19, part2)]
pub fn solve_part2(data: &InputType) -> SolutionType {
    let mut result = 0;
    let mut mem = HashMap::new();
    mem.insert("", 1);
    for pattern in &data.1 {
        result += count(&mut mem, &data.0, pattern);
    }
    result
}
