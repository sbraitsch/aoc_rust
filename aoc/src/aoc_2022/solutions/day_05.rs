use crate::utils::file_to_lines;

pub fn solve() {
    let input = file_to_lines("2022", "05");
    let (crates, moves_raw) = input.split_at(10);

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..9 {
        stacks.push(Vec::new());
    }

    let moves = moves_raw.iter().map(|m| {
        let instructions: Vec<&str> = m.split(" ").collect();
        let num = instructions[1].parse::<usize>().unwrap();
        let from = instructions[3].parse::<usize>().unwrap() - 1;
        let to = instructions[5].parse::<usize>().unwrap() - 1;
        (num, from, to)
    }).collect();

    for c in crates[0..8].iter().rev() {
        for i in 0..9 {
            let cr = &c[(i*4)..(i*4) + 3].trim();
            if !cr.is_empty() {
                stacks[i].push(cr.chars().nth(1).unwrap())
            }
        }
    }

    let sol_1 = solution_1(&mut stacks.clone(), &moves);
    let sol_2 = solution_2(&mut stacks.clone(), &moves);

    println!("Day 5 | Part 1: {:?}\nDay 5 | Part 2: {:?}", sol_1, sol_2)
}

fn solution_1(stacks: &mut Vec<Vec<char>>, moves: &Vec<(usize, usize, usize)>) -> String {
    for (num, from, to) in moves {
        for _ in 0..*num {
            let ctm = stacks[*from].pop().unwrap();
            stacks[*to].push(ctm);
        }
    }

    concat_top(stacks)
}

fn solution_2(stacks: &mut Vec<Vec<char>>, moves: &Vec<(usize, usize, usize)>) -> String {
    for (num, from, to) in moves {
        let sz = stacks[*from].len();
        let mut substack: Vec<char> = stacks[*from].drain(sz - num .. sz).collect();
        stacks[*to].append(&mut substack);
    }

    concat_top(stacks)
}   

fn concat_top(stacks: &Vec<Vec<char>>) -> String{
    let mut result = String::new();
    for s in stacks {
        result.push(*s.last().unwrap());
    }
    result
}