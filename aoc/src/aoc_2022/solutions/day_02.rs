use core::panic;
use std::ops;
use crate::utils::file_to_lines;


#[derive(Clone, Copy, PartialEq)]
enum RPS {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3
}

impl RPS {

    fn loses_to(&self) -> RPS {
        match self {
            RPS::ROCK => RPS::PAPER,
            RPS::PAPER => RPS::SCISSORS,
            RPS::SCISSORS => RPS::ROCK,
        }
    }

    fn wins_against(&self) -> RPS {
        match self {
            RPS::ROCK => RPS::SCISSORS,
            RPS::PAPER => RPS::ROCK,
            RPS::SCISSORS => RPS::PAPER,
        }
    }
}

impl TryFrom<char> for RPS {
    type Error = ();

    fn try_from(v: char) -> Result<Self, Self::Error> {
        match v {
            x if x == 'A' || x == 'X' => Ok(RPS::ROCK),
            x if x == 'B' || x == 'Y' => Ok(RPS::PAPER),
            x if x == 'C' || x == 'Z' => Ok(RPS::SCISSORS),
            _ => Err(()),
        }
    }
}

impl ops::Add<RPS> for RPS {
    type Output = u8;

    fn add(self, your_choice: RPS) -> u8 {
        if self.loses_to() == your_choice {
            6 + (your_choice as u8)
        } else if self.wins_against() == your_choice {
            0 + (your_choice as u8)
        } else {
            3 + (your_choice as u8)
        }
    }
}

pub fn solve() {
    let sol_1 = solution_1();
    let sol_2 = solution_2();
    println!("Day 2 | Part 1: {:?}\nDay 2 | Part 2: {:?}", sol_1, sol_2);
}

fn solution_1() -> u32 {
    let mut temp_sum: u32 = 0;
    for line in file_to_lines("2022", "02") {
        let opp_choice: RPS;
            let your_choice: RPS;
            if let Some(x) = line.chars().nth(0) {
                opp_choice = x.try_into().unwrap();
                if let Some(y) = line.chars().nth(2) {
                    your_choice = y.try_into().unwrap();
                    temp_sum += (opp_choice + your_choice) as u32
                }
            }
    }
    temp_sum
}

fn solution_2() -> u32 {
    let mut temp_sum: u32 = 0;
    for line in file_to_lines("2022", "02") {
        let opp_choice: RPS;
            let your_choice: RPS;
            if let Some(x) = line.chars().nth(0) {
                opp_choice = x.try_into().unwrap();
                if let Some(y) = line.chars().nth(2) {
                    your_choice = match y {
                        'X' => opp_choice.wins_against(),
                        'Y' => opp_choice,
                        'Z' => opp_choice.loses_to(),
                        _ => panic!("Error parsing input.")
                    };
                    temp_sum += (opp_choice + your_choice) as u32
                }
            }
    }
    temp_sum
}