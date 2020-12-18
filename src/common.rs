use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryInfo {
    path: String,
    hash: String,
    children: Vec<DirectoryInfo>,
    files: Vec<FileInfo>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    path: String,
    hash: String
}

impl DirectoryInfo {
    pub fn create(path: String, hash: String, children: Vec<DirectoryInfo>, files:Vec<FileInfo>) -> DirectoryInfo {
        DirectoryInfo {
            path,
            hash,
            children,
            files
        }
    }
}

impl FileInfo {
    pub fn create(path: String, hash: String) -> FileInfo {
        FileInfo {
            path,
            hash
        }
    }
}