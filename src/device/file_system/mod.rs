pub mod file;

use file::{Commands, FileSystem};

pub struct StackItem {
    dir_name: String,
    total_size: u32,
}
pub struct Stack(Vec<StackItem>);

#[derive(Debug)]
pub enum Terminal {
    Input(Commands),
    Output(FileSystem),
}

impl Terminal {
    pub fn tokenize(terminal_item: &str) -> Terminal {
        if terminal_item.contains("$ cd ") {
            if terminal_item.contains("..") {
                Terminal::Input(Commands::ChangeDirUp)
            } else {
                Terminal::Input(Commands::ChangeDir(
                    terminal_item
                        .strip_prefix("$ cd ")
                        .unwrap_or_else(|| panic!("Failed to get dir name from cd command"))
                        .to_string(),
                ))
            }
        } else if terminal_item.contains("$ ls") {
            Terminal::Input(Commands::List)
        } else if terminal_item.contains("dir ") {
            let path = terminal_item
                .strip_prefix("dir ")
                .expect("Failed to get dir name");
            Terminal::Output(FileSystem::new_directory(path))
        } else {
            let (size, path) = terminal_item
                .split_once(' ')
                .expect(&format!("Failed to get file info from {terminal_item}"));
            Terminal::Output(FileSystem::new_file(
                path,
                size.parse::<u32>().expect(&format!(
                    "Failed to parse file size for command {terminal_item}"
                )),
            ))
        }
    }

    pub fn parse_terminal(terminal_data: &str) -> Vec<Terminal> {
        terminal_data
            .split('\n')
            .filter(|line| !line.is_empty())
            .into_iter()
            .map(|command| Self::tokenize(command))
            .collect::<Vec<Terminal>>()
    }
}

pub fn delete_directories(input_content: &str, _size_threshold: u32) {
    let stack = Stack(vec![]);
    let commands = Terminal::parse_terminal(input_content);
    commands.iter().map(|command| match command {
        Terminal::Input(command) => {
            todo!()
        }
        Terminal::Output(command) => match command {
            FileSystem::Directory(dir) => {
                todo!()
            }
            FileSystem::File(file) => {
                todo!()
            }
        },
    });
}
