use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut highest_seat_id = 0;

    let mut seats_occupied = [false; 902];

    for line in reader.lines() {
        let unwrapped_line = line.unwrap();

        let row_str = &unwrapped_line[..7];
        let column_str = &unwrapped_line[7..];

        let row_id = row_str
            .char_indices()
            .filter(|(_, c)| *c == 'B')
            .fold(0, |acc, (i, _)| acc + (i32::pow(2, (6usize - i) as u32)));

        let column_id = column_str
            .char_indices()
            .filter(|(_, c)| *c == 'R')
            .fold(0, |acc, (i, _)| acc + (i32::pow(2, (2usize - i) as u32)));

        let seat_id = row_id * 8 + column_id;

        seats_occupied[seat_id as usize] = true;
        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }

        // println!(
        //     "{}:{} {}:{} # {}",
        //     row_str, row_id, column_str, column_id, seat_id
        // );
    }

    println!("highest seat id: {}", highest_seat_id);
    print!("seats left: ");
    seats_occupied
        .iter()
        .enumerate()
        .filter(|&(_, x)| *x == false)
        .for_each(|(i, _)| print!("{} ", i));

    Ok(())
}
