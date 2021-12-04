use std::cmp;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part1(nums: Vec<i32>) -> [i32; 3] {
    let mut diffs = [0; 3];

    let mut joltage = 0;

    for n in nums {
        diffs[(n - joltage - 1) as usize] += 1;
        joltage = n;
    }

    diffs[2] += 1;

    return diffs;
}

fn part2(nums: Vec<i32>) -> i32 {
    let max = nums.iter().max().map(|n| *n).unwrap();

    step(&nums, 0, max)
}

fn step<'a>(
    // arrangement: Vec<i32>,
    aval_nums: &'a [i32],
    last_added: i32,
    stop_joltage: i32,
) -> i32 {
    if last_added == stop_joltage || aval_nums.is_empty() {
        // println!("end: {:?}", arrangement);
        return 1;
    }

    let mut res = 0;

    // println!("current: {:?} possible: {:?}", arrangement, aval_nums);

    for n in 0..cmp::min(aval_nums.len(), 3) {
        if aval_nums[n] - last_added <= 3 {
            // println!("n: {} next aval: {:?}", n, &aval_nums[1 + n..]);
            // let mut arr_cloned = arrangement.clone();
            // arr_cloned.push(aval_nums[n]);
            res += step(&aval_nums[1 + n..], aval_nums[n], stop_joltage)
        }
    }

    res
}

use std::collections::BTreeMap;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // let mut nums: Vec<i32> = reader
    //     .lines()
    //     .map(|l| l.unwrap().parse().unwrap())
    //     .collect();

    // nums.sort();
    // println!("{:?}", &nums);

    // let part1_result = part1(nums.clone());
    // println!("part1: {:?}", part1_result[0] * part1_result[2]);

    let mut adapters: Vec<usize> = reader
        .lines()
        .filter_map(|s| s.ok()?.parse().ok())
        .collect();
    adapters.push(0);
    adapters.sort();

    let mut edges = BTreeMap::new();
    adapters.iter().for_each(|&a| {
        edges.entry(a).or_insert_with(|| Vec::new()).extend(
            (1..4)
                .map(|n| a + n)
                .filter(|b| adapters.binary_search(&b).is_ok()),
        );
    });

    // The input is a Directed Acyclic Graph
    // count number of path from 0 to max
    let mut paths_count = BTreeMap::new();
    adapters.iter().rev().for_each(|&v| {
        let neighbours = &edges[&v];
        if neighbours.is_empty() {
            paths_count.insert(v, 1u64);
        } else {
            paths_count.insert(v, neighbours.iter().map(|n| paths_count[n]).sum());
        }
    });

    println!("{:?}", paths_count[&0]);

    Ok(())
}
