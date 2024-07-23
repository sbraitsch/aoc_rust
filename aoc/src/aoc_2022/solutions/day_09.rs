use std::{time::Instant, collections::HashSet};
use crate::utils::file_to_lines;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn should_move(self: &Knot, trailing: &Knot) -> bool {
        self.x.abs_diff(trailing.x) > 1 || self.y.abs_diff(trailing.y) > 1
    }

    fn adjust_tail(self: &Knot, knots: &mut Vec<Knot>) {
        let mut head = self.clone();
        for tail in knots.iter_mut() {
            if head.should_move(tail) {
                tail.x += (head.x - tail.x).clamp(-1, 1);
                tail.y += (head.y - tail.y).clamp(-1, 1);
                head = tail.clone();
            } else { break; }
        }
    }
}

pub fn solve() {
    let now = Instant::now();
    let input = file_to_lines("2022", "09");
    println!("Day 9 | Part 1: {:?}\nDay 9 | Part 2: {:?}", solution_1(&input), solution_2(&input));
    println!("Took: {:?}", now.elapsed());
}

fn solution_1(input: &Vec<String>) -> usize {
    let head = Knot { x: 0, y: 0 };
    let tail = vec![Knot { x: 0, y: 0 }];
    move_head(input, head, tail)
}

fn solution_2(input: &Vec<String>) -> usize {
    let head = Knot { x: 0, y: 0 };
    let tail = vec![Knot { x: 0, y: 0 }; 9];
    move_head(input, head, tail)
}

fn move_head(input: &Vec<String>, mut head: Knot, mut tail: Vec<Knot>) -> usize {
    let mut visited: HashSet<Knot> = HashSet::new();

    for mv in input {
        let distance = &mv[2..].parse::<i32>().unwrap();
        for _ in 0..*distance {
            match mv.chars().nth(0).unwrap() {
                'L' => { head.x -= 1; },
                'R' => { head.x += 1; },
                'U' => { head.y -= 1; },
                'D' => { head.y += 1; },
                _ => panic!("Invalid Input")
            }
            head.adjust_tail(&mut tail);
            visited.insert(tail.last().unwrap().clone());
        }
    }
    visited.len()
}