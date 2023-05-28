use clap::Parser;
use rust_aoc_2022::*;

#[derive(Parser, Debug)]
struct CliArgs {
    #[arg(short, long)]
    day: Option<usize>,
}

fn main() {
    let args = CliArgs::parse();
    let day = if let Some(day) = args.day {
        day as i32
    } else {
        println!("Day not mentioned");
        0
    };
    match day {
        1 => {
            let input_file_name = "src/days/day1/input_2.txt".to_string();
            days::day1a_solution(&input_file_name);
            days::day1b_solution(&input_file_name);
        }
        2 => {
            days::day2a_solution();
        }
        other => {
            eprintln!("Day {other} not yet covered");
            // https://doc.rust-lang.org/std/macro.eprintln.html
        }
    };
}
