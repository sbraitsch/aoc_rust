use std::collections::{HashMap, HashSet};
use crate::utils;
use std::time::Instant;
use itertools::Itertools;
use rayon::prelude::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub fn solve() {
    let lines = utils::file_to_lines("2023", "16");
    let dim = lines.len();
    let mut mirror_map = HashMap::new();
    lines.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            mirror_map.insert((x, y), c);
        })
    });

    let mut time = Instant::now();
    println!("Part 1: {:?} in {:?}", part_one(&mirror_map, dim, ((0,0), Direction::RIGHT)), time.elapsed());
    time = Instant::now();
    println!("Part 2: {:?} in {:?}", part_two(&mirror_map, dim), time.elapsed());
}

fn part_one(map: &HashMap<(usize, usize), char>, dim: usize, start: ((usize, usize), Direction)) -> usize {
    visit_tiles(map, dim, start)
}

fn part_two(map: &HashMap<(usize, usize), char>, dim: usize) -> usize {
    let mut starting_points = Vec::new();
    (0..dim).for_each(|i| {
        starting_points.push(((0, i), Direction::RIGHT));
        starting_points.push(((dim-1, i), Direction::LEFT));
        starting_points.push(((i, 0), Direction::DOWN));
        starting_points.push(((i, dim-1), Direction::UP));
    });
    starting_points.into_par_iter().map(|p| {
        part_one(map, dim, p)
    }).max().unwrap()
}


fn visit_tiles(map: &HashMap<(usize, usize), char>, dim: usize, start: ((usize, usize), Direction)) -> usize {
    let mut found: Vec<((usize, usize), Direction)> = Vec::new();
    let mut visited: HashSet<((usize, usize), Direction)> = HashSet::new();
    found.push(start);

    while !found.is_empty() {
        let ((x, y), dir) = found.pop().unwrap();
        visited.insert(((x, y), dir.clone()));
        get_valid_neighbours((x, y), &dir, map, dim).iter()
            .for_each(|e|
                if !visited.contains(&(e)) {
                    found.push(e.clone())
                }
            );
    }
    visited.iter().unique_by(|e| e.0).count()
}

fn get_valid_neighbours((x, y): (usize, usize), dir_in: &Direction, map: &HashMap<(usize, usize), char>, dim: usize) -> Vec<((usize, usize), Direction)> {
    let mut neighbours = Vec::new();
    let cur = map[&(x, y)];
    match dir_in {
        Direction::UP => {
            match cur {
                '.' | '|' => if y > 0 { neighbours.push(((x, y - 1), dir_in.clone())) },
                '\\' => if x > 0 { neighbours.push(((x - 1, y), Direction::LEFT)) },
                '/' => if x + 1 < dim { neighbours.push(((x + 1, y), Direction::RIGHT)) },
                _ => {
                    if x > 0 { neighbours.push(((x - 1, y), Direction::LEFT)) }
                    if x + 1 < dim { neighbours.push(((x + 1, y), Direction::RIGHT)) }
                }
            };
        }
        Direction::DOWN => {
            match cur {
                '.' | '|' => if y + 1 < dim { neighbours.push(((x, y + 1), dir_in.clone())) },
                '\\' => if x + 1 < dim { neighbours.push(((x + 1, y), Direction::RIGHT)) },
                '/' => if x > 0 { neighbours.push(((x - 1, y), Direction::LEFT)) },
                _ => {
                    if x > 0 { neighbours.push(((x - 1, y), Direction::LEFT)) }
                    if x + 1 < dim { neighbours.push(((x + 1, y), Direction::RIGHT)) }
                }
            };
        }
        Direction::LEFT => {
            match cur {
                '.' | '-' => if x > 0 { neighbours.push(((x - 1, y), dir_in.clone())) },
                '\\' => if y > 0 { neighbours.push(((x, y - 1), Direction::UP)) },
                '/' => if y + 1 < dim { neighbours.push(((x, y + 1), Direction::DOWN)) },
                _ => {
                    if y > 0 { neighbours.push(((x, y - 1), Direction::UP)) }
                    if y + 1 < dim { neighbours.push(((x, y + 1), Direction::DOWN)) }
                }
            };
        }
        Direction::RIGHT => {
            match cur {
                '.' | '-' => if x + 1 < dim { neighbours.push(((x + 1, y), dir_in.clone())) },
                '\\' => if y + 1 < dim { neighbours.push(((x, y + 1), Direction::DOWN)) },
                '/' => if y > 0 { neighbours.push(((x, y - 1), Direction::UP)) },
                _ => {
                    if y > 0 { neighbours.push(((x, y - 1), Direction::UP)) }
                    if  y + 1 < dim { neighbours.push(((x, y + 1), Direction::DOWN)) }
                }
            }
        }
    }
    neighbours
}
