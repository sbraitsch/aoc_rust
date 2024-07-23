use std::time::Instant;
use crate::utils::file_to_lines;

type Forest = Vec<Vec<usize>>;

pub fn solve() {
    let now = Instant::now();
    let mut forest: Forest = Vec::with_capacity(99);

    for row in file_to_lines("2022", "08").iter() {
        forest.push(row.chars().map(|c| c.to_digit(10).unwrap() as usize).collect());
    }

    let (part_1, part_2) = map_vis(&forest);

    println!("Day 8 | Part 1: {:?}\nDay 8 | Part 2: {:?}", part_1, part_2);
    println!("Took: {:?}", now.elapsed());
}

fn map_vis(forest: &Forest) -> (usize, usize) {
    let mut visibles = 0;
    let mut most_scenic = 0;

    for y in 0..forest.len() {
        for x in 0..forest.len() {
            if !(x == 0 || y == 0 || x == 98 || y == 98) {
                let cur_height = forest[y][x];
                let mut blocked = 0;
                let mut view_distance = 0;
                let mut scenic_score = 1;
                
                //west
                for (vd, i) in (0..x).rev().enumerate() {
                    view_distance = vd+1;
                    if forest[y][i] >= cur_height {
                        blocked += 1;
                        break;
                    }
                }
                scenic_score *= view_distance;
    
                //east
                for (vd, i) in (x+1..99).enumerate() {
                    view_distance = vd+1;
                    if forest[y][i] >= cur_height {
                        blocked += 1;
                        break;
                    }
                }
                scenic_score *= view_distance;
    
                //north
                for (vd, i) in (0..y).rev().enumerate() {
                    view_distance = vd+1;
                    if forest[i][x] >= cur_height {
                        blocked += 1;
                        break;
                    }
                }
                scenic_score *= view_distance;
    
                //south
                for (vd, i) in (y+1..99).enumerate() {
                    view_distance = vd+1;
                    if forest[i][x] >= cur_height {
                        blocked += 1;
                        break;
                    }
                }
                scenic_score *= view_distance;
                
                if blocked < 4 { visibles += 1; }
                if scenic_score > most_scenic { most_scenic = scenic_score; }

            } else { visibles += 1 }
        }
    }
    (visibles, most_scenic)
}