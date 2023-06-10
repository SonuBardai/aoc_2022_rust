#[derive(Debug)]
pub enum Commands {
    List,
    ChangeDir(String),
    ChangeDirUp,
}

#[derive(Debug)]
pub struct PathInfo {
    path: String,
    size: u32,
}

impl PathInfo {
    pub fn new() -> Self {
        PathInfo {
            path: "".to_string(),
            size: 0,
        }
    }

    pub fn from(path: &str, size: u32) -> Self {
        let mut path_info = Self::new();
        path_info.path = path.to_string();
        path_info.size = size;
        path_info
    }
}

#[derive(Debug)]
pub enum FileSystem {
    Directory(PathInfo),
    File(PathInfo),
}

impl FileSystem {
    pub fn new_directory(path: &str) -> Self {
        FileSystem::Directory(PathInfo::from(path, 0))
    }

    pub fn new_file(path: &str, size: u32) -> Self {
        FileSystem::File(PathInfo::from(path, size))
    }
}
