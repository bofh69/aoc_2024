// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use ahash::{HashMap, HashMapExt};

#[derive(Debug, Clone)]
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

#[allow(dead_code)]
fn find_deps<'a>(reg: &'a str, deps: &mut Vec<&'a str>, ops: &'a Vec<(Operator, String)>) {
    if deps.contains(&reg) {
        return;
    }
    for (op, dest) in ops {
        if dest == reg {
            let op = match op {
                Operator::And(a, b) => (a, b),
                Operator::Or(a, b) => (a, b),
                Operator::Xor(a, b) => (a, b),
            };
            if !op.0.starts_with('x') && !op.0.starts_with('y') {
                find_deps(op.0, deps, ops);
            }
            if !deps.contains(&op.0.as_str()) {
                deps.push(op.0);
            }
            if !op.1.starts_with('x') && !op.1.starts_with('y') {
                find_deps(op.1, deps, ops);
            }
            if !deps.contains(&op.1.as_str()) {
                deps.push(op.1);
            }
        }
    }
}

#[allow(dead_code)]
fn find_ops_for_reg<'a>(reg: &'a str, result: &mut Vec<usize>, ops: &'a Vec<(Operator, String)>) {
    for (i, (op, dest)) in ops.iter().enumerate() {
        if dest == reg {
            if result.contains(&i) {
                return;
            }
            result.push(i);
            let op = match op {
                Operator::And(a, b) => (a, b),
                Operator::Or(a, b) => (a, b),
                Operator::Xor(a, b) => (a, b),
            };
            if !op.0.starts_with('x') && !op.0.starts_with('y') {
                find_ops_for_reg(op.0, result, ops);
            }
            if !op.1.starts_with('x') && !op.1.starts_with('y') {
                find_ops_for_reg(op.1, result, ops);
            }
        }
    }
}

fn add(x: u64, y: u64, ops: &[(Operator, String)]) -> u64 {
    let mut regs = HashMap::new();
    for i in 0..45 {
        regs.insert(format!("x{:02}", i), x & (1 << i) != 0);
        regs.insert(format!("y{:02}", i), y & (1 << i) != 0);
    }
    calc(&mut regs, ops);
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

fn is_ok_for(i: usize, ops: &[(Operator, String)]) -> bool {
    if add(0, 0, ops) != 0 {
        return false;
    }
    if i < 45 {
        let x = 1 << i;
        if add(x, 0, ops) != x || add(0, x, ops) != x {
            return false;
        }
        if add(x, 1, ops) != (x + 1) || add(1, x, ops) != (x + 1) {
            return false;
        }
    }
    if i > 0 {
        let x = (1 << i) - 1;
        if add(x, x, ops) != x * 2 {
            return false;
        }
        let x = 1 << i;
        let y = 1 << (i - 1);
        if add(x, y, ops) != x + y {
            return false;
        }
        if add(y, x, ops) != x + y {
            return false;
        }
    }
    true
}

fn fix_from(from: usize, ops: &mut Vec<(Operator, String)>) -> Option<Vec<String>> {
    let mut tmp = "".to_string();
    for i in from..45 {
        if !is_ok_for(i, ops) {
            if i <= from {
                return None;
            }
            for first_idx in 0..ops.len() {
                'second_idx: for second_idx in first_idx + 1..ops.len() {
                    use std::mem::swap;
                    let a = ops[first_idx].1.clone();
                    let b = ops[second_idx].1.clone();
                    swap(&mut ops[first_idx].1, &mut tmp);
                    swap(&mut ops[second_idx].1, &mut tmp);
                    swap(&mut ops[first_idx].1, &mut tmp);
                    for j in 0..=i {
                        let j = i - j;
                        if !is_ok_for(j, ops) {
                            swap(&mut ops[first_idx].1, &mut tmp);
                            swap(&mut ops[second_idx].1, &mut tmp);
                            swap(&mut ops[first_idx].1, &mut tmp);
                            continue 'second_idx;
                        }
                    }
                    if let Some(mut result) = fix_from(i + 1, ops) {
                        result.push(a);
                        result.push(b);
                        return Some(result);
                    }
                    swap(&mut ops[first_idx].1, &mut tmp);
                    swap(&mut ops[second_idx].1, &mut tmp);
                    swap(&mut ops[first_idx].1, &mut tmp);
                }
            }
            return None;
        }
    }
    if add(0, 0, ops) != 0 {
        return None;
    }
    for i in 0..44 {
        let x = 1 << i;
        if add(x, 0, ops) != x || add(0, x, ops) != x {
            return None;
        }
        if add(x, x, ops) != 2 * x {
            return None;
        }
        let x = ((1 << 45) - 1) ^ (1 << i);
        if add(x, 0, ops) != x || add(0, x, ops) != x {
            return None;
        }
        if add(x, x, ops) != 2 * x {
            return None;
        }
    }
    let x = (1 << 45) - 1;
    if add(x, 0, ops) != x || add(0, x, ops) != x {
        return None;
    }
    if add(x, 1, ops) != (x + 1) || add(1, x, ops) != (x + 1) {
        return None;
    }
    if add(x, x, ops) != 2 * x {
        return None;
    }

    Some(vec![])
}

#[aoc(day24, part2)]
pub fn solve_part2(data: &InputType) -> String {
    let mut ops = data.1.clone();
    if let Some(mut result) = fix_from(0, &mut ops) {
        result.sort();
        format!("{}", result.join(","))
    } else {
        panic!("Didn't find an answer!");
    }
}
