use serde::{Deserialize, Serialize};
use crypto::sha2::Sha256;
use crypto::digest::Digest;

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

pub fn create_hash(mut data: Vec<u8>) -> String {
    let mut hasher = Sha256::new();
    hasher.input(data.as_slice());
    hasher.result_str()
}