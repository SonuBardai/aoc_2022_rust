use crate::calories::{calculate_max_n_portions, calculate_max_portion, parse_portions};
use crate::utils::read_file;

pub fn day1a_solution(input_file_name: &str) {
    print!("Day 1a: ");
    let read_lines = read_file(input_file_name);
    let parsed_portions = parse_portions(&read_lines);
    let max_calories = calculate_max_portion(parsed_portions);
    println!("Max calories: {max_calories}");
}

pub fn day1b_solution(input_file_name: &str) {
    print!("Day 1b: ");
    let read_lines = read_file(input_file_name);
    let parsed_portions = parse_portions(&read_lines);
    let max_portions = calculate_max_n_portions(parsed_portions, 3);
    print!("3 Max calories: {max_portions:?}. ");
    let max_portions_sum: isize = max_portions.iter().sum();
    println!("Sum: {max_portions_sum}");
}
