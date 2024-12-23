// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

// use itertools::*;

// use advent_of_tools::*;
use ahash::{HashMap, HashMapExt};
use ahash::{HashSet, HashSetExt};

// use rayon::prelude::*;

type InputType = Vec<String>;
type SolutionType = usize;

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> InputType {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day23, part1)]
pub fn solve_part1(data: &InputType) -> SolutionType {
    // map.get_width() as SolutionType
    let data: Vec<_> = data.iter().map(|s| (&s[0..2], &s[3..5])).collect();
    let vertices = data.iter().fold(HashSet::new(), |mut l, e| {
        l.insert(e.0);
        l.insert(e.1);
        l
    });
    let mut edges = HashMap::new();
    for edge in &data {
        edges.entry(edge.0).or_insert(Vec::new()).push(edge.1);
        edges.entry(edge.1).or_insert(Vec::new()).push(edge.0);
    }

    let mut sets = HashSet::new();
    for vert0 in &vertices {
        if vert0.starts_with('t') {
            let vert0_edges = edges.get(vert0).unwrap();
            for vert1 in vert0_edges {
                if vert1 != vert0 {
                    for vert2 in edges.get(vert1).unwrap() {
                        if vert2 != vert0
                            && vert2 != vert1
                            && edges.get(vert2).unwrap().contains(vert0)
                            && !sets.contains(&(vert0, vert2, vert1))
                            && !sets.contains(&(vert1, vert0, vert2))
                            && !sets.contains(&(vert1, vert2, vert0))
                            && !sets.contains(&(vert2, vert0, vert1))
                            && !sets.contains(&(vert2, vert1, vert0))
                        {
                            sets.insert((vert0, vert1, vert2));
                        }
                    }
                }
            }
        }
    }

    sets.len() as SolutionType
}

#[aoc(day23, part2)]
pub fn solve_part2(data: &InputType) -> String {
    let data: Vec<_> = data.iter().map(|s| (&s[0..2], &s[3..5])).collect();
    let vertices = data.iter().fold(HashSet::new(), |mut l, e| {
        l.insert(e.0);
        l.insert(e.1);
        l
    });
    let mut edges = HashMap::new();
    for edge in &data {
        edges.entry(edge.0).or_insert(HashSet::new()).insert(edge.1);
        edges.entry(edge.1).or_insert(HashSet::new()).insert(edge.0);
    }
    let mut dir_edges = HashSet::new();
    for (v0, v1) in data.iter() {
        dir_edges.insert((*v0, *v1));
        dir_edges.insert((*v1, *v0));
    }

    let mut best_cliq = Vec::new();
    for &vert in &vertices {
        if best_cliq.contains(&vert) {
            continue;
        }
        let mut new_cliq = Vec::new();
        new_cliq.push(vert);
        'vert1: for vert1 in edges.get(&vert).unwrap() {
            for vert2 in &new_cliq {
                if vert2 == vert1 {
                    continue;
                }
                if !dir_edges.contains(&(vert1, vert2)) {
                    continue 'vert1;
                }
            }
            new_cliq.push(vert1);
        }
        if new_cliq.len() > best_cliq.len() {
            best_cliq = new_cliq;
        }
    }
    let mut best_cliq: Vec<_> = best_cliq.iter().collect();
    best_cliq.sort();
    let mut result = "".to_string();
    for (i, s) in best_cliq.iter().enumerate() {
        if i != 0 {
            result.push(',');
        }
        result.push_str(s);
    }

    result
}
