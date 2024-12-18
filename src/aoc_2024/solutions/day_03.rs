use crate::utils;
use std::time::Instant;
use regex::Regex;

// Stub generated by Elf
pub fn solve() {
    // elf splits the printed output at the first <space> and takes writes the first half as the solution
    let input = utils::file_to_string("2024", "03");
    let mut time = Instant::now();
    let p1 = part_one(&input);
    println!("{:?} in {:?} for Part 1", p1, time.elapsed());
    time = Instant::now();
    let p2 = part_two(&input);
    println!("{:?} in {:?} for Part 2", p2, time.elapsed());
}

fn part_one(input: &str) -> usize {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    regex.captures_iter(input).map(|cap| {
        let x: usize = cap[1].parse().unwrap();
        let y: usize = cap[2].parse().unwrap();
        x * y
    }).sum()
}
fn part_two(input: &str) -> usize {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)").unwrap();
    let mut do_mul = true;
    regex.captures_iter(input).map(|cap| {
        if cap[0].starts_with("mul") && do_mul {
            let x: usize = cap[1].parse().unwrap();
            let y: usize = cap[2].parse().unwrap();
            x * y
        } else {
            do_mul = cap[0] == *"do()";
            0
        }
    }).sum()
}
