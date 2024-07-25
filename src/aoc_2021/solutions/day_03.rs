use crate::utils;
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    time::Instant,
};

// Stub generated by templar
pub fn solve() {
    let mut lines = utils::file_to_lines("2021", "03");
    let mut time = Instant::now();
    println!("Part 1: {:?} in {:?}", part_one(&lines), time.elapsed());
    time = Instant::now();
    println!("Part 2: {:?} in {:?}", part_two(lines), time.elapsed());
}

fn part_one(lines: &[String]) -> usize {
    let mut bit_map = [0; 12];
    lines.iter().for_each(|bin| {
        bin.chars().enumerate().for_each(|(pos, bit)| match bit {
            '0' => bit_map[pos] -= 1,
            '1' => bit_map[pos] += 1,
            _ => panic!("That's not a binary."),
        })
    });

    let mut inverse = 0;
    bit_map.iter().enumerate().fold(0usize, |acc, (pos, bit)| {
        if bit > &0 {
            acc + 2usize.pow((11 - pos) as u32)
        } else {
            inverse += 2usize.pow((11 - pos) as u32);
            acc
        }
    }) * inverse
}
fn part_two(mut lines: Vec<String>) -> i64 {
    let mut line_copy = lines.clone();
    let oxy_bin = reduce_lines(lines.clone(), ('0', '1'));
    let scrub_bin = reduce_lines(lines, ('1', '0'));
    let oxy = i64::from_str_radix(&oxy_bin, 2).unwrap();
    let scrub = i64::from_str_radix(&scrub_bin, 2).unwrap();
    oxy * scrub
}

fn reduce_lines(mut lines: Vec<String>, ch: (char, char)) -> String {
    for i in 0..12 {
        let bit_crit = lines
            .iter()
            .map(|bin| bin.chars().nth(i).unwrap())
            .fold(0, |acc, c| if c == '1' { acc + 1 } else { acc - 1 });
        match bit_crit.cmp(&0) {
            Less => lines.retain(|bin| bin.chars().nth(i).unwrap() == ch.0),
            Equal | Greater => lines.retain(|bin| bin.chars().nth(i).unwrap() == ch.1),
        }
        if lines.len() == 1 {
            break;
        }
    }
    lines[0].clone()
}