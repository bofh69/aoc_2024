// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use ahash::{HashMap, HashMapExt};

#[derive(Debug)]
pub enum Operator {
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

type InputType = (HashMap<String, bool>, Vec<(Operator, String)>);
type SolutionType = u64;

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> InputType {
    let mut iter = input.lines();
    let mut result1 = HashMap::new();
    let mut result2 = Vec::new();
    for line in iter.by_ref() {
        if line.is_empty() {
            break;
        }
        let reg = line[0..3].to_string();
        let val = &line[5..6] == "1";
        result1.insert(reg, val);
    }

    for line in iter {
        let line: Vec<_> = line.split(" ").collect();
        let op = match line[1] {
            "AND" => Operator::And(line[0].to_string(), line[2].to_string()),
            "OR" => Operator::Or(line[0].to_string(), line[2].to_string()),
            "XOR" => Operator::Xor(line[0].to_string(), line[2].to_string()),
            _ => panic!("Unknown operator"),
        };
        result2.push((op, line[4].to_string()));
    }
    (result1, result2)
}

fn calc(regs: &mut HashMap<String, bool>, ops: &[(Operator, String)]) {
    loop {
        let mut found_any = false;
        for (op, result) in ops.iter() {
            if !regs.contains_key(result) {
                let res = match op {
                    Operator::And(a, b) => {
                        if let Some(&av) = regs.get(a) {
                            if !av {
                                Some(false)
                            } else if let Some(&bv) = regs.get(b) {
                                Some(av && bv)
                            } else {
                                None
                            }
                        } else if Some(&false) == regs.get(b) {
                            Some(false)
                        } else {
                            None
                        }
                    }
                    Operator::Or(a, b) => {
                        if let Some(&av) = regs.get(a) {
                            if av {
                                Some(true)
                            } else if let Some(&bv) = regs.get(b) {
                                Some(bv)
                            } else {
                                None
                            }
                        } else if Some(&true) == regs.get(b) {
                            Some(true)
                        } else {
                            None
                        }
                    }
                    Operator::Xor(a, b) => {
                        if let Some(&av) = regs.get(a) {
                            if let Some(&bv) = regs.get(b) {
                                Some(av != bv)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                };
                if let Some(val) = res {
                    regs.insert(result.to_string(), val);
                    found_any = true;
                }
            }
        }
        if !found_any {
            return;
        }
    }
}

#[aoc(day24, part1)]
pub fn solve_part1(data: &InputType) -> SolutionType {
    let mut regs = data.0.clone();
    calc(&mut regs, &data.1);
    let mut result = 0;
    for i in 0..64 {
        if let Some(&val) = regs.get(&format!("z{i:02}")) {
            if val {
                result |= 1 << i;
            }
        } else {
            break;
        }
    }
    result
}

#[aoc(day24, part2)]
pub fn solve_part2(data: &InputType) -> SolutionType {
    data.1.len() as SolutionType
}
