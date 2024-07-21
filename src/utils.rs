use std::fs::{read_to_string, File};
use std::io::{self, BufRead};

pub fn file_to_string(year: &str, day: &str) -> String {
    read_to_string(format!("src/aoc_{year}/inputs/input_{day}.txt")).unwrap()
}

pub fn file_to_lines(year: &str, day: &str) -> Vec<String> {
    let file = File::open(format!("src/aoc_{year}/inputs/input_{day}.txt")).expect("No such file.");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Error parsing line"))
        .collect()
}
