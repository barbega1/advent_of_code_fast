use std::{fs::read_to_string, hint::black_box};

use advent_of_code_fast::{day3, day5, day6, day7, day8};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn day3(c: &mut Criterion) {
    let s3 = read_to_string("./inputs/3.txt").unwrap();
    let s3 = s3.as_str();

    let s5 = read_to_string("./inputs/5.txt").unwrap();
    let s5 = s5.as_str();

    c.bench_function("day5 part1", |b| b.iter(|| day5::part1(black_box(s5))));
    c.bench_function("day5 part2", |b| b.iter(|| day5::part2(black_box(s5))));

    assert_eq!(
        day5::part1(s5).to_string(),
        read_to_string("./outputs/5p1.txt").unwrap(),
    );
    assert_eq!(
        day5::part2(s5).to_string(),
        read_to_string("./outputs/5p2.txt").unwrap(),
    );

    c.bench_function("day3 part1", |b| b.iter(|| day3::part1(black_box(s3))));
    c.bench_function("day3 part2", |b| b.iter(|| day3::part2(black_box(s3))));

    // assert_eq!(
    //     day3::part1(s3).to_string(),
    //     read_to_string("./outputs/3p1.txt").unwrap(),
    // );
    // assert_eq!(
    //     day3::part2(s3).to_string(),
    //     read_to_string("./outputs/3p2.txt").unwrap(),
    // );

    // let s6 = read_to_string("./inputs/6.txt").unwrap();
    // let s6 = s6.as_str();

    // c.bench_function("day6 part1", |b| b.iter(|| day6::part1(black_box(s6))));
    // c.bench_function("day6 part2", |b| b.iter(|| day6::part2(black_box(s6))));

    // assert_eq!(
    //     day6::part1(s6).to_string(),
    //     read_to_string("./outputs/6p1.txt").unwrap(),
    // );
    // assert_eq!(
    //     day6::part2(s6).to_string(),
    //     read_to_string("./outputs/6p2.txt").unwrap(),
    // );

    // let s7 = read_to_string("./inputs/7.txt").unwrap();
    // let s7 = s7.as_str();

    // c.bench_function("day7 part1", |b| b.iter(|| day7::part1(black_box(s7))));
    // c.bench_function("day7 part2", |b| b.iter(|| day7::part2(black_box(s7))));

    // assert_eq!(
    //     day7::part1(s7).to_string(),
    //     read_to_string("./outputs/7p1.txt").unwrap(),
    // );
    // assert_eq!(
    //     day7::part2(s7).to_string(),
    //     read_to_string("./outputs/7p2.txt").unwrap(),
    // );

    let s8 = read_to_string("./inputs/8.txt").unwrap();
    let s8 = s8.as_str();

    c.bench_function("day8 part1", |b| b.iter(|| day8::part1(black_box(s8))));
    c.bench_function("day8 part2", |b| b.iter(|| day8::part2(black_box(s8))));

    assert_eq!(
        day8::part1(s8).to_string(),
        read_to_string("./outputs/8p1.txt").unwrap(),
    );
    assert_eq!(
        day8::part2(s8).to_string(),
        read_to_string("./outputs/8p2.txt").unwrap(),
    );
}

criterion_group!(benches, day3);
criterion_main!(benches);
