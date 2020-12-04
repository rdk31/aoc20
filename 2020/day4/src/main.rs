extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

fn string_to_option(string: String) -> Option<String> {
    if string.is_empty() {
        None
    } else {
        Some(string)
    }
}

impl Passport {
    fn is_valid1(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid2(&self) -> bool {
        if !self.is_valid1() {
            return false;
        }

        let byr: i32 = self.byr.as_ref().unwrap().parse().unwrap();
        if byr < 1920 || byr > 2002 {
            return false;
        }

        let iyr: i32 = self.iyr.as_ref().unwrap().parse().unwrap();
        if iyr < 2010 || iyr > 2020 {
            return false;
        }

        let eyr: i32 = self.eyr.as_ref().unwrap().parse().unwrap();
        if eyr < 2020 || eyr > 2030 {
            return false;
        }

        let hgt = self.hgt.as_ref().unwrap();
        if hgt.ends_with("cm") {
            let height: i32 = hgt.strip_suffix("cm").unwrap().parse().unwrap();
            if height < 150 || height > 193 {
                return false;
            }
        } else if hgt.ends_with("in") {
            let height: i32 = hgt.strip_suffix("in").unwrap().parse().unwrap();
            if height < 59 || height > 76 {
                return false;
            }
        } else {
            return false;
        }

        let hair_color_regex = Regex::new(r"^#(\d|[a-f]){6}$").unwrap();
        let hcl = self.hcl.as_ref().unwrap();
        if !hair_color_regex.is_match(hcl) {
            return false;
        }

        let valid_eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        let ecl = self.ecl.as_ref().unwrap();
        if !valid_eye_colors.contains(&ecl.as_str()) {
            return false;
        }

        let passport_id_regex = Regex::new(r"^\d{9}$").unwrap();
        let pid = self.pid.as_ref().unwrap();
        if !passport_id_regex.is_match(pid) {
            return false;
        }

        true
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut passports: Vec<Passport> = Vec::new();

    let mut byr = String::new();
    let mut iyr = String::new();
    let mut eyr = String::new();
    let mut hgt = String::new();
    let mut hcl = String::new();
    let mut ecl = String::new();
    let mut pid = String::new();
    let mut cid = String::new();

    for line in reader.lines() {
        let unwrapped_line = line.unwrap();

        if unwrapped_line.is_empty() {
            passports.push(Passport {
                byr: string_to_option(byr.clone()),
                iyr: string_to_option(iyr.clone()),
                eyr: string_to_option(eyr.clone()),
                hgt: string_to_option(hgt.clone()),
                hcl: string_to_option(hcl.clone()),
                ecl: string_to_option(ecl.clone()),
                pid: string_to_option(pid.clone()),
                cid: string_to_option(cid.clone()),
            });

            byr.clear();
            iyr.clear();
            eyr.clear();
            hgt.clear();
            hcl.clear();
            ecl.clear();
            pid.clear();
            cid.clear();
        } else {
            for field in unwrapped_line.split(" ") {
                let split: Vec<&str> = field.split(":").collect();
                let key = split[0];
                let val = split[1];

                match key {
                    "byr" => byr = val.to_string(),
                    "iyr" => iyr = val.to_string(),
                    "eyr" => eyr = val.to_string(),
                    "hgt" => hgt = val.to_string(),
                    "hcl" => hcl = val.to_string(),
                    "ecl" => ecl = val.to_string(),
                    "pid" => pid = val.to_string(),
                    "cid" => cid = val.to_string(),
                    _ => panic!(),
                }
            }
        }
    }

    passports.push(Passport {
        byr: string_to_option(byr.clone()),
        iyr: string_to_option(iyr.clone()),
        eyr: string_to_option(eyr.clone()),
        hgt: string_to_option(hgt.clone()),
        hcl: string_to_option(hcl.clone()),
        ecl: string_to_option(ecl.clone()),
        pid: string_to_option(pid.clone()),
        cid: string_to_option(cid.clone()),
    });

    let mut part1 = 0;
    let mut part2 = 0;

    for passport in passports {
        if passport.is_valid1() {
            part1 += 1;
        }

        if passport.is_valid2() {
            part2 += 1;
        }

        // println!("{:?}", passport);
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);

    Ok(())
}
