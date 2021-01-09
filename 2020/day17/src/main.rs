use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Clone)]
struct Dimension {
    cells: Vec<bool>,
    cycle: u32,
    width: usize,
    height: usize,
    depth: usize,
    fourth: usize,
}

type Pos = (usize, usize, usize, usize);

impl Dimension {
    fn new(input: &Vec<String>, width: usize, height: usize, depth: usize, fourth: usize) -> Self {
        let mut cells: Vec<bool> = (0..width * height * depth * fourth)
            .map(|_| false)
            .collect();

        let middle_x = width / 2 - input[0].len() / 2;
        let middle_y = height / 2 - input.len() / 2;
        let middle_z = depth / 2;
        let middle_fourth = fourth / 2;

        for (y, line) in input.iter().enumerate() {
            for (x, c) in line.char_indices() {
                if c == '#' {
                    let i = middle_fourth * depth * width * height
                        + middle_z * width * height
                        + (y + middle_y) * width
                        + x
                        + middle_x;
                    cells[i as usize] = true;
                }
            }
        }

        Dimension {
            cells,
            cycle: 0,
            width,
            height,
            depth,
            fourth,
        }
    }

    fn get_cell(&self, pos: &Pos) -> Option<&bool> {
        let i = pos.3 * self.width * self.height * self.depth
            + pos.2 * self.width * self.height
            + pos.1 * self.width
            + pos.0;
        self.cells.get(i)
    }

    fn get_mut_cell(&mut self, pos: &Pos) -> Option<&mut bool> {
        let i = pos.3 * self.width * self.height * self.depth
            + pos.2 * self.width * self.height
            + pos.1 * self.width
            + pos.0;
        self.cells.get_mut(i)
    }

    fn get_num_of_neighbors(&self, pos: &Pos) -> i32 {
        let mut count = 0;

        let options = [
            // (0, 0, 0, 0),
            (0, 0, 0, 1),
            (0, 0, 0, -1),
            (0, 0, 1, 0),
            (0, 0, 1, 1),
            (0, 0, 1, -1),
            (0, 0, -1, 0),
            (0, 0, -1, 1),
            (0, 0, -1, -1),
            (0, 1, 0, 0),
            (0, 1, 0, 1),
            (0, 1, 0, -1),
            (0, 1, 1, 0),
            (0, 1, 1, 1),
            (0, 1, 1, -1),
            (0, 1, -1, 0),
            (0, 1, -1, 1),
            (0, 1, -1, -1),
            (0, -1, 0, 0),
            (0, -1, 0, 1),
            (0, -1, 0, -1),
            (0, -1, 1, 0),
            (0, -1, 1, 1),
            (0, -1, 1, -1),
            (0, -1, -1, 0),
            (0, -1, -1, 1),
            (0, -1, -1, -1),
            (1, 0, 0, 0),
            (1, 0, 0, 1),
            (1, 0, 0, -1),
            (1, 0, 1, 0),
            (1, 0, 1, 1),
            (1, 0, 1, -1),
            (1, 0, -1, 0),
            (1, 0, -1, 1),
            (1, 0, -1, -1),
            (1, 1, 0, 0),
            (1, 1, 0, 1),
            (1, 1, 0, -1),
            (1, 1, 1, 0),
            (1, 1, 1, 1),
            (1, 1, 1, -1),
            (1, 1, -1, 0),
            (1, 1, -1, 1),
            (1, 1, -1, -1),
            (1, -1, 0, 0),
            (1, -1, 0, 1),
            (1, -1, 0, -1),
            (1, -1, 1, 0),
            (1, -1, 1, 1),
            (1, -1, 1, -1),
            (1, -1, -1, 0),
            (1, -1, -1, 1),
            (1, -1, -1, -1),
            (-1, 0, 0, 0),
            (-1, 0, 0, 1),
            (-1, 0, 0, -1),
            (-1, 0, 1, 0),
            (-1, 0, 1, 1),
            (-1, 0, 1, -1),
            (-1, 0, -1, 0),
            (-1, 0, -1, 1),
            (-1, 0, -1, -1),
            (-1, 1, 0, 0),
            (-1, 1, 0, 1),
            (-1, 1, 0, -1),
            (-1, 1, 1, 0),
            (-1, 1, 1, 1),
            (-1, 1, 1, -1),
            (-1, 1, -1, 0),
            (-1, 1, -1, 1),
            (-1, 1, -1, -1),
            (-1, -1, 0, 0),
            (-1, -1, 0, 1),
            (-1, -1, 0, -1),
            (-1, -1, 1, 0),
            (-1, -1, 1, 1),
            (-1, -1, 1, -1),
            (-1, -1, -1, 0),
            (-1, -1, -1, 1),
            (-1, -1, -1, -1),
        ];

        for (x, y, z, w) in options.iter() {
            let neighbor = (
                (pos.0 as i32 + x) as usize,
                (pos.1 as i32 + y) as usize,
                (pos.2 as i32 + z) as usize,
                (pos.3 as i32 + w) as usize,
            );
            if let Some(enabled) = self.get_cell(&neighbor) {
                if *enabled {
                    count += 1;
                }
            }
        }

        count
    }

    fn cycle(&mut self) {
        let cloned = self.clone();

        for w in 1..self.fourth - 1 {
            for z in 1..self.depth - 1 {
                for y in 1..self.height - 1 {
                    for x in 1..self.width - 1 {
                        let pos = (x, y, z, w);
                        let neighbors = cloned.get_num_of_neighbors(&pos);
                        let cell = self.get_mut_cell(&pos).unwrap();

                        if *cell {
                            if neighbors != 2 && neighbors != 3 {
                                *cell = false;
                            }
                        } else {
                            if neighbors == 3 {
                                *cell = true
                            }
                        }
                    }
                }
            }
        }

        self.cycle += 1;
    }

    fn get_num_of_active(&self) -> i32 {
        let mut count = 0;

        for w in 0..self.fourth {
            for z in 0..self.depth {
                for y in 0..self.height {
                    for x in 0..self.width {
                        if let Some(enabled) = self.get_cell(&(x, y, z, w)) {
                            if *enabled {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }

        count
    }

    fn print(&self) {
        println!("cycle: {}\n", self.cycle);

        for w in 0..self.fourth {
            for z in 0..self.depth {
                println!("z={} w={}", z, w);

                let mut empty = true;
                for y in 0..self.height {
                    for x in 0..self.width {
                        if let Some(enabled) = self.get_cell(&(x, y, z, w)) {
                            if *enabled {
                                empty = false;
                                break;
                            }
                        }
                    }
                }

                if !empty {
                    for y in 0..self.height {
                        for x in 0..self.width {
                            if let Some(enabled) = self.get_cell(&(x, y, z, w)) {
                                if *enabled {
                                    print!("#");
                                } else {
                                    print!(".");
                                }
                            }
                        }
                        println!();
                    }
                    println!();
                }
            }
        }
        println!();
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut dimension = Dimension::new(&lines, 100, 100, 20, 20);
    // println!("{:?}", dimension);

    // dimension.print();

    for _ in 0..6 {
        dimension.cycle();
        //     dimension.print();
    }

    println!(
        "cycles: {} active: {}",
        dimension.cycle,
        dimension.get_num_of_active()
    );

    Ok(())
}
