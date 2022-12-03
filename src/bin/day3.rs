use itertools::{Itertools, Chunk};
use std::{collections::HashSet, fs::read_to_string, vec::IntoIter, slice::Iter};

fn main() {
    let file = read_to_string("input/day3.txt").unwrap();

    let lines: Vec<&str> = file
        .lines()
        .collect();

    task1(&lines);
    task2(&lines);
}

#[allow(clippy::needless_collect)]
fn get_common_char<'a>((a, b): (&'a str, &'a str)) -> IntoIter<char> {
    let set: HashSet<char> = a.chars().collect();
    let set2: HashSet<char> = b.chars().collect();

    let common_letters: Vec<char> = set.intersection(&set2).copied().collect();
    common_letters.into_iter()
}

fn get_common_char_iter(a: Chunk<Iter<&str>>) -> HashSet<char> {
    let mut vec: Vec<HashSet<char>> = a.map(|x| x.chars().collect::<HashSet<char>>()).collect();

    let mut one = vec.pop().unwrap();
    one.retain(|item| {
        vec.iter().all(|set| set.contains(item))
    });
    one
}


pub trait Score {
    fn score(self) -> u32;
}

impl<T> Score for T
where
    T: Iterator<Item = char>,
{
    fn score(self) -> u32 {
        self.into_iter()
            .filter(|x| x.is_ascii())
            .map(|x| {
                if x.is_lowercase() {
                    x as u32 - 96
                } else {
                    x as u32 - 64 + 26
                }
            })
            .sum()
    }
}


fn task1(vec: &[&str]) {
    let score = vec
        .iter()
        .map(|x| x.split_at(x.len() / 2))
        .flat_map(get_common_char)
        .score();

    println!("Task 1: {score}");
}

fn task2(vec: &[&str]) {
    let score: u32 = vec
        .iter()
        .chunks(3)
        .into_iter()
        .flat_map(get_common_char_iter)
        .score();
        
    println!("Task 2: {score}");
}
