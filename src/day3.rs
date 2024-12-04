extern crate regex;
use regex::Regex;
use std::fmt::Display;


pub fn part1(s: &str) -> impl Display {
    let mut part1 : u32 = 0;
    let mut part2 : u32 = 0;
    let re = Regex::new(r"(do\(\))|(don't\(\))|(mul\(([0-9]{1,3}),([0-9]{1,3})\))").unwrap();
    let caps = re.captures_iter(s);
    let mut do_cmd = true;
    for cap in caps {
        match (cap.get(1), cap.get(2), cap.get(4), cap.get(5)) {
            (_, _, Some(a), Some(b)) => {
                let mult = a.as_str().parse::<u32>().unwrap() * b.as_str().parse::<u32>().unwrap();
                part1 += mult;
                if do_cmd { part2 += mult; }
            },
            (Some(_), _, _, _) => {
                do_cmd = true;
            },
            (_, Some(_), _, _) => {
                do_cmd = false;
            },
            _ => ()
        }
    }
    part1
}

pub fn part2(s: &str) -> impl Display {
    let mut part1 : u32 = 0;
    let mut part2 : u32 = 0;
    let re = Regex::new(r"(do\(\))|(don't\(\))|(mul\(([0-9]{1,3}),([0-9]{1,3})\))").unwrap();
    let caps = re.captures_iter(s);
    let mut do_cmd = true;
    for cap in caps {
        match (cap.get(1), cap.get(2), cap.get(4), cap.get(5)) {
            (_, _, Some(a), Some(b)) => {
                let mult = a.as_str().parse::<u32>().unwrap() * b.as_str().parse::<u32>().unwrap();
                part1 += mult;
                if do_cmd { part2 += mult; }
            },
            (Some(_), _, _, _) => {
                do_cmd = true;
            },
            (_, Some(_), _, _) => {
                do_cmd = false;
            },
            _ => ()
        }
    }
    part2
}