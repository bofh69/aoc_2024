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

fn add_keys(result: &mut Vec<String>, curr_pos: (i8, i8), new_pos: (i8, i8), blank_pos: (i8, i8)) {
    let diff_x = new_pos.0 - curr_pos.0;
    let diff_y = new_pos.1 - curr_pos.1;
    if diff_x == 0 {
        // Only move y
        for s in result.iter_mut() {
            for _ in 0..diff_y.abs() {
                if diff_y < 0 {
                    s.push('^');
                } else {
                    s.push('v');
                }
            }
        }
    } else if diff_y == 0 {
        // Only move y
        for s in result.iter_mut() {
            for _ in 0..diff_x.abs() {
                if diff_x < 0 {
                    s.push('<');
                } else {
                    s.push('>');
                }
            }
        }
    } else if diff_x != 0 && diff_y != 0 {
        let mut new_results = result.clone();
        if curr_pos.0 == blank_pos.0 && new_pos.1 == blank_pos.1 {
            // Don't
            new_results.clear();
        } else {
            for s in new_results.iter_mut() {
                for _ in 0..diff_y.abs() {
                    if diff_y < 0 {
                        s.push('^');
                    } else {
                        s.push('v');
                    }
                }
            }
            add_keys(
                &mut new_results,
                (curr_pos.0, new_pos.1),
                new_pos,
                blank_pos,
            );
        }
        if curr_pos.1 == blank_pos.1 && new_pos.0 == blank_pos.0 {
            // Don't
            result.clear();
        } else {
            for s in result.iter_mut() {
                for _ in 0..diff_x.abs() {
                    if diff_x < 0 {
                        s.push('<');
                    } else {
                        s.push('>');
                    }
                }
            }
            add_keys(result, (new_pos.0, curr_pos.1), new_pos, blank_pos);
        }
        result.append(&mut new_results);
        return;
    }

    for s in result.iter_mut() {
        s.push('A');
    }
}

fn find_shortest_seq(keypad: &[&str], code: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut curr_pos = find_key(keypad, 'A');
    let blank_pos = find_key(keypad, ' ');
    result.push("".to_owned());
    for key in code.chars() {
        let new_pos = find_key(keypad, key);
        add_keys(&mut result, curr_pos, new_pos, blank_pos);
        curr_pos = new_pos;
    }

    let shortest = result.iter().map(|s| s.len()).min().unwrap();
    result
        .iter()
        .filter(|s| s.len() <= shortest)
        .cloned()
        .collect()
}

fn count_shortest_seq(keypad: &[&str], code: &str) -> usize {
    let mut result = Vec::new();
    let mut curr_pos = find_key(keypad, 'A');
    let blank_pos = find_key(keypad, ' ');
    result.push("".to_owned());
    for key in code.chars() {
        let new_pos = find_key(keypad, key);
        add_keys(&mut result, curr_pos, new_pos, blank_pos);
        curr_pos = new_pos;
    }

    result.iter().map(|s| s.len()).min().unwrap()
}

#[aoc(day21, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let mut result = 0;
    for code in data {
        // println!("For code {code}:");
        let num = usize::from_str(&code[0..3]).unwrap();
        let mut shortest = usize::MAX;
        let sequences = find_shortest_seq(&["789", "456", "123", " 0A"], code);
        for seq in sequences {
            let sequences = find_shortest_seq(&[" ^A", "<v>"], &seq);
            for seq in sequences {
                shortest = shortest.min(count_shortest_seq(&[" ^A", "<v>"], &seq));
            }
        }
        // println!("{code}: {shortest} * {num} = {}", num * shortest);
        result += num * shortest;
    }
    result
}

#[aoc(day21, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    data.len()
}
