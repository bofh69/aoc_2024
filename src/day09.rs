// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::aoc;

// use std::collections::HashSet;
// use std::str::FromStr;

type SolutionType = u64;

#[aoc(day9, part1)]
pub fn solve_part1(data: &str) -> SolutionType {
    let mut used = std::collections::VecDeque::new();
    let mut free = std::collections::VecDeque::new();
    let mut is_file = true;
    let mut file_nr: u16 = 0;
    let mut block_nr: u32 = 0;
    for c in data.chars() {
        let c = c as u8 - b'0';

        if c > 0 {
            if is_file {
                used.push_back((block_nr, file_nr, c));
                file_nr += 1;
            } else {
                free.push_back((block_nr, c));
            }
        }
        block_nr += c as u32;
        is_file = !is_file;
    }

    let mut checksum: SolutionType = 0;

    block_nr = used.pop_front().unwrap().2 as u32;

    while !used.is_empty() {
        if block_nr == used.front().unwrap().0 {
            {
                let file = used.front_mut().unwrap();
                checksum += (block_nr * file.1 as u32) as SolutionType;
                *file = (file.0 + 1, file.1, file.2 - 1);
            }
            if 0 == used.front().unwrap().2 {
                used.pop_front();
            }
        } else {
            if free.is_empty() {
                // No more free, just move it to the current position
                let file = used.front_mut().unwrap();
                *file = (block_nr, file.1, file.2);
                continue;
            }
            // Fill from back in first empty
            {
                let free_pos = free.front_mut().unwrap();
                {
                    let file = used.back_mut().unwrap();
                    checksum += (block_nr * file.1 as u32) as SolutionType;
                    *file = (file.0 + 1, file.1, file.2 - 1);
                    *free_pos = (free_pos.0 + 1, free_pos.1 - 1);
                }
                if 0 == used.back().unwrap().2 {
                    used.pop_back();
                }
            }
            if 0 == free.front().unwrap().1 {
                free.pop_front();
            }
        }
        block_nr += 1;
    }

    checksum
}

//////////////////////////////////////////

#[aoc(day9, part2)]
pub fn solve_part2(data: &str) -> SolutionType {
    let mut used = std::collections::VecDeque::new();
    let mut free = std::collections::VecDeque::new();
    let mut is_file = true;
    let mut file_nr: u16 = 0;
    let mut block_nr: u32 = 0;
    for c in data.chars() {
        let c = c as u8 - b'0';

        if c > 0 {
            if is_file {
                used.push_back((block_nr, file_nr, c));
                file_nr += 1;
            } else {
                free.push_back((block_nr, c));
            }
        }
        block_nr += c as u32;
        is_file = !is_file;
    }

    let mut first_idx = [0usize; 10];

    let mut checksum: SolutionType = 0;
    'next: while let Some((file_block, file_nr, len)) = used.pop_back() {
        for i in first_idx[len as usize]..free.len() {
            let free_space = free.get_mut(i).unwrap();
            if free_space.0 < file_block && free_space.1 >= len {
                for j in 0..len {
                    checksum += (file_nr as u64 * (free_space.0 as u64 + j as u64)) as SolutionType;
                }
                first_idx[len as usize] = i;
                *free_space = (free_space.0 + len as u32, free_space.1 - len);
                if (free_space.0 as usize) < first_idx[free_space.1 as usize] {
                    first_idx[free_space.1 as usize] = i;
                }
                continue 'next;
            } else if free_space.0 >= file_block {
                for j in 0..len {
                    checksum += (file_nr as u64 * (file_block as u64 + j as u64)) as SolutionType;
                }
                continue 'next;
            }
        }
        first_idx[len as usize] = usize::MAX;
        for j in 0..len {
            checksum += (file_nr as u64 * (file_block as u64 + j as u64)) as SolutionType;
        }
    }
    checksum
}
