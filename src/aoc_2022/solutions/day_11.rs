use std::{time::Instant, collections::{VecDeque, BinaryHeap}, cell::RefCell};

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    op: fn(usize) -> usize,
    test_div: (usize, usize, usize),
    inspections: usize
}
pub fn solve() {
    let now = Instant::now();
    let mut monkeys: Vec<RefCell<Monkey>> = Vec::with_capacity(8);
    monkeys.push(RefCell::new(Monkey { items: VecDeque::from(vec![89,73,66,57,64,80]), op: |item| item * 3, test_div: (13, 6, 2), inspections: 0 }));
    monkeys.push(RefCell::new(Monkey { items: VecDeque::from(vec![83,78,81,55,81,59,69]), op: |item| item + 1, test_div: (3, 7, 4), inspections: 0 }));
    monkeys.push(RefCell::new(Monkey { items: VecDeque::from(vec![76,91,58,85]), op: |item| item * 13, test_div: (7, 1, 4), inspections: 0 }));
    monkeys.push(RefCell::new(Monkey { items: VecDeque::from(vec![71,72,74,76,68]), op: |item| item * item, test_div: (2, 6, 0), inspections: 0 }));
    monkeys.push(RefCell::new(Monkey { items: VecDeque::from(vec![98,85,84]), op: |item| item + 7, test_div: (19, 5, 7), inspections: 0 }));
    monkeys.push(RefCell::new(Monkey { items: VecDeque::from(vec![78]), op: |item| item + 8, test_div: (5, 3, 0), inspections: 0 }));
    monkeys.push(RefCell::new(Monkey { items: VecDeque::from(vec![86,70,60,88,88,78,74,83]), op: |item| item + 4, test_div: (11, 1, 2), inspections: 0 }));
    monkeys.push(RefCell::new(Monkey { items: VecDeque::from(vec![81,58]), op: |item| item + 5, test_div: (17, 3, 5), inspections: 0 }));

    println!("Day 11 | Part 1: {:?}\nDay 11 | Part 2: {:?}", solution_1(&mut monkeys.clone()), solution_2(&mut monkeys.clone()));
    println!("Took: {:?}", now.elapsed());
}

fn solution_1(monkeys: &mut Vec<RefCell<Monkey>>) -> usize {
    monke(monkeys, 20, 0)
}

fn solution_2(monkeys: &mut Vec<RefCell<Monkey>>) -> usize {
    monke(monkeys, 10000, monkeys.iter().map(|m| m.borrow().test_div.0).product())
}

fn monke(monkeys: &mut Vec<RefCell<Monkey>>, rounds: usize, divisor: usize) -> usize {
    for _ in 0..rounds {
        for i in 0..8 {
            let mut monkey = monkeys[i].borrow_mut();
            monkey.inspections += monkey.items.len();
            while monkey.items.len() > 0 {
                let item = monkey.items.pop_front().unwrap();
                let new_worry = if divisor != 0 { (monkey.op)(item) % divisor } else { (monkey.op)(item) / 3 };
                if new_worry % monkey.test_div.0 == 0 {
                    monkeys[monkey.test_div.1].borrow_mut().items.push_back(new_worry);
                } else {
                    monkeys[monkey.test_div.2].borrow_mut().items.push_back(new_worry);
                }
            }
        }
    }
    let mut heap = monkeys.iter().map(|m| m.borrow().inspections).collect::<BinaryHeap<usize>>();
    heap.pop().unwrap() * heap.pop().unwrap()
}