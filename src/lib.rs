use std::fs::{File, read_dir};
use std::io::{BufReader, Read, Write};
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use std::path::Path;
use std::io;
use crate::common::{FileInfo, DirectoryInfo};
use crate::map::{save_map, create_map};
use crate::diff::Diff;

pub mod common;
pub mod map;
pub mod compare;
pub mod diff;

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
    let map = map::create_map(path);

    match map {
        Ok(di) => {
            
            println!("Mapped! Directory hash: {}", di.hash);
            println!("Saving map to `{:?}`", output);
            match save_map(di, output) {
                Ok(_) => {
                    println!("Done");
                },
                Err(e) => {
                    println!("Error saving map: {}", e)
                }
            }
        }
        Err(e) => {
            println!("Error creating map: {}", e)
        }
    }
}

pub fn compare(path1: &Path, path2: &Path) -> Vec<Diff> {
    
    let map1 = create_map(path1).unwrap();
    let map2 = create_map(path2).unwrap();
    
    let diff = diff::calc_diff(map1, map2);
    
    println!("Diff: {:?}", diff);
    
    diff
}