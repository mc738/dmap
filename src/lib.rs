use std::fs::{File, read_dir};
use std::io::{BufReader, Read, Write};
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use std::path::Path;
use std::io;
use crate::common::{FileInfo, DirectoryInfo};

pub mod common;
pub mod map;
pub mod compare;

/// A wrapper for the `map` command.
/// 
/// This will attempt to map a directory and sub directories,
/// then create and save a `.dmap` file with the results.  
/// 
/// # Arguments
/// 
/// * `path` - The directory path.
/// * `output` - The path to save the results to.
pub fn map(path: &Path, output: &Path) {

    // Map the directory and sub directories.
    // TODO add excludes/ignores.
    
    println!("Mapping directory `{:?}`", path);
    
    let map = map_directory(path).unwrap();

    let json = serde_json::to_string(&map).unwrap();
    
    println!("Save map to `{:?}`", output);
    
    let mut output = File::create(output).unwrap();
    output.write_all(json.as_ref());


    println!("Done");
    // Save the output as a `.dmap` file.
}

fn map_directory(path: &Path) -> io::Result<DirectoryInfo> {
    let mut entries = read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();

    // println!("Directory: {:?}", entries);
    // The entries have now been sorted by their path.

    let mut files: Vec<FileInfo> = Vec::new();
    let mut children: Vec<DirectoryInfo> = Vec::new();
    
    for entry in entries {
        let file = File::open(&entry);

        match file {
            Ok(mut f) => {
                let mut data: Vec<u8> = Vec::new();
                let metadata = f.metadata()?;

                if metadata.is_file() {
                    f.read_to_end(&mut data);

                    let hash = create_hash(data);
                    
                    let fi = FileInfo::create(entry.into_os_string().into_string().unwrap(), hash);
                    
                    files.push(fi);
                    
                    //println!("Path: {:?}", entry);
                    // println!("\tHash: {}", hash);
                    // println!("\tMetadata: {:?}", metadata);
                } else {
                    // println!("Path: {:?}", &entry);
                    // println!("\t******* Is directory");
                    let dir = map_directory(entry.as_path())?;
                    children.push(dir);
                }
            }
            Err(_) => ()
        };
        //hash_file(entry.as_path());
    };
    
    Ok(DirectoryInfo::create(path.to_string_lossy().parse().unwrap(), "".to_string(), children, files))
}

fn create_hash(mut data: Vec<u8>) -> String {
    let mut hasher = Sha256::new();
    hasher.input(data.as_slice());
    hasher.result_str()
}

pub struct FileEntry {
    hash: String,
    path: String,
}

pub struct DirectoryEntry {}

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