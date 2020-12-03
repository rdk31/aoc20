use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn get_scopes(map: &Vec<Vec<char>>, x_offset: usize, y_offset: usize) -> i64 {
    let width = map[0].len();

    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;

    while y < map.len() {
        if map[y][x % width] == '#' {
            trees += 1;
        }

        x += x_offset;
        y += y_offset;
    }

    trees
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let map: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    // println!("{:?}", map);

    let part1 = get_scopes(&map, 3, 1);
    println!("part1: {}", part1);

    let mut part2 = get_scopes(&map, 1, 1);
    part2 *= part1;
    part2 *= get_scopes(&map, 5, 1);
    part2 *= get_scopes(&map, 7, 1);
    part2 *= get_scopes(&map, 1, 2);
    println!("part2: {}", part2);

    Ok(())
}
