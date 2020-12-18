use serde::{Deserialize, Serialize};
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryInfo {
    path: String,
    pub(crate) hash: String,
    children: Vec<DirectoryInfo>,
    pub(crate) files: Vec<FileInfo>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    path: String,
    pub(crate) hash: String
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
    
    pub fn files_to_dict(&self) -> HashMap<String, String> {
        let mut dict: HashMap<String, String> = HashMap::with_capacity(self.files.len());
        
        for fi in &self.files {
            dict.insert(fi.path.clone(), fi.hash.clone());
        };
        
        dict
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