// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

// use ahash::{HashMap, HashMapExt};
use std::str::FromStr;

type NumType = i64;

#[derive(Debug)]
pub struct InputType {
    a: NumType,
    _b: NumType,
    _c: NumType,
    prog: Vec<NumType>,
}

type SolutionType = NumType;

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> InputType {
    let mut lines = input.lines();

    // Register A: 729
    let mut a_iter = lines.next().unwrap().split(": ");
    a_iter.next();
    let a = NumType::from_str(a_iter.next().unwrap()).unwrap();

    let mut b_iter = lines.next().unwrap().split(": ");
    b_iter.next();
    let b = NumType::from_str(b_iter.next().unwrap()).unwrap();

    let mut c_iter = lines.next().unwrap().split(": ");
    c_iter.next();
    let c = NumType::from_str(c_iter.next().unwrap()).unwrap();

    lines.next();

    let mut d_iter = lines.next().unwrap().split(": ");
    d_iter.next();

    let prog = d_iter
        .next()
        .unwrap()
        .split(',')
        .map(|s| NumType::from_str(s).unwrap())
        .collect();

    InputType {
        a,
        _b: b,
        _c: c,
        prog,
    }
}

#[aoc(day17, part1)]
pub fn solve_part1(data: &InputType) -> SolutionType {
    let a = data.a;
    let result = run_prog(a, 0, 0, &data.prog);
    println!(
        "{}",
        result.iter().fold(String::new(), |mut acc, v| {
            if !acc.is_empty() {
                acc.push(',');
            }
            acc.push_str(&format!("{v}"));
            acc
        })
    );
    0
}

fn run_prog(mut a: NumType, mut b: NumType, mut c: NumType, prog: &[NumType]) -> Vec<NumType> {
    let mut isp = 0;
    let mut result = Vec::new();
    while isp < prog.len() {
        let inst = prog[isp];
        let o = prog[isp + 1];
        isp += 2;
        let combo = |o| match o {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => a,
            5 => b,
            6 => c,
            _ => panic!("Invalid combo operand"),
        };
        match inst {
            0 => {
                // ADV
                a /= 1 << combo(o);
            }
            6 => {
                // bdv
                b = a / (1 << combo(o));
            }
            7 => {
                // cdv
                c = a / (1 << combo(o));
            }
            1 => {
                // BXL
                b ^= o;
            }
            2 => {
                // BST
                b = combo(o) % 8;
            }
            3 => {
                // jnz
                if a != 0 {
                    isp = o as usize;
                }
            }
            4 => {
                // BXC
                b ^= c;
            }
            5 => {
                // OUT
                let out = combo(o) % 8;
                result.push(out);
            }
            _ => panic!("Invalid operand"),
        }
    }
    result
}

#[aoc(day17, part2)]
pub fn solve_part2(data: &InputType) -> SolutionType {
    for a in 1000000000..NumType::MAX {
        let result = run_prog(a, 0, 0, &data.prog);
        if result == data.prog {
            return a;
        }
        /*
        if result.len() < 3 {
            continue;
        }
        println!("A={a}: {} {} {} .. {} {} {}",
            result[0],
            result[1],
            result[1],
            data.prog[0],
            data.prog[1],
            data.prog[2],
            );
        */
    }
    0
}
