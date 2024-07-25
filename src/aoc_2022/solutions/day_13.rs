use std::{time::Instant, cmp::Ordering};

use crate::utils::file_to_lines;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    Integer(i32),
    List(Vec<Packet>),
}

fn parse(s: &str) -> Packet {
    // recursively parse line by keeping reference to nesting level
    // removing comma on the fly
    if &s[0..1] == "[" {
        let mut nesting: u8 = 0;
        Packet::List(
            // ignore enclosing []
            s[1..s.len() - 1]
                .split(|c| {
                    if c == '[' {
                        nesting += 1
                    } else if c == ']' {
                        nesting -= 1
                    }
                    c == ',' && nesting == 0
                })
                .filter_map(|s| (!s.is_empty()).then(|| parse(s)))
                .collect(),
        )
    } else {
        Packet::Integer(s.parse().unwrap())
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        // comparison definitions for all types of packet enum combinations
        match (self, other) {
            // simple number comparison
            (Packet::Integer(a), Packet::Integer(b)) => a.cmp(b),
            // put integer in list to compare to other list
            (Packet::Integer(_), _) => Packet::List(vec![self.clone()]).cmp(other),
            // put other integer in list to compare to own list
            (_, Packet::Integer(_)) => self.cmp(&Packet::List(vec![other.clone()])),
            // compare two lists
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


pub fn solve() {
    let now = Instant::now();
    let mut packets: Vec<Packet> = file_to_lines("2022", "13")
        .iter()
        .filter_map(|l| (!l.is_empty()).then(|| parse(l.as_str())))
        .collect();

    println!("Day 12 | Part 1: {:?}\nDay 12 | Part 2: {:?}", solution_1(&packets), solution_2(&mut packets));
    println!("Benchmark: {:?}", now.elapsed());
}

fn solution_1(packets: &Vec<Packet>) -> usize {
    packets
        .chunks(2)
        .enumerate()
        .fold(0, |acc, (idx, chunk)| 
            if chunk[0].cmp(&chunk[1]) == Ordering::Less { acc + idx + 1 } else { acc }
        )
}

fn solution_2(packets: &mut Vec<Packet>) -> usize {
    let div1 = parse("[[2]]");
    let div2 = parse("[[6]]");

    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort();

    let div1_idx = packets.binary_search(&div1).unwrap();
    let div2_idx = packets.binary_search(&div2).unwrap();
    (div1_idx + 1) * (div2_idx + 1)
}