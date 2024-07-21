use crate::utils::file_to_lines;

pub fn solve() {
    let sol_1 = solution_1();
    let sol_2 = solution_2();
    println!("Day 3 | Part 1: {:?}\nDay 3 | Part 2: {:?}", sol_1, sol_2);

}

fn solution_1() -> usize{
    let mut priority_sum = 0;
    for line in file_to_lines("2022", "03") {
        let (a, b) = line.split_at(line.len() / 2);
        for item in a.chars() {
            if b.contains(item) {
                priority_sum += prio_value(item);
                break;
            }
        }
    }
    priority_sum
}

fn solution_2() -> usize{
    let mut priority_sum = 0;
    for chunk in file_to_lines("2022", "03").chunks(3) {
        let (a,b,c) = (&chunk[0],&chunk[1],&chunk[2]);
        for item in a.chars() {
            if b.contains(item) && c.contains(item) {
                priority_sum += prio_value(item);
                break;
            }
        }
    }
    priority_sum
}

fn prio_value(item : char) -> usize {
    if item.is_uppercase() {
        item as usize - 38
    } else {
        item as usize - 96
    }
}