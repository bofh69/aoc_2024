// SPDX-FileCopyrightText: 2024 Sebastian Andersson
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::path::PathBuf;

use advent_of_tools::*;
use rand::prelude::*;

#[derive(Debug)]
struct AppArgs {
    file1: PathBuf,
    file2: PathBuf,
    time1: i64,
    time2: i64,
}

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        println!(
            "\
            generator - generate input for day 14, part 2.

            USAGE:
                --file1 <path>
                --file2 <path>
                --time1 <num>
                --time2 <num>
            "
        );
        std::process::exit(0);
    }

    let args = AppArgs {
        time1: pargs.value_from_str("--time1")?,
        time2: pargs.value_from_str("--time2")?,
        file1: pargs.opt_value_from_os_str("--file1", parse_path)?.unwrap(),
        file2: pargs.opt_value_from_os_str("--file2", parse_path)?.unwrap(),
    };

    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Unknown arguments: {:?}", remaining);
    }

    Ok(args)
}

fn parse_path(s: &std::ffi::OsStr) -> Result<std::path::PathBuf, &'static str> {
    Ok(s.into())
}

fn read_all_from_file(name: &PathBuf) -> Result<String, std::io::Error> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(name)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn add_random_pos(map: &mut Map<i64>, n: usize) {
    use rand::distributions::Uniform;
    let mut rng = rand::thread_rng();
    let dist_x = Uniform::from(0..map.get_width());
    let dist_y = Uniform::from(0..map.get_height());

    for _ in 0..n {
        loop {
            let x = dist_x.sample(&mut rng);
            let y = dist_y.sample(&mut rng);
            let pos = Point { x, y };
            if map.get_at(pos) == Some(b'.') {
                map.set_at(pos, b'*');
                break;
            }
        }
    }
}

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };
    let result = read_all_from_file(&args.file1);
    if let Err(e) = result {
        eprintln!("Failed to read {:?}: {}", args.file1, e);
        std::process::exit(1);
    }
    let file1 = result.unwrap();
    let result = read_all_from_file(&args.file2);
    if let Err(e) = result {
        eprintln!("Failed to read {:?}: {}", args.file2, e);
        std::process::exit(1);
    }
    let file2 = result.unwrap();

    let mut map1: Map<i64> = advent_of_tools::Map::from_string(&file1);
    let mut map2: Map<i64> = advent_of_tools::Map::from_string(&file2);

    let mut pos1 = map1.find(b'*');
    let mut pos2 = map2.find(b'*');

    if pos1.len() < pos2.len() {
        add_random_pos(&mut map1, pos2.len() - pos1.len());
        pos1 = map1.find(b'*');
    }
    if pos2.len() < pos1.len() {
        add_random_pos(&mut map2, pos1.len() - pos2.len());
        pos2 = map2.find(b'*');
    }

    let time_d = args.time2 - args.time1;

    let mut rng = rand::thread_rng();
    pos1.shuffle(&mut rng);
    pos2.shuffle(&mut rng);

    let mut result = Vec::new();
    for i in 0..pos1.len() {
        let mut pos1 = pos1[i];
        let pos2 = pos2[i];

        if rand::random() {
            pos1.x += WIDTH;
        } else {
            pos1.x -= WIDTH;
        }
        if rand::random() {
            pos1.y += HEIGHT;
        } else {
            pos1.y -= HEIGHT;
        }
        let sign_x = (pos2.x - pos1.x).signum();
        let sign_y = (pos2.y - pos1.y).signum();

        let mut v_x = WIDTH + 1;
        for v in 0..WIDTH {
            if (pos1.x + sign_x * v * time_d).rem_euclid(WIDTH) == pos2.x {
                v_x = v * sign_x;
                break;
            }
        }
        if v_x == WIDTH + 1 {
            eprintln!("Can't be solved");
            std::process::exit(1);
        }
        let mut v_y = HEIGHT + 1;
        for v in 0..HEIGHT {
            if (pos1.y + sign_y * v * time_d).rem_euclid(HEIGHT) == pos2.y {
                v_y = v * sign_y;
                break;
            }
        }
        if v_y == HEIGHT + 1 {
            eprintln!("Can't be solved");
            std::process::exit(1);
        }
        pos1.x = (pos1.x - v_x * args.time1).rem_euclid(WIDTH);
        pos1.y = (pos1.y - v_y * args.time1).rem_euclid(HEIGHT);
        result.push((pos1, Point { x: v_x, y: v_y }));
    }

    for r in result {
        println!("p={},{} v={},{}", r.0.x, r.0.y, r.1.x, r.1.y);
    }
}
