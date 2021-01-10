use std::fs::{File, read_dir};
use std::io::{BufReader, Read, Write};
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use std::path::Path;
use std::io;
use crate::common::{FileInfo, DirectoryInfo, InputType};
use crate::map::{save_map, create_map, DMap};
use crate::diff::{Diff, DiffReport};

pub mod common;
pub mod map;
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
    let map = DMap::create(path);

    match map {
        Ok(map) => {
            
            println!("Mapped! Directory hash: {}", map.get_hash());
            println!("Saving map to `{:?}`", output);
            match map.save(output) {
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

pub fn diff(path1: InputType, path2: InputType) -> DiffReport {
    
    // TODO remove unwraps
    let map1 = DMap::create_from_input(path1).unwrap();
    let map2 = DMap::create_from_input(path2).unwrap();
    
    let diff = DiffReport::calc_diff(map1, map2);

    println!("Diff: {:?}", diff);

    diff
}