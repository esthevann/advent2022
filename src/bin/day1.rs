use std::fs::read_to_string;

fn main() {
    let calories = get_calories();
    let task1 = task_1(&calories);
    println!("Task 1: {task1}");
    let task2 = task_2(&calories);
    println!("Task 2: {task2}");
}

fn get_calories() -> Vec<u32> {
    let file = read_to_string("input/day1.txt").unwrap();
    let split_by_elf: Vec<u32> = file
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()))
        .map(|x| x.sum())
        .collect();

    split_by_elf
}

fn task_1(vec: &[u32]) -> u32 {
    *vec.iter().max().unwrap()
}

fn task_2(vec: &[u32]) -> u32 {
    let mut vec_clone = vec.to_vec();
    vec_clone.sort_unstable();
    vec_clone.iter().rev().take(3).sum::<u32>()
}