
use std::fmt::Display;


pub fn part1(s: &str) -> impl Display {
    unsafe{
    let mut part1: u32 = 0;
    let mut cache: [[u8; 100]; 100] = [[0; 100]; 100];

    let bytes = s.as_bytes();

    let mut temp_num: usize = 0;
    let mut a = 0;
    let mut i = 0;
    loop {
        let c = bytes.get_unchecked(i);
        match c {
            b'|' => {
                a = temp_num;
                temp_num = 0;
            }
            b'\n' => {
                if a == 0 {
                    break;
                } else {
                    *cache.get_unchecked_mut(a).get_unchecked_mut(temp_num) = 1;
                }
                temp_num = 0;
                a = 0;
            }
            _ => temp_num = temp_num.unchecked_mul(10).unchecked_add(c.unchecked_sub(b'0').into()),
        }
        i = i.unchecked_add(1);
    }
    let mut line_len: usize = 0;
    let mut prev_num : usize = 0;
    let mut line_ok = true;
    i = i.unchecked_add(1);
    loop {
        let c = bytes.get_unchecked(i);
        line_len = line_len.unchecked_add(1);
        match c {
            b',' => {
                if prev_num != 0 {
                    line_ok &= cache[prev_num as usize][temp_num as usize] == 1u8;
                    //println!("{} {} {}", prev_num, temp_num, line_ok);
                }
                prev_num = temp_num;
                temp_num = 0;
            }
            b'\n' => {
                line_ok &= cache.get_unchecked(prev_num).get_unchecked(temp_num) == &1u8;
                if line_ok {
                    //println!("Line is ok {}", line_len);
                    part1 = part1.unchecked_add ((10 * (bytes.get_unchecked(i.unchecked_sub(line_len >> 1).unchecked_sub(1)) - 0x30)
                        + bytes.get_unchecked(i.unchecked_sub(line_len >> 1))
                        - 0x30).into()) as u32;
                    //println!("{}" , part1);
                }
                line_ok = true;
                temp_num = 0;
                prev_num = 0;
                line_len = 0;
                if i + 1 == bytes.len() {
                    break;
                }
            }
            _ => temp_num = temp_num.unchecked_mul(10).unchecked_add(c.unchecked_sub(b'0').into()),
        }
        i = i.unchecked_add(1);
    }
    part1
}
}

pub fn part2(s: &str) -> impl Display {
    unsafe {
    let mut part2: u32 = 0;
    let mut cache: [[u8; 100]; 100] = [[0; 100]; 100];
    let mut line_nums: [u8; 23] = [0; 23];
    let mut line_nums_sort: [u8; 23] = [0; 23];

    let bytes = s.as_bytes();

    let mut temp_num: u8 = 0;
    let mut a = 0;
    let mut i = 0;
    loop {
        let c = bytes.get_unchecked(i);
        match c {
            b'|' => {
                a = temp_num;
                temp_num = 0;
            }
            b'\n' => {
                if a == 0 {
                    break;
                } else {
                    cache[a as usize][temp_num as usize] = 1;
                }
                temp_num = 0;
                a = 0;
            }
            _ => temp_num = temp_num.unchecked_mul(10).unchecked_add(c.unchecked_sub(b'0').into()),
        }
        i = i.unchecked_add(1);
    }
    let mut prev_num = 0;
    let mut line_ok = true;
    let mut line_nums_i = 0;
    i += 1;
    loop {
        let c = bytes.get_unchecked(i);
        match c {
            b',' => {
                if prev_num != 0 {
                    line_ok &= cache[prev_num as usize][temp_num as usize] == 1;
                }
                line_nums[line_nums_i] = temp_num;
                line_nums_i += 1;
                prev_num = temp_num;
                temp_num = 0;
            }
            b'\n' => {
                line_ok &= cache[prev_num as usize][temp_num as usize] == 1;
                if !line_ok {
                    line_nums[line_nums_i] = temp_num;
                    let target_index = (line_nums_i) /2;
                    let mut left = 0;
                    let mut right = line_nums_i;
                    while left < right {
                        let pivot = (left + right)/2;
                        let pivot_value = line_nums[pivot];
                        let mut storage = left;
                        line_nums[pivot] = line_nums[right];
                        line_nums[right] = pivot_value;
                        for j in left..right{
                            if cache[line_nums[j] as usize][pivot_value as usize] == 0 {
                                let temp = line_nums[storage];
                                line_nums[storage] = line_nums[j];
                                line_nums[j]=temp;
                                storage += 1;
                            }
                        }
                        line_nums[right]=line_nums[storage];
                        line_nums[storage]=pivot_value;
                        if storage < target_index {
                            left = storage+1;
                        } else {
                            right = storage;
                        }
                    }
                    part2 += line_nums[target_index] as u32;
                }
                line_nums_i = 0;
                line_ok = true;
                temp_num = 0;
                prev_num = 0;
                if i + 1 == bytes.len() {
                    break;
                }
            }
            _ => temp_num = temp_num * 10 + (c - b'0') as u8,
        }
        i = i.unchecked_add(1);
    }
    part2
}
}