use std::{time::Instant, collections::{BinaryHeap, HashSet}, cmp::Ordering};

use crate::utils::file_to_lines;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Node {
    idx: usize,
    elevation: u8,
    cost: usize
}

// reverse natural order to create minheap
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve() {
    let now = Instant::now();
    let mut map: Vec<u8> = Vec::new();
    let mut start = 0;
    let mut target = 0;
    let input = file_to_lines("2022", "12");
    let dim_x = input.first().unwrap().len();
    let dim_y = input.len();

    for (y, row) in input.iter().enumerate() {
        for (x, height) in row.chars().enumerate() {
            match height {
                'S' => { 
                    start = x + y * dim_x;
                    map.push(1);
                },
                'E' => { 
                    target = x + y * dim_x;
                    map.push(26);
                },
                c => {
                    let height = c as u8 - 96;
                    map.push(height);
                }
            }
        }
    }

    println!("Day 12 | Part 1: {:?}\nDay 12 | Part 2: {:?}", solution_1(&map, start, target, dim_x, dim_y), solution_2(&map, target, dim_x, dim_y));
    println!("Benchmark: {:?}", now.elapsed());
}

fn solution_1(map: &Vec<u8>, start: usize, target: usize, dim_x: usize, dim_y: usize) -> usize {
    shortest_path(map, start, target, dim_x, dim_y, false)
}

fn solution_2(map: &Vec<u8>, target: usize, dim_x: usize, dim_y: usize) -> usize {
    shortest_path(map, target, 0, dim_x, dim_y, true)
}

fn shortest_path(maze: &Vec<u8>, start: usize, target: usize, dim_x: usize, dim_y: usize, p2: bool)-> usize {
    let mut open_list = BinaryHeap::new();
    let mut visited: HashSet<usize> = HashSet::new();

    open_list.push(Node { idx: start, elevation: maze[start], cost: 0});
    visited.insert(start);

    while let Some(Node {idx, elevation, cost}) = open_list.pop() {
        if (p2 && elevation == 1) || (!p2 && idx == target) {
            return cost;
        } else {
            for adj in get_adjacent(idx, dim_x, dim_y) {
                let adj_elevation = maze[adj];
                if (adj_elevation as i8 - elevation as i8 > 1 && !p2) || (elevation as i8 - adj_elevation as i8 > 1 && p2) { 
                    continue; 
                }
                if visited.insert(adj) {
                    open_list.push(Node { idx: adj, elevation: adj_elevation, cost: cost + 1 });
                }
            }
        }
    }
    0
}

fn get_adjacent<'a>(idx: usize, dim_x: usize, dim_y: usize) -> Vec<usize> {
    let x = idx % dim_x;
    let y = idx / dim_x;

    let mut adj = Vec::new();
    if x > 0 { adj.push(idx - 1); }
    if x < dim_x - 1 { adj.push(idx + 1); }
    if y > 0 { adj.push(idx - dim_x); }
    if y < dim_y - 1 { adj.push(idx + dim_x); }

    adj
}

fn get_idx(x: usize, y: usize, dim_x: usize) -> usize {
    (y * dim_x) + x
}