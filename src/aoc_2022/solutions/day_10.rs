use std::{time::Instant};
use crate::utils::file_to_lines;

pub fn solve() {
    let now = Instant::now();
    let mut ops: Vec<i32> = Vec::new();
    for op in file_to_lines("2022", "10").iter() {
        if op.starts_with("addx") {
            ops.push(0);
            ops.push(op.split(" ").last().unwrap().parse::<i32>().unwrap());
        } else { ops.push(0) }
    }

    println!("Day 10 | Part 1: {:?}\nDay 10 | Part 2:\n", solution_1(&ops));
    solution_2(&ops);
    println!("Took: {:?}", now.elapsed());
}

fn solution_1(ops: &Vec<i32>) -> i32 {
    let mut x_reg: i32 = 1;
    let mut signal_strength: i32 = 0;
    for (c, op) in ops.iter().enumerate() {
        if (c + 1) % 40 == 20 {
            signal_strength += (c + 1) as i32 * x_reg;
        }
        x_reg += op;
    }
    signal_strength
}

fn solution_2(ops: &Vec<i32>) {
    let mut pixels: Vec<Vec<char>> = vec![vec!['.'; 40]; 6];
    let mut current_pos: Vec<i32> = vec![0, 1, 2];
    for (pixel, op) in ops.iter().enumerate() {
        let ps = (pixel % 40) as i32;
        if current_pos.contains(&ps) {
            pixels[pixel / 40][pixel % 40] = '#';
        }
        current_pos = current_pos.iter().map(|pos| pos + op).collect();
    }
    for row in pixels {
        println!("{:?}", row);
    }
}