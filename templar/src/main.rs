use std::{env, error::Error, fs, io::ErrorKind, path::Path};

use reqwest::header::{HeaderMap, HeaderValue, COOKIE};

const TEMPLATE: &str = include_str!("aoc_template.txt");

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <year> <filename> <session>", args[0]);
        std::process::exit(1);
    }

    let year = &args[1];
    let day = &args[2];
    let session = &args[3];

    let base_path = format!("aoc/src/aoc_{year}");

    if let Err(e) = write_solution_template(&base_path, year, day) {
        eprintln!("Error writing solution template: {}", e);
        std::process::exit(1);
    }
    if let Err(e) = write_mod(&base_path, day) {
        eprintln!("Error updating mod.rs: {}", e);
        std::process::exit(1);
    }
    if let Err(e) = write_input(&base_path, year, day, session) {
        eprintln!("Error writing input file: {}", e);
        std::process::exit(1);
    }
}

fn write_solution_template(base_path: &str, year: &str, day: &str) -> Result<(), Box<dyn Error>> {
    let content = TEMPLATE.replace("{{year}}", year).replace("{{day}}", day);
    let filename = format!("day_{day}.rs");
    let file_path = Path::new(&base_path).join("solutions").join(&filename);
    write_file(&file_path, &content)?;
    println!("<Created new file {filename} for AoC {year}>");
    Ok(())
}

fn write_mod(base_path: &str, day: &str) -> Result<(), Box<dyn Error>> {
    let mod_path = Path::new(&base_path).join("solutions").join("mod.rs");
    let mut content = if mod_path.exists() {
        fs::read_to_string(&mod_path)?
    } else {
        String::new()
    };
    let add_to_mod = format!("pub mod day_{};\n", day);
    if !content.contains(&add_to_mod) {
        content.push_str(&add_to_mod);
        fs::write(mod_path, content)?;
        println!("<Updated mod.rs>");
    }

    Ok(())
}

fn write_input(
    base_path: &str,
    year: &str,
    day: &str,
    session: &str,
) -> Result<(), Box<dyn Error>> {
    let filename = format!("input_{day}.txt");
    let file_path = Path::new(&base_path).join("inputs").join(&filename);
    let d = day.parse::<i32>().unwrap();
    let url = format!("https://adventofcode.com/{year}/day/{d}/input");
    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()?;
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(session)?);
    let response = client.get(url).headers(headers).send()?;
    write_file(&file_path, &response.text()?)?;
    println!("<Retrieved puzzle input>");
    Ok(())
}

fn write_file(full_path: &Path, content: &str) -> Result<(), Box<dyn Error>> {
    if !full_path.exists() {
        fs::write(full_path, content)?;
    } else {
        let err = std::io::Error::new(ErrorKind::AlreadyExists, "File already exists");
        return Err(Box::new(err));
    }
    Ok(())
}
