use std::{env, error::Error, fs, path::Path};

use reqwest::header::{HeaderMap, HeaderValue, COOKIE};

const TEMPLATE: &str = include_str!("aoc_template.txt");

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <year> <filename> <session>", args[0]);
        std::process::exit(1);
    }

    let year = &args[1];
    let filename = &args[2];
    let session = &args[3];

    if let Err(e) = write_template_to_file(year, filename, session) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn write_template_to_file(year: &str, day: &str, session: &str) -> Result<(), Box<dyn Error>> {
    let content = TEMPLATE.replace("{{year}}", year).replace("{{day}}", day);
    let in_dir = format!("aoc/src/aoc_{year}/inputs");
    let sol_dir = format!("aoc/src/aoc_{year}/solutions");
    let dir_path = Path::new(&sol_dir);
    let filename = format!("day_{day}.rs");
    let file_path = dir_path.join(&filename);
    if !file_path.exists() {
        fs::write(file_path, content)?;
        update_mod(dir_path, day)?;
        download_input(Path::new(&in_dir), year, day, session)?;
        println!("Created new file {filename} for AoC {year}");
    } else {
        println!("File already exists.")
    }

    Ok(())
}

fn update_mod(dir: &Path, day: &str) -> Result<(), Box<dyn Error>> {
    let mod_path = dir.join("mod.rs");

    let mut content = if mod_path.exists() {
        fs::read_to_string(&mod_path)?
    } else {
        String::new()
    };

    let add_to_mod = format!("pub mod day_{};", day);
    if !content.contains(&add_to_mod) {
        content.push_str(&add_to_mod);
        fs::write(mod_path, content)?;
    }

    Ok(())
}

fn download_input(dir: &Path, year: &str, day: &str, session: &str) -> Result<(), Box<dyn Error>> {
    let d = day.parse::<i32>().unwrap();
    let url = format!("https://adventofcode.com/{year}/day/{d}/input");
    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()?;
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(session)?);
    let response = client.get(url).headers(headers).send()?;
    let filename = format!("input_{day}.txt");
    let file_path = dir.join(&filename);
    if !file_path.exists() {
        fs::write(file_path, response.text()?)?;
        println!("Retrieved input file for AoC {year}, Day {day}.");
    } else {
        println!("File already exists.")
    }
    Ok(())
}
