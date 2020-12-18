use std::fs::File;
use std::io::{BufReader, Read};
use crypto::sha2::Sha256;
use crypto::digest::Digest;

pub struct FileEntry {
    hash: String,
    path: String
}

pub struct DirectoryEntry {
    
}

impl DirectoryEntry {
    //pub fn create(&str path)
}

// impl FileEntry {
//     pub fn create(path: &str) -> Option<FileEntry> {
//         // 
//         let file = File::open(path);
//         
//         match file {
//             Ok(f) => {
//                 
//                 // Hash the file.
//                 let mut hasher = Sha256::new();
//                 
//                 // let mut hasher 
//                 let buffer = BufReader::new(f);
//                 
//                 hasher.input(buffer.bytes()[0..]);
//                 
//                 let result = hasher.result_str();
//                 
//                 
//                 
//                 Some (FileEntry {
//                     hash: result,
//                     path: path.to_string()
//                 })
//             }
//             Err(_) => None
//         }
//     }
// }