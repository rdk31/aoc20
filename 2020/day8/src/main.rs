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
}

struct Registers {
    acc: i32,
    ip: usize,
}

struct Emulator {
    registers: Registers,
    code: Vec<(Op, bool)>,
}

impl Emulator {
    fn new(code: Vec<(Op, bool)>) -> Self {
        Emulator {
            registers: Registers { acc: 0, ip: 0 },
            code: code,
        }
    }

    fn reset(&mut self) {
        self.registers.acc = 0;
        self.registers.ip = 0;

        self.code.iter_mut().for_each(|(_, run)| *run = false);
    }

    fn step(&mut self) -> Option<bool> {
        let (op, run) = match self.code.get_mut(self.registers.ip) {
            Some(x) => x,
            None => return None,
        };

        if *run {
            return Some(true);
        }

        match op.op_type {
            OpType::Nop => self.registers.ip += 1,
            OpType::Jmp => self.registers.ip = ((self.registers.ip as i32) + op.arg) as usize,
            OpType::Acc => {
                self.registers.acc += op.arg;
                self.registers.ip += 1;
            }
        }

        *run = true;

        Some(false)
    }

    fn part1(&mut self) -> i32 {
        self.reset();

        let mut op_run_before = self.step().unwrap();

        while !op_run_before {
            op_run_before = self.step().unwrap();
        }

        return self.registers.acc;
    }

    fn part2(&mut self) -> Option<i32> {
        self.reset();

        loop {
            match self.step() {
                Some(true) => return None,
                Some(false) => (),
                None => return Some(self.registers.acc),
            }
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut code: Vec<(Op, bool)> = Vec::new();

    for line in reader.lines() {
        let unwrapped_line = line.unwrap();
        let split: Vec<&str> = unwrapped_line.split(" ").collect();

        let arg: i32 = split[1].parse().unwrap();

        let op = match split[0] {
            "nop" => Op {
                op_type: OpType::Nop,
                arg: 0,
            },
            "jmp" => Op {
                op_type: OpType::Jmp,
                arg: arg,
            },
            "acc" => Op {
                op_type: OpType::Acc,
                arg: arg,
            },
            _ => panic!(),
        };

        code.push((op, false));
    }

    let mut emulator = Emulator::new(code.clone());

    let part1 = emulator.part1();
    println!("part1: {}", part1);

    let mut part2_res = emulator.part2();

    let mut pos = 0;
    let mut jmp = true;

    while part2_res.is_none() {
        let mut cloned = code.clone();

        if jmp {
            pos += cloned
                .iter()
                .skip(pos)
                .position(|(op, _)| op.op_type == OpType::Jmp)
                .unwrap();
        } else {
            pos += cloned
                .iter()
                .skip(pos)
                .position(|(op, _)| op.op_type == OpType::Nop)
                .unwrap();
        }

        if jmp {
            cloned[pos].0.op_type = OpType::Nop;
        } else {
            cloned[pos].0.op_type = OpType::Jmp;
        }

        pos += 1;

        if pos >= code.len() {
            pos = 0;
            jmp = false;
        }

        emulator = Emulator::new(cloned);
        part2_res = emulator.part2();
    }

    println!("part2: {}", part2_res.unwrap());

    Ok(())
}
