use std::{collections::HashSet, time::Instant};

use crate::utils::file_to_string;

pub fn solve() {
    //naive/intuitive version
    let now = Instant::now();
    let sequence = file_to_string("2022", "06").chars().collect::<Vec<char>>();
    let sol_1 = solution_1(&sequence);
    let sol_2 = solution_2(&sequence);
    println!("Day 6 | Part 1: {:?}\nDay 6 | Part 2: {:?}", sol_1, sol_2);
    println!("Took: {:?}", now.elapsed());

    let compare = Instant::now();

    // bitcounting version. much faster
    // shifts once by char value compared to 'a', counts 1's for each window.
    // matches window size if all chars are unique
    let scan = |size| size + include_bytes!("input.txt")
        .windows(size)
        .position(|w| w.iter().fold(0u32, |c, b| c | 1 << b - b'a').count_ones() as usize == size)
        .unwrap();

    println!("Day 6 | Part 1: {:?}\nDay 6 | Part 2: {:?}", scan(4), scan(14));
    println!("Took: {:?}", compare.elapsed());
}

fn solution_1(sequence: &Vec<char>) -> usize {
    index_of_unique_window(sequence, 4)
}

fn solution_2(sequence: &Vec<char>) -> usize {
    index_of_unique_window(sequence, 14)
}

fn index_of_unique_window(sequence: &Vec<char>, window_size: usize) -> usize {
    let marker = sequence
        .windows(window_size)
        .enumerate()
        .find(|(_, w)| HashSet::<_>::from_iter(w.iter()).len() == window_size)
        .unwrap();
    marker.0 + window_size
}