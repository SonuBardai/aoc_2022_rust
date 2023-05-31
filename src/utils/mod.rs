use std::fs;

pub fn read_file(input_file_name: &String) -> String {
    match fs::read_to_string(input_file_name) {
        Ok(read_lines) => read_lines,
        Err(err) => {
            println!("Failed to read file. {}", err);
            "".to_string()
        }
    }
}


pub fn get_all_values(string_input: &str) -> Vec<&str> {
    string_input.split('\n').collect()
}
