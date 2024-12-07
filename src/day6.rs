use std::fmt::Display;

const SIZE: usize = 130;

pub fn part1(s: &str) -> impl Display {
    let mut part1: u32 = 0;
    let mut state: [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    let mut i = 0;
    let mut j;
    let mut guard_x = 0;
    let mut guard_y = 0;
    let mut guard_dir = 0;
    for line in s.lines() {
        j = 0;
        for c in line.chars() {
            state[i][j] = match c {
                '.' => 0,
                '#' => 1,
                '^' => {
                    guard_x = j;
                    guard_y = i;
                    2
                }
                _ => panic!("Invalid character"),
            };
            j += 1;
            if j >= SIZE {
                break;
            }
        }
        i += 1;
        if i >= SIZE {
            break;
        }
    }
    loop {
        match guard_dir {
            0 => {
                if guard_y == 0 {
                    break;
                } else if state[guard_y - 1][guard_x] != 1 {
                    guard_y -= 1;
                    state[guard_y][guard_x] = 2;
                } else {
                    guard_dir = 1;
                }
            }
            1 => {
                if guard_x == SIZE - 1 {
                    break;
                } else if state[guard_y][guard_x + 1] != 1 {
                    guard_x += 1;
                    state[guard_y][guard_x] = 2;
                } else {
                    guard_dir = 2;
                }
            }
            2 => {
                if guard_y == SIZE - 1 {
                    break;
                } else if state[guard_y + 1][guard_x] != 1 {
                    guard_y += 1;
                    state[guard_y][guard_x] = 2;
                } else {
                    guard_dir = 3;
                }
            }
            3 => {
                if guard_x == 0 {
                    break;
                } else if state[guard_y][guard_x - 1] != 1 {
                    guard_x -= 1;
                    state[guard_y][guard_x] = 2;
                } else {
                    guard_dir = 0;
                }
            }
            _ => panic!("Invalid direction"),
        }
    }

    for i in 0..SIZE {
        for j in 0..SIZE {
            if state[i][j] == 2 {
                part1 += 1;
            }
        }
    }
    part1
}

enum Dir {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

use crate::day6::Dir::{Down, Left, Right, Up};

pub fn part2(s: &str) -> impl Display {
    let mut part2: u32 = 0;
    let mut state: [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    let mut map: [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    let mut i = 0;
    let mut j;
    let mut guard_x = 0;
    let mut guard_y = 0;
    let mut guard_original_x = 0;
    let mut guard_original_y = 0;
    let mut guard_dir: Dir = Up;
    for line in s.lines() {
        j = 0;
        for c in line.chars() {
            state[i][j] = match c {
                '.' => 0,
                '#' => 1,
                '^' => {
                    guard_x = j;
                    guard_original_x = j;
                    guard_y = i;
                    guard_original_y = i;
                    2
                }
                _ => panic!("Invalid character"),
            };
            map[i][j] = state[i][j];
            j += 1;
            if j >= SIZE {
                break;
            }
        }
        i += 1;
        if i >= SIZE {
            break;
        }
    }
    let mut original_steps = 0;
    loop {
        original_steps += 1;
        match guard_dir {
            Up => {
                if guard_y == 0 {
                    break;
                } else if state[guard_y - 1][guard_x] != 1 {
                    guard_y -= 1;
                    state[guard_y][guard_x] = 2;
                } else {
                    guard_dir = Right;
                }
            }
            Right => {
                if guard_x == SIZE - 1 {
                    break;
                } else if state[guard_y][guard_x + 1] != 1 {
                    guard_x += 1;
                    state[guard_y][guard_x] = 2;
                } else {
                    guard_dir = Down;
                }
            }
            Down => {
                if guard_y == SIZE - 1 {
                    break;
                } else if state[guard_y + 1][guard_x] != 1 {
                    guard_y += 1;
                    state[guard_y][guard_x] = 2;
                } else {
                    guard_dir = Left;
                }
            }
            Left => {
                if guard_x == 0 {
                    break;
                } else if state[guard_y][guard_x - 1] != 1 {
                    guard_x -= 1;
                    state[guard_y][guard_x] = 2;
                } else {
                    guard_dir = Up;
                }
            }
        }
    }

    for x_obstacle in 0..SIZE {
        for y_obstacle in 0..SIZE {
            if (state[y_obstacle][x_obstacle] != 2) {
                continue;
            }
            if x_obstacle == guard_original_x && y_obstacle == guard_original_y {
                continue;
            }
            let mut steps_limit = original_steps + 2000;
            let mut steps = 0;
            guard_dir = Up;
            guard_x = guard_original_x;
            guard_y = guard_original_y;
            let mut hit_obstacle = false;
            map[y_obstacle][x_obstacle] = 1;
            loop {
                unsafe {
                    steps += 1;
                    if steps > steps_limit {
                        part2 += 1;
                        break;
                    }
                    match guard_dir {
                        Up => {
                            if guard_y == 0 {
                                break;
                            } else if *map.get_unchecked(guard_y - 1).get_unchecked(guard_x) != 1 {
                                guard_y -= 1;
                            } else {
                                if(y_obstacle == guard_y -1 && x_obstacle == guard_x && !hit_obstacle) {
                                    steps_limit = steps + 5000;
                                    hit_obstacle = true;
                                }
                                guard_dir = Right;
                            }
                        }
                        Right => {
                            if guard_x == SIZE - 1 {
                                break;
                            } else if *map.get_unchecked(guard_y).get_unchecked(guard_x + 1) != 1 {
                                guard_x += 1;
                            } else {
                                if(y_obstacle == guard_y && x_obstacle == guard_x + 1 && !hit_obstacle) {
                                    steps_limit = steps + 5000;
                                    hit_obstacle = true;
                                }
                                guard_dir = Down;
                            }
                        }
                        Down => {
                            if guard_y == SIZE - 1 {
                                break;
                            } else if *map.get_unchecked(guard_y + 1).get_unchecked(guard_x) != 1 {
                                guard_y += 1;
                            } else {
                                if(y_obstacle == guard_y + 1 && x_obstacle == guard_x && !hit_obstacle) {
                                    hit_obstacle = true;
                                    steps_limit = steps + 5000;
                                }
                                guard_dir = Left;
                            }
                        }
                        Left => {
                            if guard_x == 0 {
                                break;
                            } else if *map.get_unchecked(guard_y).get_unchecked(guard_x - 1) != 1 {
                                guard_x -= 1;
                            } else {
                                if(y_obstacle == guard_y && x_obstacle == guard_x - 1 && !hit_obstacle) {
                                    hit_obstacle = true;
                                    steps_limit = steps + 5000;
                                }
                                guard_dir = Up;
                            }
                        }
                    }
                }
            }
            map[y_obstacle][x_obstacle] = 0;
        }
    }
    part2
}
