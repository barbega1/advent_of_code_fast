extern crate regex;
use regex::Regex;
use std::fmt::Display;

pub fn part1(s: &str) -> impl Display {
    let mut part1: u32 = 0;
    let bytes = s.as_bytes();
    for i in 0..(bytes.len() - 8) / 2 {
        let two_letters: u16 = bytes[i * 2] as u16 + 256u16 * bytes[i * 2 + 1] as  u16;
        let mut index = 0;
        let mut found_index = 0;
        let mut a:u16 = 0;
        let mut b:u16 = 0;
        if two_letters == 0x756du16 {
            if bytes[i * 2 + 2] == b'l' && bytes[i * 2 + 3] == b'(' {
                index = i * 2 + 4;
                found_index = index;
            }
        } else if two_letters == 0x6c75u16 {
            if bytes[i * 2 + 2] == b'(' {
                index = i * 2 + 3;
                found_index = index;
            }
        }
        if found_index > 0 {
            while bytes[index] != b',' {
                a *= 10;
                a += bytes[index] as u16 - b'0' as u16;
                index += 1;
            }
            index += 1;
            while bytes[index] != b')' {
                b *= 10;
                b += bytes[index] as u16 - b'0' as u16;
                index += 1;
            }
            if a < 1000 && b < 1000 && index - found_index <=7  {
                part1 += a as u32 * b as u32;
            }
        }
    }
    part1
}

pub fn part2(s: &str) -> impl Display {
    let mut part2: u32 = 0;
    let re = Regex::new(r"(do\(\))|(don't\(\))|(mul\(([0-9]{1,3}),([0-9]{1,3})\))").unwrap();
    let caps = re.captures_iter(s);
    let mut do_cmd = true;
    for cap in caps {
        match (cap.get(1), cap.get(2), cap.get(4), cap.get(5)) {
            (_, _, Some(a), Some(b)) => {
                if do_cmd {
                    part2 +=
                        a.as_str().parse::<u32>().unwrap() * b.as_str().parse::<u32>().unwrap();
                }
            }
            (Some(_), _, _, _) => {
                do_cmd = true;
            }
            (_, Some(_), _, _) => {
                do_cmd = false;
            }
            _ => (),
        }
    }
    part2
}
