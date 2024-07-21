use crate::utils::file_to_lines;

pub fn solve() {
    let tuples: Vec<((usize, usize), (usize, usize))> = file_to_lines("2022", "04").iter()
        .map(|s| s.split_once(','))
        .map(|r| (r.unwrap().0.split_once('-').unwrap(), r.unwrap().1.split_once('-').unwrap()))
        .map(|(a, b)| ((a.0.parse::<usize>().unwrap(), a.1.parse::<usize>().unwrap()), (b.0.parse::<usize>().unwrap(), b.1.parse::<usize>().unwrap())))
        .collect();

    let sol_1 = solution_1(tuples.as_ref());
    let sol_2 = solution_2(tuples.as_ref());

    println!("Day 4 | Part 1: {:?}\nDay 4 | Part 2: {:?}", sol_1, sol_2);

}


fn solution_1(pairs: &Vec<((usize, usize), (usize, usize))>) -> usize {
    pairs.iter().fold(0, |acc, x| acc + full_overlap(&x) as usize )
}

fn solution_2(pairs: &Vec<((usize, usize), (usize, usize))>) -> usize{
    pairs.iter().fold(0, |acc, x| acc + partial_overlap(&x) as usize )
}

fn full_overlap((a, b): &((usize, usize), (usize, usize))) -> bool {
    if a.0 >= b.0 && a.1 <= b.1 {
        true
    } else if b.0 >= a.0 && b.1 <= a.1 {
        true
    } else {
        false
    }
}

fn partial_overlap((a, b): &((usize, usize), (usize, usize))) -> bool {
    if a.0 <= b.0 && a.1 >= b.0 {
        true
    } else if b.0 <= a.0 && b.1 >= a.0 {
        true
    } else {
        false
    }
}
