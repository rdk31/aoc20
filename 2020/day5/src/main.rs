use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut highest_seat_id = 0;

    let mut seats = Vec::with_capacity(901);
    for i in 0..901 {
        seats.push(i);
    }

    for line in reader.lines() {
        let unwrapped_line = line.unwrap();

        let row_str = &unwrapped_line[..7];
        let column_str = &unwrapped_line[7..];

        let row_id = row_str.char_indices().fold(0, |acc, (i, c)| {
            if c == 'B' {
                acc + i32::pow(2, (6usize - i) as u32)
            } else {
                acc
            }
        });

        let column_id = column_str.char_indices().fold(0, |acc, (i, c)| {
            if c == 'R' {
                acc + i32::pow(2, (2usize - i) as u32)
            } else {
                acc
            }
        });

        let seat_id = row_id * 8 + column_id;

        let index = seats.iter().position(|&x| x == seat_id);
        if index.is_some() {
            seats.remove(index.unwrap());
        }
        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }

        println!(
            "{}:{} {}:{} # {}",
            row_str, row_id, column_str, column_id, seat_id
        );
    }

    println!("highest seat id: {}", highest_seat_id);
    println!("left seats: {:?}", seats);

    Ok(())
}
