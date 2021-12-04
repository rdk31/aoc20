use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut part1 = 0;
    let mut part2 = 0;

    for line in reader.lines() {
        let unwrapped_line = line.unwrap();

        let first_split: Vec<&str> = unwrapped_line.split(":").collect();
        let password = &first_split[1][1..];
        let second_split: Vec<&str> = first_split[0].split(" ").collect();
        let character = second_split[1].chars().nth(0).unwrap();
        let third_split: Vec<&str> = second_split[0].split("-").collect();

        let lower_bounds: i32 = third_split[0].parse::<i32>().unwrap();
        let higher_bounds: i32 = third_split[1].parse::<i32>().unwrap();

        let count = password.matches(character).count() as i32;

        if count <= higher_bounds && count >= lower_bounds {
            part1 += 1;
        }

        let first_char = password.chars().nth((lower_bounds - 1) as usize).unwrap();
        let second_char = password.chars().nth((higher_bounds - 1) as usize).unwrap();

        if (first_char == character && second_char != character)
            || (first_char != character && second_char == character)
        {
            part2 += 1;
        }
    }

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);

    Ok(())
}
