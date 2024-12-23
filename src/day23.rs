// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use itertools::*;

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
    let data : Vec<_> = data.iter().map(|s| (&s[0..2], &s[3..5])).collect();
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
                        if vert2 != vert0 && vert2 != vert1 {
                            if edges.get(vert2).unwrap().contains(vert0) {
                                if ! sets.contains(&(vert0, vert2, vert1)) &&
                                   ! sets.contains(&(vert1, vert0, vert2)) &&
                                   ! sets.contains(&(vert1, vert2, vert0)) &&
                                   ! sets.contains(&(vert2, vert0, vert1)) &&
                                   ! sets.contains(&(vert2, vert1, vert0)) {
                                    sets.insert((vert0, vert1, vert2));
                                    // println!("Maybe {vert0} {vert1} {vert2}");
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // println!("{sets:?}");
   
    sets.len() as SolutionType
}

fn is_cliq(vertices: &Vec<&&str>, edges: &HashSet<(&str, &str)>) -> bool {
    for pair in vertices.iter().combinations(2) {
        if !edges.contains(&(pair[0], pair[1])) {
            return false;
        }
    }
    return true;
}

#[aoc(day23, part2)]
pub fn solve_part2(data: &InputType) -> SolutionType {
    let data : Vec<_> = data.iter().map(|s| (&s[0..2], &s[3..5])).collect();
    let vertices : HashSet<&str> = data.iter().fold(HashSet::new(), |mut l, e| {
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

    let mut best = "".to_string();
    let mut all_in_cliqs = vertices.clone();
    for i in 3.. {
        let mut new_all_in_cliqs = HashSet::new();
        for mut possible_cliq in all_in_cliqs.iter().combinations(i) {
            if is_cliq(&possible_cliq, &dir_edges) {
                for &&vert in &possible_cliq {
                    new_all_in_cliqs.insert(vert);
                }
                possible_cliq.sort();
                best = "".to_string();
                for (i, s) in possible_cliq.iter().enumerate() {
                    if i != 0 {
                        best.push(',');
                    }
                    best.push_str(s);
                }
                println!("Best so far: {best}");
            }
        }
        if new_all_in_cliqs.len() > 0 {
            for vert in vertices.clone() {
                if !new_all_in_cliqs.contains(vert) {
                    all_in_cliqs.remove(vert);
                }
            }
        } else {
            break;
        }
    }

    println!("{best:?}");

    /*
    println!("graph {{");
    for edge in &data {
        println!("{} -- {}", edge.0, edge.1);
    }
    println!("}}");
    */

    data.len() as SolutionType
}
