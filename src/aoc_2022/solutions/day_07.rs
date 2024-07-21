use std::{collections::HashMap, time::Instant};
use crate::utils::file_to_lines;

pub fn solve() {
    let now = Instant::now();
    let commands = file_to_lines("2022", "07");

    let dir_structure = parse_file_structure(commands);

    println!("Day 7 | Part 1: {:?}\nDay 7 | Part 2: {:?}", solution_1(&dir_structure), solution_2(&dir_structure));
    println!("Took: {:?}", now.elapsed());
}

fn solution_1(directories: &HashMap<Vec<String>, usize>) -> usize {
    directories.iter().filter(|(_, s)| s <= &&100000).map(|(_,s)| s).sum()
}

fn solution_2(directories: &HashMap<Vec<String>, usize>) -> usize {
    let unused_space = 70000000 - directories.get(&vec!["/".to_string()]).unwrap();
    let space_to_free = 30000000 - unused_space;
    directories.iter().filter(|(_, s)| s >= &&space_to_free).map(|(_,s)| *s).into_iter().min().unwrap()
}

fn parse_file_structure(commands: Vec<String>) -> HashMap<Vec<String>, usize> {
    let mut directory_sizes: HashMap<Vec<String>, usize> = HashMap::new();
    let mut current_dir: Vec<String> = Vec::new();
    let mut current_dir_size = 0;

    for command in commands {

        let split = command.split(" ").collect::<Vec<&str>>();
        if split[1] == "cd" {
            update_past_dirs(&current_dir, &mut directory_sizes, current_dir_size);
            match split[2] {
                ".." => {                       
                    current_dir.pop();
                }
                folder => {
                    current_dir.push(folder.to_string());
                }
            }
            current_dir_size = 0;
        } else if let Ok(size) = split[0].parse::<usize>() {
            current_dir_size += size;
        }
    }

    update_past_dirs(&current_dir, &mut directory_sizes, current_dir_size);

    directory_sizes
}

fn update_past_dirs(current_dir: &Vec<String>, directory_sizes: &mut HashMap<Vec<String>, usize>, current_dir_size: usize) {
    for dir in 0..current_dir.len() {
        let partial = &current_dir[0..=dir];
        directory_sizes.entry(Vec::from(partial)).and_modify(|s| *s += current_dir_size).or_insert(current_dir_size);
    }
}