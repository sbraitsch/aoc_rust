use itertools::Itertools;

use crate::utils;
use std::time::Instant;

// Stub generated by Elf
pub fn solve() {
    // elf splits the printed output at the first <space> and takes writes the first half as the solution
    let lines = utils::file_to_lines("2024", "02");
    let mut time = Instant::now();
    let numbers: Vec<Vec<isize>> = lines
        .iter()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect()
        })
        .collect();
    let p1 = part_one(&numbers);
    println!("{:?} in {:?} for Part 1", p1, time.elapsed());
    time = Instant::now();
    let p2 = part_two(&numbers);
    println!("{:?} in {:?} for Part 2", p2, time.elapsed());
}

fn part_one(lines: &[Vec<isize>]) -> usize {
    lines
        .iter()
        .filter(|report| check_continuous(report))
        .count()
}

fn part_two(lines: &[Vec<isize>]) -> usize {
    lines
        .iter()
        .filter(|report| {
            permutations(report).iter().any(|p| check_continuous(p))
        })
        .count()
}

fn check_continuous(numbers: &[isize]) -> bool {
    let diffs: Vec<isize> = numbers.iter().tuple_windows().map(|(a, b)| a - b).collect();
    diffs.iter().all(|&n| n > 0 && n.abs() <= 3) || diffs.iter().all(|&n| n < 0 && n.abs() <= 3)
}

fn permutations(numbers: &[isize]) -> Vec<Vec<isize>> {
    let mut res = Vec::new();

    for i in 0..numbers.len() {
        let mut permutation = numbers.to_vec();
        permutation.remove(i);
        res.push(permutation);
    }

    res
}