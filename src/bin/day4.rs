// Clippy giving false positives on these
#![allow(clippy::useless_conversion)]
#![allow(clippy::unnecessary_to_owned)]

use std::{
    collections::{hash_map::RandomState, HashSet},
    fs::read_to_string,
    ops::RangeInclusive,
};

fn main() {
    let file = read_to_string("input/day4.txt").unwrap();

    let numbers: Vec<((u32, u32), (u32, u32))> = file
        .lines()
        .map(|x| x.split_once(',').unwrap())
        .map(|(x, y)| (x.split_once('-').unwrap(), y.split_once('-').unwrap()))
        .map(|(x, y)| {
            (
                (x.0.parse::<u32>().unwrap(), x.1.parse::<u32>().unwrap()),
                (y.0.parse::<u32>().unwrap(), y.1.parse::<u32>().unwrap()),
            )
        })
        .collect();

    task1(&numbers);
    task2(&numbers);
}

fn check_if_range_contains(b: &RangeInclusive<u32>, c: &RangeInclusive<u32>) -> bool {
    let b: HashSet<u32, RandomState> = HashSet::from_iter(b.to_owned().into_iter());
    let c: HashSet<u32, RandomState> = HashSet::from_iter(c.to_owned().into_iter());

    b.is_subset(&c) || c.is_subset(&b)
}

fn check_if_range_overlaps(b: &RangeInclusive<u32>, c: &RangeInclusive<u32>) -> bool {
    let b: HashSet<u32, RandomState> = HashSet::from_iter(b.to_owned().into_iter());
    let c: HashSet<u32, RandomState> = HashSet::from_iter(c.to_owned().into_iter());

    b.intersection(&c).count() != 0 || c.intersection(&b).count() != 0
}

type SliceOfRanges<'a> = &'a [((u32, u32), (u32, u32))];

fn task1(vec: SliceOfRanges) {
    let num = vec
        .iter()
        .map(|(x, y)| (x.0..=x.1, y.0..=y.1))
        .filter(|(x, y)| check_if_range_contains(x, y))
        .count();

    println!("Task 1: {num}");
}

fn task2(vec: SliceOfRanges) {
    let num = vec
        .iter()
        .map(|(x, y)| (x.0..=x.1, y.0..=y.1))
        .filter(|(x, y)| check_if_range_overlaps(x, y))
        .count();

    println!("Task 2: {num}");
}

