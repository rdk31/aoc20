use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, PartialEq)]
struct Expression {
    body: Vec<Item>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Item {
    Number(u32),
    Addition,
    Multiplication,
    LeftParenthesis,
    RightParenthesis,
}
fn get_precedence_part1(item: Item) -> i32 {
    match item {
        Item::Addition => 1,
        Item::Multiplication => 1,
        _ => -1,
    }
}
fn get_precedence_part2(item: Item) -> i32 {
    match item {
        Item::Addition => 2,
        Item::Multiplication => 1,
        _ => -1,
    }
}

impl Expression {
    fn new(input: &str, part1: bool) -> Self {
        let lexed_input: Vec<Item> = input
            .chars()
            .filter(|&c| c != ' ')
            .map(|c| match c {
                '0'..='9' => Item::Number(c.to_digit(10).unwrap()),
                '+' => Item::Addition,
                '*' => Item::Multiplication,
                '(' => Item::LeftParenthesis,
                ')' => Item::RightParenthesis,
                _ => panic!("not supported char"),
            })
            .collect();

        // println!("{:?}", &lexed_input);

        let mut rpn = Vec::new();
        let mut operators = Vec::new();

        for item in lexed_input {
            match item {
                Item::Number(_) => rpn.push(item),
                Item::Addition | Item::Multiplication => {
                    while let Some(op) = operators.pop() {
                        // println!("op: {:?}", op);
                        if part1 {
                            if get_precedence_part1(item) <= get_precedence_part1(op) {
                                rpn.push(op);
                            } else {
                                operators.push(op);
                                break;
                            }
                        } else {
                            if get_precedence_part2(item) <= get_precedence_part2(op) {
                                rpn.push(op);
                            } else {
                                operators.push(op);
                                break;
                            }
                        }
                    }
                    operators.push(item);
                }
                Item::LeftParenthesis => operators.push(item),
                Item::RightParenthesis => {
                    while let Some(op) = operators.pop() {
                        if op != Item::LeftParenthesis {
                            rpn.push(op);
                        } else {
                            operators.push(op);
                            break;
                        }
                    }

                    operators.pop();
                }
            }

            // println!("{:?}", &item);
            // println!("out: {:?}", &rpn);
            // println!("ops: {:?}", &operators);
            // println!();
        }

        for &operator in operators.iter().rev() {
            rpn.push(operator);
        }

        // println!("{:?}", &rpn);

        Self { body: rpn }
    }

    fn solve(&self) -> u64 {
        let mut stack = Vec::new();

        for item in &self.body {
            // println!("{:?}", &stack);
            match item {
                Item::Number(x) => stack.push(*x as u64),
                Item::Addition => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a + b);
                }
                Item::Multiplication => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a * b);
                }
                Item::LeftParenthesis => {}
                Item::RightParenthesis => {}
            }
        }

        stack.pop().unwrap()
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let part1: u64 = lines.iter().map(|l| Expression::new(l, true).solve()).sum();
    println!("part1: {}", part1);
    let part2: u64 = lines
        .iter()
        .map(|l| Expression::new(l, false).solve())
        .sum();
    println!("part2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_expression() {
        assert_eq!(Expression::new("2 * 3 + (4 * 5)", true).solve(), 26)
    }
    #[test]
    fn solve_expression2() {
        assert_eq!(
            Expression::new("5 + (8 * 3 + 9 + 3 * 4 * 3)", true).solve(),
            437
        )
    }
    #[test]
    fn solve_expression3() {
        assert_eq!(
            Expression::new("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", true).solve(),
            12240
        )
    }
    #[test]
    fn solve_expression4() {
        assert_eq!(
            Expression::new("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", true).solve(),
            13632
        )
    }

    #[test]
    fn solve_expression5() {
        assert_eq!(Expression::new("2 * 3 + (4 * 5)", false).solve(), 46)
    }
    #[test]
    fn solve_expression6() {
        assert_eq!(
            Expression::new("5 + (8 * 3 + 9 + 3 * 4 * 3)", false).solve(),
            1445
        )
    }
    #[test]
    fn solve_expression7() {
        assert_eq!(
            Expression::new("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", false).solve(),
            669060
        )
    }
    #[test]
    fn solve_expression8() {
        assert_eq!(
            Expression::new("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", false).solve(),
            23340
        )
    }
    #[test]
    fn solve_expression9() {
        assert_eq!(Expression::new("1 + 2 * 3 + 4 * 5 + 6", false).solve(), 231)
    }
}
