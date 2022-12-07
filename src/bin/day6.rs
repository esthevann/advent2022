use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day6.txt").unwrap();
    let chars: Vec<char> = file.chars().collect();

    println!("Task 1: {}", get_start_of_message_position(4, &chars));
    println!("Task 2: {}", get_start_of_message_position(14, &chars));
}

fn get_start_of_message_position(windows_size: usize, chars: &[char]) -> usize{
    let mut finish = 0;

    for (i, w) in chars.windows(windows_size).enumerate() {
        let mut clone = w.clone().to_vec();
        clone.sort();
        clone.dedup();
        if clone.len() == windows_size {
            finish = i + windows_size;
            break;
        }
    }

    finish
}