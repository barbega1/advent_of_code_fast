#![feature(unchecked_shifts)]
extern crate advent_of_code_fast;
extern crate aoc_runner;
extern crate aoc_runner_derive;
use std::fs::read_to_string;
use std::hint::black_box;

use aoc_runner_derive::aoc_main;

//aoc_main! { lib = advent_of_code_fast }

pub mod day3;
pub mod day5;

fn main() {
    let s = read_to_string("./inputs/3.txt").unwrap();
    let s = s.as_str();
    day3::part1(black_box(s));
}
