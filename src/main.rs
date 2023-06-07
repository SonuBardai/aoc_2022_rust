use clap::Parser;
use rust_aoc_2022::*;

#[derive(Parser, Debug)]
struct CliArgs {
    #[arg(short, long)]
    day: Option<usize>,

    #[arg(short, long)]
    input: Option<usize>,
}

fn main() {
    let args = CliArgs::parse();
    let day = if let Some(day) = args.day {
        day as i32
    } else {
        println!("Day not mentioned");
        0
    };

    let input = if let Some(input) = args.input {
        input as i32
    } else {
        2
    };

    match day {
        1 => {
            let input_file_name = format!("src/days/day1/inputs/input_{input}.txt");
            days::day1a_solution(&input_file_name);
            days::day1b_solution(&input_file_name);
        }
        2 => {
            let input_file_name = format!("src/days/day2/inputs/input_{input}.txt");
            days::day2a_solution(&input_file_name);
            days::day2b_solution(&input_file_name);
        }
        3 => {
            let input_file_name = format!("src/days/day3/inputs/input_{input}.txt");
            days::day3a_solution(&input_file_name);
            days::day3b_solution(&input_file_name);
        }
        4 => {
            let input_file_name = format!("src/days/day4/inputs/input_{input}.txt");
            days::day4a_solution(&input_file_name);
            days::day4b_solution(&input_file_name);
        }
        5 => {
            let input_file_content = match input {
                1 => {
                    include_str!("days/day5/inputs/input_1.txt")
                }
                2 => {
                    include_str!("days/day5/inputs/input_2.txt")
                }
                _other => {
                    panic!("Input file not found");
                }
            };
            days::day5a_solution(input_file_content);
            days::day5b_solution(input_file_content);
        }
        6 => {
            let input_file_content = include_str!("days/day6/inputs/input_1.txt");
            days::day6a_solution(input_file_content);
            days::day6b_solution(input_file_content);
        }
        7 => {
            let input_file_name = format!("src/days/day4/inputs/input_{input}.txt");
            days::day7a_solution(&input_file_name);
            days::day7b_solution(&input_file_name);
        }
        other => {
            eprintln!("Day {other} not yet covered");
            // https://doc.rust-lang.org/std/macro.eprintln.html
        }
    };
}
