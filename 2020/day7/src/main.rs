extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, space1},
    combinator::map,
    multi::separated_list1,
    number::complete::be_i32,
    sequence::tuple,
    IResult,
};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Hash)]
struct Bag {
    name: String,
    count: usize,
}

impl Bag {
    fn contains_recur(&self, bag: &str, collection: &HashMap<String, Vec<Bag>>) -> bool {
        if self.name == bag {
            return true;
        }
        collection
            .get(&self.name)
            .unwrap()
            .iter()
            .any(|br| br.contains_recur(bag, collection))
    }
    fn bag_count(&self, collection: &HashMap<String, Vec<Bag>>, prev_count: usize) -> usize {
        let rules = collection.get(&self.name).unwrap();
        if rules.is_empty() {
            prev_count
        } else {
            rules
                .iter()
                .map(|br| br.bag_count(collection, br.count * prev_count))
                .sum::<usize>()
                + prev_count
        }
    }
}

fn parse_bag(input: &str) -> IResult<&str, String> {
    let (input, characteristics) = alphanumeric1(input)?;
    let (input, _) = space1(input)?;
    let (input, color) = alphanumeric1(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = alt((tag("bags"), tag("bag")))(input)?;

    Ok((
        input,
        format!("{} {}", characteristics, color.to_owned()).to_owned(),
    ))
}

fn parse_elem(input: &str) -> IResult<&str, Bag> {
    let (input, _) = space1(input)?;
    let (input, count) = digit1(input)?;
    let (input, _) = space1(input)?;
    let (input, bag) = parse_bag(input)?;

    Ok((
        input,
        Bag {
            name: bag,
            count: count.parse().unwrap(),
        },
    ))
}

fn parse_line(input: &str) -> IResult<&str, (String, Vec<Bag>)> {
    let (input, bag) = parse_bag(input)?;
    let (input, _) = tag(" contain")(input)?;

    let mut test = alt((
        map(tag(" no other bags"), |_| Vec::new()),
        separated_list1(tag(","), parse_elem),
    ));
    let (input, vec) = test(input)?;

    Ok((input, (bag, vec)))
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let bags: HashMap<String, Vec<Bag>> = lines.iter().map(|l| parse_line(&l).unwrap().1).collect();

    // for bag in &bags {
    //     println!("{:?}", bag);
    // }

    let part1 = bags
        .iter()
        .filter(|(bag, rules)| {
            if bag.as_str() == "shiny gold" {
                false
            } else {
                rules
                    .iter()
                    .any(|br| br.contains_recur("shiny gold", &bags))
            }
        })
        .count();

    println!("part1: {}", part1);

    let rules = bags.get("shiny gold").unwrap();
    let part2 = rules
        .iter()
        .map(|br| br.bag_count(&bags, br.count))
        .sum::<usize>();

    println!("part2: {}", part2);

    Ok(())
}
