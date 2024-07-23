use crate::utils;
use std::time::Instant;

pub fn solve() {
    let lines = utils::file_to_lines("2023", "01");
    let mut time = Instant::now();
    println!("Part 1: {:?} in {:?}", part_one(&lines), time.elapsed());
    time = Instant::now();
    println!("Part 2: {:?} in {:?}", part_one(&lines), time.elapsed());
}

fn part_one(lines: &[String]) -> usize {
    0
}
fn part_two(lines: &[String]) -> usize {
    0
}
