use rayon::prelude::*;

use crate::utils;
use core::panic;
use std::time::Instant;

#[derive(Debug)]
struct Equation {
    numbers: Vec<usize>,
    target: usize,
}

impl Equation {
    fn try_solve(&self, idx: usize, current: usize, concat: bool) -> bool {
        if idx == self.numbers.len() {
            return current == self.target;
        }

        let add = self.try_solve(idx + 1, current + self.numbers[idx], concat);
        let mul = self.try_solve(idx + 1, current * self.numbers[idx], concat);
        let conc = concat && self.try_solve(idx + 1, concat_num(current, self.numbers[idx]), concat);
        add || mul || conc
    }

    fn calibration_result(&self, concat: bool) -> usize {
        self.try_solve(1, self.numbers[0], concat) as usize * self.target
    }
}

fn concat_num(a: usize, b: usize) -> usize {
    let mut b_digits = 10;
    let mut temp_b = b;
    while temp_b >= 10 {
        temp_b /= 10;
        b_digits *= 10;
    }
    a * b_digits + b
}

fn parse_input(lines: &[String]) -> Vec<Equation> {
    lines
        .iter()
        .map(|line| {
            if let Some((first, second)) = line.split_once(':') {
                Equation {
                    numbers: second
                        .split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect(),
                    target: first.parse().unwrap(),
                }
            } else {
                panic!("Invalid Input")
            }
        })
        .collect()
}

// Stub generated by Elf
pub fn solve() {
    // elf splits the printed output at the first <space> and takes writes the first half as the solution
    let equations = parse_input(&utils::file_to_lines("2024", "07"));
    let mut time = Instant::now();
    let p1 = part_one(&equations);
    println!("{:?} in {:?} for Part 1", p1, time.elapsed());
    time = Instant::now();
    let p2 = part_two(&equations);
    println!("{:?} in {:?} for Part 2", p2, time.elapsed());
}

fn part_one(equations: &[Equation]) -> usize {
    equations.iter().map(|e| e.calibration_result(false)).sum()
}
fn part_two(equations: &[Equation]) -> usize {
    equations.par_iter().map(|e| e.calibration_result(true)).sum()
}