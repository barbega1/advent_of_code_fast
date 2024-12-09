use std::fmt::Display;

#[derive(Default, Copy, Clone, Debug)]
struct Frequency {
    x1: i8,
    y1: i8,
    x2: i8,
    y2: i8,
    x3: i8,
    y3: i8,
    x4: i8,
    y4: i8,
    num: u8,
}

#[inline]
pub fn set_grid(x1:i8, y1:i8, x2:i8, y2:i8, state: &mut [[u8; 50]; 50]) {
    let mut x = x2 + x2 - x1;
    let mut y = y2 + y2 - y1;
    if x >= 0 && x < 50 && y >= 0 && y < 50 {
        state[y as usize][x as usize] = 1;
    }
    x = x1 + x1 - x2;
    y = y1 + y1 - y2;
    if x >= 0 && x < 50 && y >= 0 && y < 50 {
        state[y as usize][x as usize] = 1;
    }
}

pub fn part1(s: &str) -> impl Display {
    let mut frequencies: [Frequency;172] = [Frequency::default();172];
    let mut part1: u32 = 0;
    let mut x = 0;
    let mut y = 0;
    for line in s.lines() {
        x = 0;
        for c in line.chars() {
            match c {
                '.' => {},
                _ => {
                    let index: usize = c as usize;
                    match frequencies[index].num {
                        0 => {
                            frequencies[index].x1 = x;
                            frequencies[index].y1 = y;
                            frequencies[index].num = 1;
                        },
                        1 => {
                            frequencies[index].x2 = x;
                            frequencies[index].y2 = y;
                            frequencies[index].num = 2;
                        },
                        2 => {
                            frequencies[index].x3 = x;
                            frequencies[index].y3 = y;
                            frequencies[index].num = 3;
                        },
                        3 => {
                            frequencies[index].x4 = x;
                            frequencies[index].y4 = y;
                            frequencies[index].num = 4;
                        },
                        _ => {
                            panic!("Invalid frequency number");
                        }
                    }
                }
            }
            x += 1;
        }
        y += 1
    }
    let mut state = [[0; 50]; 50];
    for freq in frequencies {
        match freq.num {
            0 => {},
            1 => {},
            2 => {
                // One line between 1 and 2
                set_grid(freq.x1, freq.y1, freq.x2, freq.y2, &mut state);
            },
            3 => {
                // Three lines
                set_grid(freq.x1, freq.y1, freq.x2, freq.y2, &mut state);
                set_grid(freq.x2, freq.y2, freq.x3, freq.y3, &mut state);
                set_grid(freq.x3, freq.y3, freq.x1, freq.y1, &mut state);
            },
            4 => {
                // Six lines
                set_grid(freq.x1, freq.y1, freq.x2, freq.y2, &mut state);
                set_grid(freq.x2, freq.y2, freq.x3, freq.y3, &mut state);
                set_grid(freq.x3, freq.y3, freq.x4, freq.y4, &mut state);
                set_grid(freq.x4, freq.y4, freq.x1, freq.y1, &mut state);
                set_grid(freq.x1, freq.y1, freq.x3, freq.y3, &mut state);
                set_grid(freq.x2, freq.y2, freq.x4, freq.y4, &mut state);
            },
            _ => {
                panic!("Invalid frequency number");
            }
        }
    }
    for i in 0..50 {
        for j in 0..50 {
            part1 += state[i][j] as u32;
        }
    }
    part1
}

#[inline]
pub fn set_grid2(x1:i8, y1:i8, x2:i8, y2:i8, state: &mut [[u8; 50]; 50]) {
    let mut diff_x = x1 - x2;
    let mut diff_y = y1 - y2;
    for i in 0..50 {
        let x = x1 + (diff_x * i);
        let y = y1 + (diff_y * i);
        if x >= 0 && x < 50 && y >= 0 && y < 50 {
            state[y as usize][x as usize] = 1;
        } else {
            break;
        }
    }
    for i in 0..50 {
        let x = x2 - (diff_x * i);
        let y = y2 - (diff_y * i);
        if x >= 0 && x < 50 && y >= 0 && y < 50 {
            state[y as usize][x as usize] = 1;
        } else {
            break;
        }
    }
}

pub fn part2(s: &str) -> impl Display {
    let mut frequencies: [Frequency;172] = [Frequency::default();172];
    let mut part2: u32 = 0;
    let mut x = 0;
    let mut y = 0;
    for line in s.lines() {
        x = 0;
        for c in line.chars() {
            match c {
                '.' => {},
                _ => {
                    let index: usize = c as usize;
                    match frequencies[index].num {
                        0 => {
                            frequencies[index].x1 = x;
                            frequencies[index].y1 = y;
                            frequencies[index].num = 1;
                        },
                        1 => {
                            frequencies[index].x2 = x;
                            frequencies[index].y2 = y;
                            frequencies[index].num = 2;
                        },
                        2 => {
                            frequencies[index].x3 = x;
                            frequencies[index].y3 = y;
                            frequencies[index].num = 3;
                        },
                        3 => {
                            frequencies[index].x4 = x;
                            frequencies[index].y4 = y;
                            frequencies[index].num = 4;
                        },
                        _ => {
                            panic!("Invalid frequency number");
                        }
                    }
                }
            }
            x += 1;
        }
        y += 1
    }
    let mut state = [[0; 50]; 50];
    for freq in frequencies {
        match freq.num {
            0 => {},
            1 => {},
            2 => {
                // One line between 1 and 2
                set_grid2(freq.x1, freq.y1, freq.x2, freq.y2, &mut state);
            },
            3 => {
                // Three lines
                set_grid2(freq.x1, freq.y1, freq.x2, freq.y2, &mut state);
                set_grid2(freq.x2, freq.y2, freq.x3, freq.y3, &mut state);
                set_grid2(freq.x3, freq.y3, freq.x1, freq.y1, &mut state);
            },
            4 => {
                // Six lines
                set_grid2(freq.x1, freq.y1, freq.x2, freq.y2, &mut state);
                set_grid2(freq.x2, freq.y2, freq.x3, freq.y3, &mut state);
                set_grid2(freq.x3, freq.y3, freq.x4, freq.y4, &mut state);
                set_grid2(freq.x4, freq.y4, freq.x1, freq.y1, &mut state);
                set_grid2(freq.x1, freq.y1, freq.x3, freq.y3, &mut state);
                set_grid2(freq.x2, freq.y2, freq.x4, freq.y4, &mut state);
            },
            _ => {
                panic!("Invalid frequency number");
            }
        }
    }
    for i in 0..50 {
        for j in 0..50 {
            part2 += state[i][j] as u32;
        }
    }
    part2
}
