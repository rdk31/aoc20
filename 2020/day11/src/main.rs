use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn step(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = map[0].len();
    // let height = map.len();

    let mut cloned = map.clone();

    for y in 0..map.len() {
        for x in 0..width {
            let diffs = [
                (-1, -1),
                (0, -1),
                (-1, 0),
                (1, 1),
                (1, 0),
                (0, 1),
                (1, -1),
                (-1, 1),
            ];

            let mut adjacent = 0;

            for &diff in diffs.iter() {
                if let Some(row) = map.get((y as i32 + diff.1) as usize) {
                    if let Some(col) = row.get((x as i32 + diff.0) as usize) {
                        if *col == '#' {
                            adjacent += 1;
                        }
                    }
                }
            }

            let c = cloned.get_mut(y).unwrap().get_mut(x).unwrap();

            if *c == 'L' && adjacent == 0 {
                *c = '#';
            } else if *c == '#' && adjacent >= 4 {
                *c = 'L';
            }
        }
    }

    cloned
}

use std::cmp;

fn step2(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = map[0].len();
    let height = map.len();

    let mut cloned = map.clone();

    for y in 0..map.len() {
        for x in 0..width {
            let diffs = [
                (-1, -1),
                (0, -1),
                (-1, 0),
                (1, 1),
                (1, 0),
                (0, 1),
                (1, -1),
                (-1, 1),
            ];

            let mut adjacent = 0;

            for &diff in diffs.iter() {
                for dist in 1..std::cmp::max(width, height) {
                    if let Some(row) = map.get((y as i32 + diff.1 * dist as i32) as usize) {
                        if let Some(col) = row.get((x as i32 + diff.0 * dist as i32) as usize) {
                            if *col == '#' {
                                adjacent += 1;
                                break;
                            } else if *col == 'L' {
                                break;
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
            let c = cloned.get_mut(y).unwrap().get_mut(x).unwrap();

            // if *c == 'L' {
            //     println!("adj: {}", adjacent);
            // }

            if *c == 'L' && adjacent == 0 {
                *c = '#';
            } else if *c == '#' && adjacent >= 5 {
                *c = 'L';
            }
        }
    }

    cloned
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let map2 = map.clone();

    let mut last_map = map.clone();
    map = step(map);

    while map != last_map {
        last_map = map.clone();
        map = step(map);
    }

    let part1: usize = map
        .iter()
        .map(|row| row.iter().filter(|&c| *c == '#').count())
        .sum();

    println!("part1: {}", part1);

    let mut last_map = map2.clone();
    map = step2(map2);

    while map != last_map {
        last_map = map.clone();
        map = step2(map);
    }

    // for row in &map {
    //     println!("{:?}", row);
    // }

    let part2: usize = map
        .iter()
        .map(|row| row.iter().filter(|&c| *c == '#').count())
        .sum();

    println!("part2: {}", part2);

    Ok(())
}
