use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut part1 = 0;
    let mut part2 = 0;

    for line in reader.lines() {
        let b = line.unwrap();

        let first_split: Vec<&str> = b.split(":").collect();
        let password = &first_split[1][1..];
        let second_split: Vec<&str> = first_split[0].split(" ").collect();
        let character = second_split[1];
        let third_split: Vec<&str> = second_split[0].split("-").collect();

        let lower_bounds: i32 = third_split[0].parse().unwrap();
        let higher_bounds: i32 = third_split[1].parse().unwrap();

        let count = password.matches(character).count() as i32;

        if count <= higher_bounds && count >= lower_bounds {
            part1 += 1;
        }

        if (password.as_bytes()[(lower_bounds - 1) as usize] == character.as_bytes()[0]
            && password.as_bytes()[(higher_bounds - 1) as usize] != character.as_bytes()[0])
            || (password.as_bytes()[(lower_bounds - 1) as usize] != character.as_bytes()[0]
                && password.as_bytes()[(higher_bounds - 1) as usize] == character.as_bytes()[0])
        {
            part2 += 1;
        }
    }

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);

    Ok(())
}
