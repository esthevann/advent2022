use std::{collections::VecDeque, fs::read_to_string};

use itertools::Itertools;

#[derive(Debug)]
struct Command {
    quantity: u32,
    origin: u32,
    destination: u32,
}

#[derive(Debug, Clone)]
struct Ship {
    crates: VecDeque<Crate>,
}

#[derive(Debug, Clone)]
struct Crate {
    value: char,
    ship: usize,
}

fn main() {
    let file = read_to_string("input/day5.txt").unwrap();

    // Split map and commands
    let (map, commands) = file.split_once("\n\n").unwrap();

    // Turn command phrase into struct
    let commands: Vec<Command> = commands
        .lines()
        .map(|x| x.split(' '))
        .map(|x| {
            let a = x.collect_vec();
            (a[1], a[3], a[5])
        })
        .map(|(x, y, z)| (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()))
        .map(|(quantity, origin, destination)| Command {
            quantity,
            origin,
            destination,
        })
        .collect();

    let mut crates: Vec<Crate> = Vec::new();

    // Get crate from map based on position on string
    for row in map.lines() {
        // position of alphabetic char on string minus
        // increasing multiples of 3 per iteration (j)
        // is the number of the ship it belongs to if i > 0 else i
        for (i, j) in (1..=35).step_by(4).zip((0..=35).step_by(3)) {
            let char = row.chars().nth(i as usize).unwrap();
            if char.is_alphabetic() {
                if i == 1 {
                    crates.push(Crate {
                        ship: i,
                        value: char,
                    });
                } else {
                    crates.push(Crate {
                        ship: i - j,
                        value: char,
                    });
                }
            }
        }
    }

    // create nine ships
    let mut ships = Vec::new();
    for _ in 1..10 {
        ships.push(Ship {
            crates: VecDeque::new(),
        })
    }

    // populate ships with crates
    for crate_ in crates {
        ships[crate_.ship as usize - 1].crates.push_back(crate_);
    }


    let mut ships_2 = ships.clone();

    // execute commands
    execute_command1(&commands, &mut ships);
    execute_command2(&commands, &mut ships_2);



    let word1 = get_word_from_ships(&ships);
    let word2 = get_word_from_ships(&ships_2);

    println!("Task 1: {word1}");
    println!("Task 2: {word2}");
}

fn get_word_from_ships(ships: &[Ship]) -> String {
    let mut word = String::new();
    for i in ships {
        if !i.crates.is_empty() {
            word.push(i.crates[0].value);
        }
    }
    word
}

fn execute_command1(commands: &[Command], ships: &mut [Ship]) {
    for command in commands {
        let crates_to_move = &ships[command.origin as usize - 1]
            .crates
            .drain(0..command.quantity as usize)
            .collect_vec();
        
        for i in crates_to_move.iter() {
            ships[command.destination as usize - 1].crates.push_front(i.to_owned())
        }
    }
}

fn execute_command2(commands: &[Command], ships: &mut [Ship]) {
    for command in commands {
        let crates_to_move = &ships[command.origin as usize - 1]
            .crates
            .drain(0..command.quantity as usize)
            .collect_vec();
        
        for i in crates_to_move.iter().rev() {
            ships[command.destination as usize - 1].crates.push_front(i.to_owned())
        }
    }
}