use serde::{Deserialize, Serialize};
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use std::collections::HashMap;
use std::path::Path;
use std::ffi::OsStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryInfo {
    pub(crate) path: String,
    pub(crate) hash: String,
    pub(crate) children: Vec<DirectoryInfo>,
    pub(crate) files: Vec<FileInfo>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    path: String,
    pub(crate) hash: String
}

#[derive(Debug)]
pub enum InputType<'a> {
    Directory(&'a Path),
    Map(&'a Path)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exception {
    message: String,
    inner: String, 
    hint: String,
    debug: String
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Diff {
    Add(String),
    Remove(String),
    Changed(String)
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
    
   pub fn flatten(&self) -> HashMap<String, String> {
        let mut dict  = self.files_to_dict();
        
      
       
        for di in &self.children {
            dict.extend(di.flatten())
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

impl InputType<'_> {
    pub fn create(filename: &str) -> Result<InputType, &'static str> {
                
        let path =  Path::new(filename);
        
        match path.exists() {
            true => {
                let ext = Path::new(filename)
                    .extension()
                    .and_then(OsStr::to_str);

                match ext {
                    Some(s) if s == "json" || s == "dmap" => Ok(InputType::Map(path)),
                    Some(_) => Err("Path is not a `.dmap` or `.json` file, or directory."),
                    None => Ok(InputType::Directory(path))
                }
            }
            false => Err("Path does not exist")
        }
    }
}

pub fn create_hash(mut data: Vec<u8>) -> String {
    let mut hasher = Sha256::new();
    hasher.input(data.as_slice());
    hasher.result_str()
}