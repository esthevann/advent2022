use std::{fs::read_to_string, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Jokenpô {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Play {
    Win,
    Lose,
    Draw,
}
#[derive(Debug)]
struct PlayError;

impl FromStr for Play {
    type Err = PlayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Play::Lose),
            "Y" => Ok(Play::Draw),
            "Z" => Ok(Play::Win),
            _ => Err(PlayError),
        }
    }
}

impl Jokenpô {
    fn get_opposing(&self) -> Jokenpô {
        match self {
            Jokenpô::Rock => Jokenpô::Paper,
            Jokenpô::Paper => Jokenpô::Scissors,
            Jokenpô::Scissors => Jokenpô::Rock,
        }
    }
    fn play_match(player: Jokenpô, enemy: Jokenpô) -> u32 {
        let mut score = player.clone() as u32;

        match (player, enemy) {
            (Jokenpô::Rock, Jokenpô::Scissors) => score += 6,
            (Jokenpô::Paper, Jokenpô::Rock) => score += 6,
            (Jokenpô::Scissors, Jokenpô::Paper) => score += 6,
            (a, b) if a != b => {}
            (_, _) => score += 3,
        }

        score
    }

    fn from_row((enemy, player): (&str, &str)) -> (Jokenpô, Jokenpô) {
        let enemy: Jokenpô = enemy.parse().unwrap();
        let play: Play = player.parse().unwrap();

        let win = enemy.get_opposing();
        let los = win.get_opposing();

        let play = match play {
            Play::Win => win,
            Play::Lose => los,
            Play::Draw => enemy.clone(),
        };

        (enemy, play)
    }
}

#[derive(Debug)]
struct JokenpôError;

impl FromStr for Jokenpô {
    type Err = JokenpôError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        match s {
            "Y" | "B" => Ok(Self::Paper),
            "Z" | "C" => Ok(Self::Scissors),
            "X" | "A" => Ok(Self::Rock),
            _ => Err(JokenpôError),
        }
    }
}

fn main() {
    let file = read_to_string("input/day2.txt").unwrap();
    let rows: Vec<(&str, &str)> = file.lines().filter_map(|x| x.split_once(' ')).collect();

    task1(&rows);
    task2(&rows);
}

fn task1(vec: &[(&str, &str)]) {
    let num: u32 = vec
        .iter()
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .map(|(x, y)| Jokenpô::play_match(y, x))
        .sum();

    println!("Task 1: {num}");
}

fn task2(vec: &[(&str, &str)]) {
    let num: u32 = vec
        .iter()
        .map(|x| Jokenpô::from_row(*x))
        .map(|(x, y)| Jokenpô::play_match(y, x))
        .sum();
        

    println!("Task 2: {num}");
}
