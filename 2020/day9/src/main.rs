use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn find_invalid(nums: &Vec<u64>, preamble: usize) -> Option<u64> {
    let mut pos = preamble;

    while pos < nums.len() {
        let possible_nums: Vec<u64> = nums
            .iter()
            .skip(pos - preamble)
            .take(preamble)
            .map(|n| *n)
            .collect();

        let num = nums.get(pos).unwrap();

        let mut checked = false;

        for num1 in &possible_nums {
            for num2 in &possible_nums {
                if num1 + num2 == *num {
                    checked = true;
                    break;
                }
            }
        }

        if !checked {
            return Some(*num);
        }

        pos += 1;
    }

    None
}

fn find_set(nums: &Vec<u64>, invalid: u64) -> Option<Vec<u64>> {
    for len in 2..nums.len() {
        for pos in 0..(nums.len() - len) {
            let set: Vec<u64> = nums.iter().skip(pos).take(len).map(|n| *n).collect();

            if set.iter().fold(0, |acc, n| acc + n) == invalid {
                return Some(set);
            }
        }
    }

    None
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let nums: Vec<u64> = reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    let part1 = find_invalid(&nums, 25).unwrap();
    println!("part1: {}", part1);

    let part2_vec = find_set(&nums, part1).unwrap();
    let part2_sum: u64 = part2_vec.iter().min().unwrap() + part2_vec.iter().max().unwrap();
    println!("part2: {}", part2_sum);

    Ok(())
}
