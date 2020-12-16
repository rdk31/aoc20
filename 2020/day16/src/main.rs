use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Rule {
    name: String,
    range_1_down: i32,
    range_1_up: i32,
    range_2_down: i32,
    range_2_up: i32,
}

type Ticket = Vec<i32>;

fn parse_rule(line: &str) -> Rule {
    let split: Vec<&str> = line.split(": ").collect();

    let name = split[0];

    let split: Vec<&str> = split[1].split(" or ").collect();

    let range: Vec<i32> = split[0]
        .split("-")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let range_1_down: i32 = range[0];
    let range_1_up: i32 = range[1];

    let range: Vec<i32> = split[1]
        .split("-")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let range_2_down: i32 = range[0];
    let range_2_up: i32 = range[1];

    Rule {
        name: name.to_string(),
        range_1_down,
        range_1_up,
        range_2_down,
        range_2_up,
    }
}

fn parse_ticket(line: &str) -> Ticket {
    line.split(",").map(|s| s.parse::<i32>().unwrap()).collect()
}

fn find_invalid_field(ticket: &Ticket, rules: &Vec<Rule>) -> Option<i32> {
    for &field in ticket {
        if !rules.iter().any(|rule| {
            (field >= rule.range_1_down && field <= rule.range_1_up)
                || (field >= rule.range_2_down && field <= rule.range_2_up)
        }) {
            return Some(field);
        }
    }

    None
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let split: Vec<&[String]> = lines.as_slice().split(|l| l == "").collect();

    let mut rules: Vec<Rule> = Vec::with_capacity(split[0].len());
    for rule_line in split[0] {
        rules.push(parse_rule(rule_line));
    }

    let my_ticket = parse_ticket(&split[1][1]);

    let mut nearby_tickets: Vec<Ticket> = Vec::with_capacity(split[2].len() - 1);
    for ticket_line in &split[2][1..] {
        nearby_tickets.push(parse_ticket(ticket_line));
    }

    // println!("rules: {:?}", rules);
    // println!("my ticket: {:?}", my_ticket);
    // println!("nearby tickets: {:?}", nearby_tickets);

    let mut part1 = 0;
    for ticket in &nearby_tickets {
        match find_invalid_field(ticket, &rules) {
            Some(field) => part1 += field,
            None => (),
        }
    }

    println!("part1: {}", part1);

    let mut valid_nearby_tickets: Vec<Ticket> = Vec::new();

    for ticket in &nearby_tickets {
        match find_invalid_field(ticket, &rules) {
            Some(_) => (),
            None => valid_nearby_tickets.push(ticket.clone()),
        }
    }

    // println!("valid tickets: {:?}", valid_nearby_tickets);

    let mut rules_matched_fields: Vec<(usize, Vec<usize>)> = Vec::new();

    for (id, rule) in rules.iter().enumerate() {
        let mut tmp = Vec::new();
        for i in 0..my_ticket.len() {
            let column: Vec<i32> = valid_nearby_tickets.iter().map(|x| x[i]).collect();

            if column.iter().all(|&x| {
                (x >= rule.range_1_down && x <= rule.range_1_up)
                    || (x >= rule.range_2_down && x <= rule.range_2_up)
            }) {
                tmp.push(i);
            }
        }
        rules_matched_fields.push((id, tmp));
    }

    rules_matched_fields.sort_by(|v1, v2| v1.1.len().cmp(&v2.1.len()));

    let mut rules_fields = vec![0; rules.len()];

    let mut matched_fields = vec![false; rules_matched_fields.len()];
    for (rule_id, fields) in &rules_matched_fields {
        for &f in fields {
            if !matched_fields[f] {
                rules_fields[*rule_id] = f;
                matched_fields[f] = true;
                break;
            }
        }
    }

    // println!("{:?}", rules_fields);

    let departure_rules_id: Vec<usize> = rules
        .iter()
        .enumerate()
        .filter(|(_, rule)| rule.name.starts_with("departure"))
        .map(|(i, _)| i)
        .collect();

    // println!("{:?}", departure_rules_id);

    let part2: u64 = departure_rules_id
        .iter()
        .map(|&i| rules_fields[i])
        .map(|i| my_ticket[i] as u64)
        .product();
    println!("part2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rule_test() {
        assert_eq!(
            parse_rule(&"class: 1-3 or 5-7"),
            Rule {
                name: "class".to_string(),
                range_1_down: 1,
                range_1_up: 3,
                range_2_down: 5,
                range_2_up: 7
            }
        );
    }

    #[test]
    fn parse_rule_test2() {
        assert_eq!(
            parse_rule(&"row: 6-11 or 33-44"),
            Rule {
                name: "row".to_string(),
                range_1_down: 6,
                range_1_up: 11,
                range_2_down: 33,
                range_2_up: 44
            }
        );
    }

    #[test]
    fn parse_rule_test3() {
        assert_eq!(
            parse_rule(&"seat: 13-40 or 45-50"),
            Rule {
                name: "seat".to_string(),
                range_1_down: 13,
                range_1_up: 40,
                range_2_down: 45,
                range_2_up: 50
            }
        );
    }

    #[test]
    fn parse_rule_test4() {
        assert_eq!(
            parse_rule(&"departure location: 25-80 or 90-961"),
            Rule {
                name: "departure location".to_string(),
                range_1_down: 25,
                range_1_up: 80,
                range_2_down: 90,
                range_2_up: 961
            }
        );
    }

    #[test]
    fn parse_ticket_test() {
        assert_eq!(parse_ticket("7,1,14"), vec![7, 1, 14]);
    }
    #[test]
    fn parse_ticket_test2() {
        assert_eq!(parse_ticket("7,3,47"), vec![7, 3, 47]);
    }

    #[test]
    fn parse_ticket_test3() {
        assert_eq!(parse_ticket("40,4,50"), vec![40, 4, 50]);
    }

    #[test]
    fn parse_ticket_test4() {
        assert_eq!(parse_ticket("55,2,20"), vec![55, 2, 20]);
    }

    #[test]
    fn parse_ticket_test5() {
        assert_eq!(parse_ticket("38,6,12"), vec![38, 6, 12]);
    }

    #[test]
    fn find_invalid_field_test() {
        let rules = vec![
            Rule {
                name: "class".to_string(),
                range_1_down: 1,
                range_1_up: 3,
                range_2_down: 5,
                range_2_up: 7,
            },
            Rule {
                name: "row".to_string(),
                range_1_down: 6,
                range_1_up: 11,
                range_2_down: 33,
                range_2_up: 44,
            },
            Rule {
                name: "seat".to_string(),
                range_1_down: 13,
                range_1_up: 40,
                range_2_down: 45,
                range_2_up: 50,
            },
        ];

        let ticket = vec![7, 3, 47];

        assert_eq!(find_invalid_field(&ticket, &rules), None);
    }

    #[test]
    fn find_invalid_field_test2() {
        let rules = vec![
            Rule {
                name: "class".to_string(),
                range_1_down: 1,
                range_1_up: 3,
                range_2_down: 5,
                range_2_up: 7,
            },
            Rule {
                name: "row".to_string(),
                range_1_down: 6,
                range_1_up: 11,
                range_2_down: 33,
                range_2_up: 44,
            },
            Rule {
                name: "seat".to_string(),
                range_1_down: 13,
                range_1_up: 40,
                range_2_down: 45,
                range_2_up: 50,
            },
        ];

        let ticket = vec![40, 4, 50];

        assert_eq!(find_invalid_field(&ticket, &rules), Some(4));
    }

    #[test]
    fn find_invalid_field_test3() {
        let rules = vec![
            Rule {
                name: "class".to_string(),
                range_1_down: 1,
                range_1_up: 3,
                range_2_down: 5,
                range_2_up: 7,
            },
            Rule {
                name: "row".to_string(),
                range_1_down: 6,
                range_1_up: 11,
                range_2_down: 33,
                range_2_up: 44,
            },
            Rule {
                name: "seat".to_string(),
                range_1_down: 13,
                range_1_up: 40,
                range_2_down: 45,
                range_2_up: 50,
            },
        ];

        let ticket = vec![55, 2, 20];

        assert_eq!(find_invalid_field(&ticket, &rules), Some(55));
    }

    #[test]
    fn find_invalid_field_test4() {
        let rules = vec![
            Rule {
                name: "class".to_string(),
                range_1_down: 1,
                range_1_up: 3,
                range_2_down: 5,
                range_2_up: 7,
            },
            Rule {
                name: "row".to_string(),
                range_1_down: 6,
                range_1_up: 11,
                range_2_down: 33,
                range_2_up: 44,
            },
            Rule {
                name: "seat".to_string(),
                range_1_down: 13,
                range_1_up: 40,
                range_2_down: 45,
                range_2_up: 50,
            },
        ];

        let ticket = vec![38, 6, 12];

        assert_eq!(find_invalid_field(&ticket, &rules), Some(12));
    }
}
