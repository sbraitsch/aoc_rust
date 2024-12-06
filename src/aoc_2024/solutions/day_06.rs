use crate::utils;
use std::{collections::HashSet, time::Instant};

fn parse_input(lines: Vec<String>) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut starting = (0, 0);
    let grid = lines
        .into_iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '^' {
                        starting = (x, y)
                    }
                    c
                })
                .collect::<Vec<char>>()
        })
        .collect();
    (grid, starting)
}

fn walk(
    cur: (usize, usize),
    dir: (isize, isize),
    grid: &Vec<Vec<char>>,
    special_block: Option<(usize, usize)>,
) -> Option<((usize, usize), (isize, isize))> {
    let new_x = (cur.0 as isize + dir.0) as usize;
    let new_y = (cur.1 as isize + dir.1) as usize;
    let new_dir = match dir {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => (0, 0),
    };
    if new_x >= grid[0].len() || new_y >= grid.len() {
        return None;
    }

    if let Some((x, y)) = special_block {
        if x == new_x && y == new_y {
            return Some((cur, new_dir));
        }
    }

    match grid[new_y][new_x] {
        '#' => Some((cur, new_dir)),
        '.' | '^' => Some(((new_x, new_y), dir)),
        _ => None,
    }
}

// Stub generated by Elf
pub fn solve() {
    // elf splits the printed output at the first <space> and takes writes the first half as the solution
    let (grid, starting) = parse_input(utils::file_to_lines("2024", "06"));
    let mut time = Instant::now();
    let p1 = part_one(&grid, starting);
    println!("{:?} in {:?} for Part 1", p1.0, time.elapsed());
    time = Instant::now();
    let p2 = part_two(&grid, starting, p1.1);
    println!("{:?} in {:?} for Part 2", p2, time.elapsed());
}

fn part_one(grid: &Vec<Vec<char>>, starting: (usize, usize)) -> (usize, HashSet<(usize, usize)>) {
    let mut path = HashSet::new();

    let mut cur_pos = starting;
    let mut cur_dir = (0, -1);
    path.insert(cur_pos);

    while let Some((pos, dir)) = walk(cur_pos, cur_dir, grid, None) {
        path.insert(pos);
        cur_pos = pos;
        cur_dir = dir;
    }
    (path.len(), path)
}

fn part_two(grid: &Vec<Vec<char>>, starting: (usize, usize), path: HashSet<(usize, usize)>) -> usize {
    let mut spots = 0;
    path.iter().for_each(|&(x, y)| {
        if grid[y][x] == '.' {
            let mut path = HashSet::new();

            let mut cur_pos = starting;
            let mut cur_dir = (0, -1);
            path.insert((cur_pos, cur_dir));

            while let Some((pos, dir)) = walk(cur_pos, cur_dir, grid, Some((x, y))) {
                let inserted = path.insert((pos, dir));
                if !inserted {
                    spots += 1;
                    break;
                };
                cur_pos = pos;
                cur_dir = dir;
            }
        };
    });
    spots
}