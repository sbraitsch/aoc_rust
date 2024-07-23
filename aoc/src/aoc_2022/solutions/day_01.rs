use std::collections::BinaryHeap;
use crate::utils::file_to_lines;

pub fn solve() {
    let mut bin_heap = BinaryHeap::new();

    let mut temp_sum = 0;
        for line in file_to_lines("2022", "01") {
            if line == ""  {
                bin_heap.push(temp_sum);
                temp_sum = 0
               } else {
                temp_sum += line.parse::<usize>().unwrap();
               }
        }

    let sol_1 = solution_1(&mut bin_heap);
    let sol_2 = solution_2(&mut bin_heap);

    println!("Day 1 | Part 1: {:?}\nDay 1 | Part 2: {:?}", sol_1, sol_2);
}

pub fn solution_1(calories: &mut BinaryHeap<usize>) -> usize {
    *calories.peek().unwrap()
}

fn solution_2(calories: &mut BinaryHeap<usize>) -> usize {
    let mut top_three_sum = 0;
    for _ in 0..3 {
        if let Some(top) = calories.pop() {
            top_three_sum += top;
        }
    }
    top_three_sum
}