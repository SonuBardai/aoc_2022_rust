use crate::device::file_system::delete_directories;
use crate::utils::read_file;

pub fn day7a_solution(input_file_name: &str) {
    let input_file_contents = read_file(input_file_name);
    delete_directories(&input_file_contents, 100_000);
}

pub fn day7b_solution(input_file_name: &str) {
    let _input_file_contents = read_file(input_file_name);
}
