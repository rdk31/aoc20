use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut memory1: HashMap<u64, u64> = HashMap::new();
    let mut memory2: HashMap<u64, u64> = HashMap::new();

    let mut mask_and: u64 = 0;
    let mut mask_or: u64 = 0;

    for line in &lines {
        if line.starts_with("me") {
            let split: Vec<&str> = line.split(" = ").collect();

            let addr: u64 = split[0][4..split[0].len() - 1].parse().unwrap();
            let val: u64 = split[1].parse().unwrap();

            let val_with_mask = (val & mask_and) | mask_or;

            // println!("addr: {} val: {} val parsed: {}", addr, val_with_mask, val);
            memory1.insert(addr, val_with_mask);
        } else if line.starts_with("ma") {
            let val = line.split(" = ").nth(1).unwrap();

            let mut tmp: u64 = 1;

            mask_and = 0;
            mask_or = 0;

            for c in val.chars().rev() {
                if c == 'X' {
                    mask_and += tmp;
                } else if c == '1' {
                    mask_or += tmp;
                }

                tmp *= 2;
            }

            // println!("and: {} or: {}", mask_and, mask_or);
        }
    }

    let part1: u64 = memory1.values().sum();
    println!("part1: {}", part1);

    let mut mask = String::new();

    for line in &lines {
        if line.starts_with("me") {
            let split: Vec<&str> = line.split(" = ").collect();

            let addr: u64 = split[0][4..split[0].len() - 1].parse().unwrap();
            let val: u64 = split[1].parse().unwrap();

            let mut addrs: Vec<u64> = Vec::new();

            let mut tmp: u64 = 1;
            mask_or = 0;
            for c in mask.chars().rev() {
                if c == '1' {
                    mask_or += tmp;
                }

                tmp *= 2;
            }

            let addr = addr | mask_or;

            let floating_count = mask.chars().filter(|c| *c == 'X').count() as u32;
            for x in 0..2u64.pow(floating_count) {
                mask_and = 0;
                mask_or = 0;
                tmp = 1;

                let mut b = 0;

                for c in mask.chars().rev() {
                    if c == 'X' {
                        let l = 2u64.pow(b as u32);
                        if x & l == l {
                            mask_or += tmp;
                        // print!("1");
                        } else {
                            // print!("0");
                        }
                        b += 1;
                    // bit i of x == 1 -> mask_or += tmp else {}
                    } else {
                        mask_and += tmp;
                    }

                    tmp *= 2;
                }
                // println!();

                addrs.push((addr & mask_and) | mask_or);
            }

            // println!("addrs: {:?} val: {}", addrs, val);
            for a in addrs {
                memory2.insert(a, val);
            }
        } else if line.starts_with("ma") {
            let val = line.split(" = ").nth(1).unwrap();

            mask = val.to_string();
        }
    }

    let part2: u64 = memory2.values().sum();
    println!("part2: {}", part2);

    Ok(())
}
