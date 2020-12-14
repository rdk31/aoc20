use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

fn navigate(actions: &Vec<Action>) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;

    let mut rot = 90;

    for action in actions {
        match action {
            Action::North(n) => y -= n,
            Action::South(n) => y += n,
            Action::East(n) => x += n,
            Action::West(n) => x -= n,
            Action::Left(r) => rot -= r,
            Action::Right(r) => rot += r,
            Action::Forward(n) => {
                if rot % 360 == 0 {
                    y -= n;
                } else if rot % 360 == 90 {
                    x += n;
                } else if rot % 360 == 180 {
                    y += n;
                } else if rot % 360 == 270 {
                    x -= n;
                }
            }
        }
        // println!("x: {} y: {} rot: {}", x, y, rot);
    }

    (x, y)
}

fn navigate2(actions: &Vec<Action>) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut way_x = 10;
    let mut way_y = -1;

    for action in actions {
        match action {
            Action::North(n) => way_y -= n,
            Action::South(n) => way_y += n,
            Action::East(n) => way_x += n,
            Action::West(n) => way_x -= n,
            Action::Left(r) => {
                let s = (-*r as f32).to_radians().sin();
                let c = (-*r as f32).to_radians().cos();
                let newx = (way_x as f32 * c - way_y as f32 * s).round();
                let newy = (way_x as f32 * s + way_y as f32 * c).round();
                way_x = newx as i32;
                way_y = newy as i32;
            }
            Action::Right(r) => {
                let s = (*r as f32).to_radians().sin();
                let c = (*r as f32).to_radians().cos();
                let newx = (way_x as f32 * c - way_y as f32 * s).round();
                let newy = (way_x as f32 * s + way_y as f32 * c).round();
                way_x = newx as i32;
                way_y = newy as i32;
            }
            Action::Forward(n) => {
                y += n * way_y;
                x += n * way_x;
            }
        }
        // println!(
        //     "action: {:?} x: {} y: {} way_x: {} way_y: {}",
        //     action, x, y, way_x, way_y
        // );
    }

    (x, y)
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut actions: Vec<Action> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        let c = line.chars().next().unwrap();
        let arg: i32 = line[1..].parse().unwrap();

        if c == 'N' {
            actions.push(Action::North(arg))
        } else if c == 'S' {
            actions.push(Action::South(arg))
        } else if c == 'E' {
            actions.push(Action::East(arg))
        } else if c == 'W' {
            actions.push(Action::West(arg))
        } else if c == 'L' {
            actions.push(Action::Left(arg))
        } else if c == 'R' {
            actions.push(Action::Right(arg))
        } else if c == 'F' {
            actions.push(Action::Forward(arg))
        }
    }

    let (x, y) = navigate(&actions);
    println!("part1: {}", x.abs() + y.abs());

    let (x, y) = navigate2(&actions);
    println!("part2: {}", x.abs() + y.abs());

    Ok(())
}
