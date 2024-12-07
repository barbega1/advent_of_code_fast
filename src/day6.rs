use std::fmt::Display;

const SIZE: usize = 130;

pub fn part1(s: &str) -> impl Display {
    let mut part1 : u32 = 0;
    let mut state : [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];
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
                _ => panic!("Invalid character")
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
                } else if state[guard_y-1][guard_x] != 1 {
                    guard_y -= 1;
                    state[guard_y][guard_x] = 2;
                } else {
                    guard_dir = 1;
                }
            },
            1 => {
                if guard_x == SIZE - 1 {
                    break;
                } else if state[guard_y][guard_x + 1] != 1 {
                    guard_x += 1;
                    state[guard_y][guard_x] = 2;
                } else {
                    guard_dir = 2;
                }
            },
            2 => {
                if guard_y == SIZE - 1 {
                    break;
                } else if state[guard_y+1][guard_x] != 1 {
                    guard_y += 1;
                    state[guard_y][guard_x] = 2;
                } else {
                    guard_dir = 3;
                }
            },
            3 => {
                if guard_x == 0 {
                    break;
                } else if state[guard_y][guard_x - 1] != 1 {
                    guard_x -= 1;
                    state[guard_y][guard_x] = 2;
                } else {
                    guard_dir = 0;
                }
            },
            _ => panic!("Invalid direction")
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

pub fn part2(s: &str) -> impl Display {
    let mut part2 : u32 = 0;
    let mut state : [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    let mut map : [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    let mut i = 0;
    let mut j;
    let mut guard_x = 0;
    let mut guard_y = 0;
    let mut guard_original_x = 0;
    let mut guard_original_y = 0;
    let mut guard_dir = 0;
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
                _ => panic!("Invalid character")
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
    loop {
        match guard_dir {
            0 => {
                if guard_y == 0 {
                    break;
                } else if state[guard_y-1][guard_x] != 1 {
                    guard_y -= 1;
                    state[guard_y][guard_x] = 2;
                } else {
                    guard_dir = 1;
                }
            },
            1 => {
                if guard_x == SIZE - 1 {
                    break;
                } else if state[guard_y][guard_x + 1] != 1 {
                    guard_x += 1;
                    state[guard_y][guard_x] = 2;
                } else {
                    guard_dir = 2;
                }
            },
            2 => {
                if guard_y == SIZE - 1 {
                    break;
                } else if state[guard_y+1][guard_x] != 1 {
                    guard_y += 1;
                    state[guard_y][guard_x] = 2;
                } else {
                    guard_dir = 3;
                }
            },
            3 => {
                if guard_x == 0 {
                    break;
                } else if state[guard_y][guard_x - 1] != 1 {
                    guard_x -= 1;
                    state[guard_y][guard_x] = 2;
                } else {
                    guard_dir = 0;
                }
            },
            _ => panic!("Invalid direction")
        }
    }
    
    for x_obstacle in 0..SIZE {
        for y_obstacle in 0..SIZE {
            if(state[y_obstacle][x_obstacle] != 2) {
                continue;
            }
            if x_obstacle == guard_original_x && y_obstacle == guard_original_y {
                continue;
            }
            let mut steps = 0;
            guard_dir = 0;
            guard_x = guard_original_x;
            guard_y = guard_original_y;
            map[y_obstacle][x_obstacle] = 1;
            loop {
                steps += 1;
                if steps > SIZE * SIZE {
                    part2 += 1;
                    break;
                }
                match guard_dir {
                    0 => {
                        if guard_y == 0 {
                            break;
                        } else if map[guard_y-1][guard_x] != 1 {
                            guard_y -= 1;
                        } else {
                            guard_dir = 1;
                        }
                    },
                    1 => {
                        if guard_x == SIZE - 1 {
                            break;
                        } else if map[guard_y][guard_x + 1] != 1 {
                            guard_x += 1;
                        } else {
                            guard_dir = 2;
                        }
                    },
                    2 => {
                        if guard_y == SIZE - 1 {
                            break;
                        } else if map[guard_y+1][guard_x] != 1 {
                            guard_y += 1;
                        } else {
                            guard_dir = 3;
                        }
                    },
                    3 => {
                        if guard_x == 0 {
                            break;
                        } else if map[guard_y][guard_x - 1] != 1 {
                            guard_x -= 1;
                        } else {
                            guard_dir = 0;
                        }
                    },
                    _ => panic!("Invalid direction")
                }
            }
            map[y_obstacle][x_obstacle] = 0;
        }
    }
    part2
}
