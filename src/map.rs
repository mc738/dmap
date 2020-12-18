use std::fs::{File, read_dir};
use std::io::{BufReader, Read, Write};
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use std::path::Path;
use std::io;
use crate::common::{DirectoryInfo, FileInfo};
use crate::common;

/// A wrapper for the `map` command.
/// 
/// This will attempt to map a directory and sub directories,
/// then create and save a `.dmap` file with the results.  
/// 
/// # Arguments
/// 
/// * `path` - The directory path.
/// * `output` - The path to save the results to.
pub fn create_map(path: &Path) -> Result<DirectoryInfo, &'static str> {
    // Map the directory and sub directories.
    // TODO add excludes/ignores.
    Ok(map_directory(path).unwrap())
}

pub fn save_map(map: DirectoryInfo, path: &Path) -> Result<(), &'static str> {
    let json = serde_json::to_string(&map).unwrap();
    
    let mut output = File::create(path).unwrap();
    output.write_all(json.as_ref());

    println!("Done");

    Ok(())
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

                    let hash = common::create_hash(data);

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
    
    let hash = hash_directory(&children, &files);

    Ok(DirectoryInfo::create(path.to_string_lossy().parse().unwrap(), hash, children, files))
}

fn hash_directory(children: &Vec<DirectoryInfo>, files: &Vec<FileInfo>) -> String {
    let mut buffer = String::new();

    let mut hasher = Sha256::new();

    // Append all directory hashes.
    for c in children {
        buffer.push_str(&c.hash);
        
    };

    // Append all file hashes.
    for f in files {
        buffer.push_str(&f.hash);
    };

    hasher.input_str(&*buffer);

    hasher.result_str()
}