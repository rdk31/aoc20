use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut answers = [0; 26];

    let mut part1 = 0;
    let mut part2 = 0;

    let mut group_lines = 0;

    for line in reader.lines() {
        let unwrapped_line = line.unwrap();

        if unwrapped_line.is_empty() {
            part1 += answers.iter().filter(|&b| *b > 0).count();
            part2 += answers.iter().filter(|&b| *b == group_lines).count();

            answers.iter_mut().for_each(|b| *b = 0);
            group_lines = 0;
        } else {
            unwrapped_line.chars().for_each(|c| {
                answers[c as usize - 'a' as usize] += 1;
            });
            group_lines += 1;
        }
    }
    part2 += answers.iter().filter(|&b| *b == group_lines).count();
    part1 += answers.iter().filter(|&b| *b > 0).count();
    answers.iter_mut().for_each(|b| *b = 0);

    println!("part1: {}", part1);
    println!("part2: {}", part2);

    Ok(())
}
