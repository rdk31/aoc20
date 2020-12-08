use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Clone, PartialEq)]
enum OpType {
    Nop,
    Jmp,
    Acc,
}

#[derive(Debug, Clone)]
struct Op {
    op_type: OpType,
    arg: i32,
    run: bool,
}

fn part1(program: &mut Vec<Op>) {
    let mut ip = 0;
    let mut op = program.get_mut(ip as usize).unwrap();
    let mut acc = 0;

    while !op.run {
        match op.op_type {
            OpType::Nop => ip += 1,
            OpType::Acc => {
                acc += op.arg;
                ip += 1
            }
            OpType::Jmp => ip += op.arg,
        }

        // println!("op: {:?} ip: {} acc: {}", op, ip, acc);

        op.run = true;
        op = program.get_mut(ip as usize).unwrap();
    }

    println!("part1: {}", acc);
}

fn part2(program: &mut Vec<Op>) -> Option<i32> {
    let mut ip = 0;
    let mut op = program.get_mut(ip as usize).unwrap();
    let mut acc = 0;

    loop {
        match op.op_type {
            OpType::Nop => ip += 1,
            OpType::Acc => {
                acc += op.arg;
                ip += 1
            }
            OpType::Jmp => ip += op.arg,
        }

        // println!("op: {:?} ip: {} acc: {}", op, ip, acc);

        op.run = true;
        op = match program.get_mut(ip as usize) {
            Some(o) => o,
            None => break,
        };

        if op.run {
            return None;
        }
    }

    Some(acc)
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut program: Vec<Op> = Vec::new();

    for line in reader.lines() {
        let unwrapped_line = line.unwrap();
        let split: Vec<&str> = unwrapped_line.split(" ").collect();

        let arg: i32 = split[1].parse().unwrap();

        let op = match split[0] {
            "nop" => Op {
                op_type: OpType::Nop,
                arg: 0,
                run: false,
            },
            "jmp" => Op {
                op_type: OpType::Jmp,
                arg: arg,
                run: false,
            },
            "acc" => Op {
                op_type: OpType::Acc,
                arg: arg,
                run: false,
            },
            _ => panic!(),
        };

        program.push(op);
    }

    // println!("{:?}", program);
    part1(&mut program);

    for op in program.iter_mut() {
        op.run = false;
    }

    let mut part2_res = part2(&mut program.clone());

    let mut pos = 0;
    let mut jmp = true;

    while part2_res.is_none() {
        let mut cloned = program.clone();

        if jmp {
            pos += cloned
                .iter()
                .skip(pos)
                .position(|op| op.op_type == OpType::Jmp)
                .unwrap();
        } else {
            pos += cloned
                .iter()
                .skip(pos)
                .position(|op| op.op_type == OpType::Nop)
                .unwrap();
        }

        if jmp {
            cloned[pos].op_type = OpType::Nop;
        } else {
            cloned[pos].op_type = OpType::Jmp;
        }

        pos += 1;

        if pos >= program.len() {
            pos = 0;
            jmp = false;
        }
        part2_res = part2(&mut cloned);
    }

    println!("part2: {}", part2_res.unwrap());

    Ok(())
}
