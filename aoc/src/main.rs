#![allow(unused)]

use std::env;

mod aoc_2021;
mod aoc_2022;
mod aoc_2023;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 && (&args[1] == "-s" || &args[1] == "--submit") {
        // reqwest to submit part arg[2] of currently used day
    } else if args.len() == 1 {
        aoc_2021::day_03::solve();
    } else {
        eprintln!("Usage: {} (<[-s | --submit]> <part>)", args[0]);
    }
}
