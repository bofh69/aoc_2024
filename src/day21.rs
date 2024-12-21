// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

// use ahash::HashSet;
use std::str::FromStr;

type InputType = String;
type SolutionType = usize;

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input.lines().map(|s| s.to_string()).collect()
}

fn find_key(keypad: &[&str], key: char) -> (i8, i8) {
    for (y, row) in keypad.iter().enumerate() {
        if let Some(x) = row.find(key) {
            return (x as i8, y as i8);
        }
    }
    panic!("Key {key} not found in keypad {keypad:?}");
}

fn add_keys(s: &mut String, curr_pos: (i8, i8), new_pos: (i8, i8), blank_pos: (i8, i8)) {
    let mut curr_pos = curr_pos;

    while new_pos != curr_pos {
        if new_pos.0 < curr_pos.0 && (new_pos.0 != blank_pos.0 || curr_pos.1 != blank_pos.1) {
            // Left
            for _ in 0..(new_pos.0 - curr_pos.0).abs() {
                s.push('<');
            }
            curr_pos.0 = new_pos.0;
        }

        if new_pos.1 < curr_pos.1 && (new_pos.1 != blank_pos.1 || curr_pos.0 != blank_pos.0) {
            // up
            for _ in 0..(new_pos.1 - curr_pos.1).abs() {
                s.push('^');
            }
            curr_pos.1 = new_pos.1;
        }

        if new_pos.1 > curr_pos.1 && (new_pos.1 != blank_pos.1 || curr_pos.0 != blank_pos.0) {
            // down
            for _ in 0..(new_pos.1 - curr_pos.1).abs() {
                s.push('v');
            }
            curr_pos.1 = new_pos.1;
        }

        if new_pos.0 > curr_pos.0 && (new_pos.0 != blank_pos.0 || curr_pos.1 != blank_pos.1) {
            // right
            for _ in 0..(new_pos.0 - curr_pos.0).abs() {
                s.push('>');
            }
            curr_pos.0 = new_pos.0;
        }
    }

    s.push('A');
}

fn find_shortest_seq(keypad: &[&str], code: &str) -> String {
    let mut result = String::new();
    let mut curr_pos = find_key(keypad, 'A');
    let blank_pos = find_key(keypad, ' ');
    for key in code.chars() {
        let new_pos = find_key(keypad, key);
        add_keys(&mut result, curr_pos, new_pos, blank_pos);
        curr_pos = new_pos;
    }
    result
}

#[aoc(day21, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let mut result = 0;
    for code in data {
        let num = usize::from_str(&code[0..3]).unwrap();
        let sequences = find_shortest_seq(&["789", "456", "123", " 0A"], code);
        let sequences = find_shortest_seq(&[" ^A", "<v>"], &sequences);
        let sequences = find_shortest_seq(&[" ^A", "<v>"], &sequences);
        let shortest = sequences.len();
        result += num * shortest;
    }
    result
}

#[aoc(day21, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let mut result = 0;
    for code in data {
        let num = usize::from_str(&code[0..3]).unwrap();
        let mut seq = find_shortest_seq(&["789", "456", "123", " 0A"], code);
        for i in 0..25 {
            println!("After gen {i}: {:?}", seq.len());
            seq = find_shortest_seq(&[" ^A", "<v>"], &seq);
        }
        let shortest = seq.len();
        result += num * shortest;
    }
    result
}
