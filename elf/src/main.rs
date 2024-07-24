mod template;
mod bootstrap;
mod submit;

use clap::{Parser, Subcommand};
use jiff::Zoned;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {

    Bootstrap {
        #[arg(short, long, default_value_t = false, help = "Flag if a new cargo project should be created.")]
        full: bool,
        #[arg(short, long, value_name = "YEAR", default_value = current_year(), help = "Specify the year")]
        year: String,
    },
    Submit {
        #[arg(short, long, value_name = "YEAR", default_value = current_year(), help = "Specify the year")]
        year: String,
        #[arg(short, long, value_name = "DAY", default_value_t = current_day(), help = "Specify the day")]
        day: i8,
        #[arg(short, long, value_name = "PART", default_value_t = 1, help = "Specify the solution part")]
        part: u8,
    },
    New {
        #[arg(short, long, value_name = "YEAR", default_value = current_year(), help = "Specify the year")]
        year: String,
        #[arg(short, long, value_name = "DAY", default_value_t = current_day(), help = "Specify the day")]
        day: i8,
        #[arg(short, long, required = false, value_name = "FILE", help = "Specify the template file to use")]
        template: Option<String>,
        #[arg(short, long, required = false, value_name = "ROOT_PATH", help = "Specify the path to the project root")]
        root: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Bootstrap { full, year }) => {
            if *full {
                bootstrap::bootstrap_full(year);
            } else {
                bootstrap::bootstrap_year_mod(year);
            }
        }
        Some(Commands::Submit { year, day, part }) => {
            submit::submit(year, *day, *part)
        }
        Some(Commands::New { year, day, template , root}) => {
            let day_fmt = if *day < 10 {
                format!("0{day}")
            } else {
                day.to_string()
            };
            println!("New command with year: {year}, day: {day}, template: {template:?}, root_path: {root:?}");
            template::new_day(year, &day_fmt, root);
        }
        None => unreachable!("A subcommand is required"), // clap ensures this
    }
}

fn current_year() -> String {
    Zoned::now().year().to_string()
}
fn current_day() -> i8 {
    Zoned::now().day()
}
