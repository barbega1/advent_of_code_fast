use std::fmt::Display;
use std::intrinsics::unchecked_div;

pub fn part1(s: &str) -> impl Display {
    unsafe { inner1(s) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner1(s: &str) -> impl Display {
    let mut part1: i64 = 0;
    let bytes = s.as_bytes();

    let mut numbers = [0; 12];
    let mut original_target: i64 = 0;
    let mut number_index = 0;
    let mut temp_num: i64 = 0;
    for c in bytes {
        match c {
            b':' => {
                original_target = temp_num;
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
                let mut target = original_target;
                let mut i = number_index - 1;
                loop {
                    while i > 0 {
                        let small = *numbers.get_unchecked(i) as i64;
                        if unchecked_div(target, small) * small as i64 == target {
                            target = unchecked_div(target, small);
                        } else {
                            *operators.get_unchecked_mut(i) = 1;
                            target = target - small as i64;
                            if target < 0 {
                                for j in i..number_index {
                                    if *operators.get_unchecked_mut(j) == 1 {
                                        *operators.get_unchecked_mut(j) = 0;
                                        target += *numbers.get_unchecked(j) as i64;
                                    } else {
                                        target *= *numbers.get_unchecked(j) as i64;
                                        target -= *numbers.get_unchecked(j) as i64;
                                        *operators.get_unchecked_mut(j) = 1;
                                        i = j;
                                        break;
                                    }
                                }
                            }
                        }
                        i -= 1
                    }
                    if *numbers.get_unchecked(0) as i64 == target {
                        part1 += original_target;
                        break;
                    } else {
                        i = 100;
                        for j in 1..number_index {
                            if *operators.get_unchecked_mut(j) == 1 {
                                *operators.get_unchecked_mut(j) = 0;
                                target += *numbers.get_unchecked(j) as i64;
                            } else {
                                target *= *numbers.get_unchecked(j) as i64;
                                target -= *numbers.get_unchecked(j) as i64;
                                *operators.get_unchecked_mut(j) = 1;
                                i = j - 1;
                                break;
                            }
                        }
                        if i == 100 {
                            break;
                        }
                    }
                }
                number_index = 0;
            }
            _ => {
                temp_num = temp_num * 10 + (c - b'0') as i64;
            }
        }
    }
    part1
}

pub fn part2(s: &str) -> impl Display {
    unsafe { inner2(s) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner2(s: &str) -> impl Display {
    let mut part2: i64 = 0;
    let bytes = s.as_bytes();

    let mut numbers = [0; 12];
    let mut original_target: i64 = 0;
    let mut number_index = 0;
    let mut temp_num: i64 = 0;
    for c in bytes {
        match c {
            b':' => {
                original_target = temp_num;
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
                let mut target = original_target;
                let mut i = number_index - 1;
                loop {
                    while i > 0 {
                        let small = *numbers.get_unchecked(i) as i64;
                        if small < 10 && (unchecked_div(target, 10) * 10) + small == target {
                            target = unchecked_div(target, 10) as i64;
                            *operators.get_unchecked_mut(i) = 2;
                        } else if small < 100
                            && (unchecked_div(target, 100) * 100) + small == target
                        {
                            target = unchecked_div(target, 100) as i64;
                            *operators.get_unchecked_mut(i) = 2;
                        } else if (unchecked_div(target, 1000) * 1000) + small == target {
                            target = unchecked_div(target, 1000) as i64;
                            *operators.get_unchecked_mut(i) = 2;
                        } else if (unchecked_div(target, small) as i64) * small as i64 == target {
                            target = unchecked_div(target, small) as i64;
                        } else {
                            *operators.get_unchecked_mut(i) = 1;
                            target = target - small as i64;
                            if target < 0 {
                                for j in i..number_index {
                                    if *operators.get_unchecked_mut(j) == 2 {
                                        *operators.get_unchecked_mut(j) = 0;
                                        if *numbers.get_unchecked(j) < 10 {
                                            target = target * 10 + *numbers.get_unchecked(j) as i64;
                                        } else if *numbers.get_unchecked(j) < 100 {
                                            target =
                                                target * 100 + *numbers.get_unchecked(j) as i64;
                                        } else {
                                            target =
                                                target * 1000 + *numbers.get_unchecked(j) as i64;
                                        }
                                        if (unchecked_div(target, *numbers.get_unchecked(j) as i64)
                                            as i64)
                                            * *numbers.get_unchecked(j) as i64
                                            == target
                                        {
                                            target = unchecked_div(
                                                target,
                                                *numbers.get_unchecked(j) as i64,
                                            )
                                                as i64;
                                            *operators.get_unchecked_mut(j) = 0;
                                        } else {
                                            target -= *numbers.get_unchecked(j) as i64;
                                            *operators.get_unchecked_mut(j) = 1;
                                        }
                                        i = j;
                                        break;
                                    } else if *operators.get_unchecked_mut(j) == 1 {
                                        *operators.get_unchecked_mut(j) = 0;
                                        target += *numbers.get_unchecked(j) as i64;
                                    } else {
                                        target *= *numbers.get_unchecked(j) as i64;
                                        target -= *numbers.get_unchecked(j) as i64;
                                        *operators.get_unchecked_mut(j) = 1;
                                        i = j;
                                        break;
                                    }
                                }
                            }
                        }
                        i -= 1
                    }
                    if *numbers.get_unchecked(0) as i64 == target {
                        part2 += original_target;
                        break;
                    } else {
                        i = 100;
                        for j in 1..number_index {
                            if *operators.get_unchecked_mut(j) == 2 {
                                if *numbers.get_unchecked(j) < 10 {
                                    target = target * 10 + *numbers.get_unchecked(j) as i64;
                                } else if *numbers.get_unchecked(j) < 100 {
                                    target = target * 100 + *numbers.get_unchecked(j) as i64;
                                } else {
                                    target = target * 1000 + *numbers.get_unchecked(j) as i64;
                                }
                                if (unchecked_div(target, *numbers.get_unchecked(j) as i64) as i64)
                                    * *numbers.get_unchecked(j) as i64
                                    == target
                                {
                                    target = unchecked_div(target, *numbers.get_unchecked(j) as i64)
                                        as i64;
                                    *operators.get_unchecked_mut(j) = 0;
                                } else {
                                    target -= *numbers.get_unchecked(j) as i64;
                                    *operators.get_unchecked_mut(j) = 1;
                                }
                                i = j - 1;
                                break;
                            } else if *operators.get_unchecked_mut(j) == 1 {
                                *operators.get_unchecked_mut(j) = 0;
                                target += *numbers.get_unchecked(j) as i64;
                            } else {
                                target *= *numbers.get_unchecked(j) as i64;
                                target -= *numbers.get_unchecked(j) as i64;
                                *operators.get_unchecked_mut(j) = 1;
                                i = j - 1;
                                break;
                            }
                        }
                        if i == 100 {
                            break;
                        }
                    }
                }
                number_index = 0;
            }
            _ => {
                temp_num = temp_num * 10 + (c - b'0') as i64;
            }
        }
    }
    part2
}
