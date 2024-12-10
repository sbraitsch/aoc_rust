use crate::utils;
use std::{time::Instant, usize};

fn parse_input(input: &str) -> (Vec<(usize, i32)>, Vec<(usize, i32)>) {
    let mut p1 = Vec::new();
    let mut p2 = Vec::new();
    input.trim().bytes().enumerate().for_each(|(idx, byte)| {
        let file_id = if idx % 2 == 0 { (idx / 2) as i32 } else { -1 };
        let val = byte - b'0';
        p1.extend((0..val).map(|_| (1, file_id)));
        p2.push((val as usize, file_id));
    });

    (p1, p2)
}

fn organize(mut files: Vec<(usize, i32)>) -> usize {
    let mut idx = files.len() - 1;
    while idx > 0 {
        let (file_size, file_id) = files[idx];
        if file_id == -1 {
            idx -= 1;
            continue;
        }
        if let Some(space_idx) = files[0..idx]
            .iter()
            .position(|&(space_size, id)| id == -1 && file_size <= space_size)
        {
            let space = files[space_idx].0;
            files[space_idx] = (file_size, file_id);
            files[idx] = (file_size, -1);
            if file_size < space {
                files.insert(space_idx + 1, (space - file_size, -1));
            }
        }
        idx -= 1;
    }
    checksum(&files)
}

fn checksum(compressed: &[(usize, i32)]) -> usize {
    compressed
        .iter()
        .flat_map(|&(size, id)| (0..size).map(move |_| id))
        .enumerate()
        .map(|(idx, num)| if num > 0 { idx * num as usize } else { 0 })
        .sum()
}

// Stub generated by Elf
pub fn solve() {
    // elf splits the printed output at the first <space> and takes writes the first half as the solution
    let mut time = Instant::now();
    let (fragmented, clustered) = parse_input(&utils::file_to_string("2024", "09"));
    let p1 = part_one(fragmented);
    println!("{:?} in {:?} for Part 1", p1, time.elapsed());
    time = Instant::now();
    let p2 = part_two(clustered);
    println!("{:?} in {:?} for Part 2", p2, time.elapsed());
}

fn part_one(files: Vec<(usize, i32)>) -> usize {
    organize(files)
}
fn part_two(files: Vec<(usize, i32)>) -> usize {
    organize(files)
}