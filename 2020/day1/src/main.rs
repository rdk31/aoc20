use std::fs::File;
use std::io::{self, prelude::*, BufReader, Error, ErrorKind};

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;

    let nums = read(file)?;

    println!("2 numbers");

    for num1 in &nums {
        for num2 in &nums {
            if num1 + num2 == 2020 {
                println!("{} {} = {}", num1, num2, num1 * num2);
            }
        }
    }

    println!("3 numbers");

    for num1 in &nums {
        for num2 in &nums {
            for num3 in &nums {
                if num1 + num2 + num3 == 2020 {
                    println!("{} {} {} = {}", num1, num2, num3, num1 * num2 * num3);
                }
            }
        }
    }

    Ok(())
}
