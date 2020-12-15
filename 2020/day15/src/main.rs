use std::collections::HashMap;

fn get_nth_number_spoken(starting_nums: &Vec<i32>, nth: usize) -> i32 {
    let mut memo: HashMap<i32, i32> = HashMap::new();
    let mut last_num = 0;

    for (i, v) in starting_nums
        .iter()
        .enumerate()
        .take(starting_nums.len() - 1)
    {
        memo.insert(*v, i as i32 + 1);
    }

    last_num = *starting_nums.iter().last().unwrap();

    for turn in starting_nums.len() + 1..nth + 1 {
        // println!("turn: {} last_num: {}", turn, last_num);
        if memo.contains_key(&last_num) {
            let num = turn as i32 - 1 - memo.get(&last_num).unwrap();
            memo.insert(last_num, turn as i32 - 1);
            // println!("updating: {} to: {}", last_num, turn - 1);
            last_num = num;
        } else {
            // println!("empty");
            memo.insert(last_num, turn as i32 - 1);
            last_num = 0;
        }

        // println!("memo: {:?}\n", memo);
    }

    last_num
}

fn main() {
    let input = vec![1, 20, 11, 6, 12, 0];
    println!("part1: {}", get_nth_number_spoken(&input, 2020));
    println!("part2: {}", get_nth_number_spoken(&input, 30000000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(get_nth_number_spoken(&vec![0, 3, 6], 2020), 436);
    }
    #[test]
    fn example1() {
        assert_eq!(get_nth_number_spoken(&vec![1, 3, 2], 2020), 1);
    }
    #[test]
    fn example2() {
        assert_eq!(get_nth_number_spoken(&vec![2, 1, 3], 2020), 10);
    }
    #[test]
    fn example3() {
        assert_eq!(get_nth_number_spoken(&vec![1, 2, 3], 2020), 27);
    }
    #[test]
    fn example4() {
        assert_eq!(get_nth_number_spoken(&vec![2, 3, 1], 2020), 78);
    }
    #[test]
    fn example5() {
        assert_eq!(get_nth_number_spoken(&vec![3, 2, 1], 2020), 438);
    }
    #[test]
    fn example6() {
        assert_eq!(get_nth_number_spoken(&vec![3, 1, 2], 2020), 1836);
    }

    #[test]
    fn example7() {
        assert_eq!(get_nth_number_spoken(&vec![0, 3, 6], 30000000), 175594);
    }
    #[test]
    fn example8() {
        assert_eq!(get_nth_number_spoken(&vec![1, 3, 2], 30000000), 2578);
    }
    #[test]
    fn example9() {
        assert_eq!(get_nth_number_spoken(&vec![2, 1, 3], 30000000), 3544142);
    }
    #[test]
    fn example10() {
        assert_eq!(get_nth_number_spoken(&vec![1, 2, 3], 30000000), 261214);
    }
    #[test]
    fn example11() {
        assert_eq!(get_nth_number_spoken(&vec![2, 3, 1], 30000000), 6895259);
    }
    #[test]
    fn example12() {
        assert_eq!(get_nth_number_spoken(&vec![3, 2, 1], 30000000), 18);
    }
    #[test]
    fn example13() {
        assert_eq!(get_nth_number_spoken(&vec![3, 1, 2], 30000000), 362);
    }
}
