use std::fmt::Display;

pub fn part1(s: &str) -> impl Display {
    let mut part1: u32 = 0;
    let bytes = s.as_bytes();

    let mut numbers = [0; 12];
    let mut target: u64 = 0;
    let mut number_index = 0;
    let mut temp_num: u64 = 0;

    for c in bytes {
        match c {
            b':' => {
                target = temp_num;
                temp_num = 0;
            }
            b' ' => {
                if temp_num != 0 {
                    numbers[number_index] = temp_num as u16;
                    number_index += 1;
                    temp_num = 0;
                }
            }
            b'\n' => {
                numbers[number_index] = temp_num as u16;
                number_index += 1;
                temp_num = 0;
                let mut operators = [0; 12];
                for i in 0..number_index {
                    print!("{} ", numbers[i]);
                    if (target / numbers[i] as u64) * numbers[i] as u64 == target {
                        target = target / numbers[i] as u64;
                    } else {
                        operators[i] = 1;
                        target = target - numbers[i] as u64;
                    }
                }
                if target == 0 || target == 1 {
                    part1 += 1;
                }
                println!("{}", target);
                number_index = 0;
            }
            _ => {
                temp_num = temp_num * 10 + (c - b'0') as u64;
            }
        }
    }
    part1
}

pub fn part2(s: &str) -> impl Display {
    let mut part2: u32 = 0;
    part2
}
