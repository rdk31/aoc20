use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines().map(|l| l.unwrap());

    let earliest_timestamp: i32 = lines.next().unwrap().parse().unwrap();
    let departs: Vec<i32> = lines
        .next()
        .unwrap()
        .split(",")
        .filter(|x| x != &"x")
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{} {:?}", earliest_timestamp, departs);

    let mut earliest_id = 0;
    let mut waiting = earliest_timestamp;

    for depart in departs {
        let mut diff = -earliest_timestamp;

        while diff < 0 {
            diff += depart;
        }

        if diff < waiting {
            waiting = diff;
            earliest_id = depart;
        }
    }

    println!("part1: {}", earliest_id * waiting);

    Ok(())
}
