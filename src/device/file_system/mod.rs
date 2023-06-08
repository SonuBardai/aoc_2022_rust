use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Directory {
    dir_name: String,
    files_size: u32,
    sub_dir: Vec<String>,
    total_size: u32,
}

impl Directory {
    pub fn dir_name(command: &str) -> String {
        let pwd = command.split(' ').collect::<Vec<&str>>();
        pwd[pwd.len() - 1].to_string()
    }

    pub fn file_size(command: &str) -> u32 {
        let file_size = command.split(' ').collect::<Vec<&str>>();
        file_size[0].parse().unwrap()
    }

    pub fn from(command: &str) -> Self {
        Directory {
            dir_name: Self::dir_name(command),
            files_size: 0,
            sub_dir: vec![],
            total_size: 0,
        }
    }

    pub fn calculate_total_sizes(directories: &Vec<Directory>) -> HashMap<&String, u32> {
        let mut directory_sizes = std::collections::HashMap::new();
        directories.iter().for_each(|dir| {
            let sub_dirs: Vec<&Directory> = directories
                .iter()
                .rev()
                .filter(|remaining_dir| dir.sub_dir.contains(&remaining_dir.dir_name))
                .collect();
            let total_size: u32 = sub_dirs
                .iter()
                .map(|sub_dir| sub_dir.total_size.clone())
                .sum();
            directory_sizes.insert(&dir.dir_name, total_size);
        });
        directory_sizes
    }
}

// DOES NOT WORK
pub fn parse_commands(raw_input: &str) {
    let mut directories: Vec<Directory> = Vec::new();
    raw_input.split('\n').for_each(|command| {
        if command.contains("$ ls") || command.is_empty() || command.contains("$ cd ..") {
            return;
        } else if command.contains("$ cd ") {
            directories.push(Directory::from(command));
        } else {
            if command.contains("dir ") {
                directories
                    .last_mut()
                    .unwrap()
                    .sub_dir
                    .push(Directory::dir_name(command));
            } else {
                directories.last_mut().unwrap().files_size += Directory::file_size(command);
            }
        }
        let total_sizes = Directory::calculate_total_sizes(&directories);
    });
}
