use crate::device::file_system::parse_commands;
use crate::utils::read_file;

pub fn day7a_solution(input_file_name: &str) {
    let input_file_contents = read_file(input_file_name);
    parse_commands(&input_file_contents);
}

pub fn day7b_solution(input_file_name: &str) {
    let _input_file_contents = read_file(input_file_name);
}
