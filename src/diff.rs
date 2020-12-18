use crate::common::{DirectoryInfo, FileInfo};
use std::fs::File;
use serde::de::Unexpected::Map;
use std::collections::HashMap;
use std::borrow::Borrow;

pub enum Diff {
    Add(String),
    Remove(String),
    Changed(String)
}


pub fn calc_diff(dir1: DirectoryInfo, dir2: DirectoryInfo) -> Vec<Diff> {
    // Check if the maps are the same length
    let mut map1 = dir1.files_to_dict();
    let mut map2 = dir2.files_to_dict();
    
    // let map1_len = map1.files.len();
    // let map2_len =map2.files.len();
    
    let pair = (map1.len(), map2.len());
    
    match pair {
        // If the same size.
        (m1, m2) if m1 == m2 => handle_equal_len(&map1, &mut map2),
        // If map1 is 0 then all files in map2 are new.
        (m1, _) if m1 == 0 => handle_map1_empty(&map2),
        // If map2 is 0 then all files in map1 are removed.
        (_, m2) if m2 == 0 => handle_map2_empty(&map1),
        // If map1 is bigger.
        (m1, m2) if m1 < m2 => handle_map1_larger(&mut map1, &mut map2),
        // If map2 is bigger.
        (m1, m2) if m1 > m2 => handle_map2_larger(&mut map1, &mut map2),
        _ => Vec::new() // This should not be hit.
    }
}

fn handle_equal_len(map1: &HashMap<String,String>, map2: &mut HashMap<String,String>) -> Vec<Diff> {
    
    let mut diffs: Vec<Diff> = Vec::new();
    
    // let mut found_in_map1: Vec<String> = Vec::with_capacity(map1.len());
    
    // We only need to record what has been found in map2.
    // We will cycle on map1, anything not found in map2 will be marked as `removed`.
    // If we all key a record of what has been found in map2 and then remove them.
    // The ones left will be new additions.
    let mut found_in_map2: Vec<String> = Vec::with_capacity(map1.len());
     
    // Run through map1 to match as much as possible.
    for (k,v) in map1 {
        // If file exists in both it is a change.
        match map2.contains_key(k) {
            true => {
                
                
                if v != map2.get(k).unwrap() {
                    diffs.push(Diff::Changed(k.clone()));
                };
                
                // Because it has been found we will keep a record.
                found_in_map2.push(k.clone());
            }
            false => {
                // If not found in map1 is a removal, 
                // and there will be an addition of new file in map2
                diffs.push(Diff::Remove(k.clone()));
            }
        }
    };
    
    // Remove all found items from map2
    for found in found_in_map2 {
        map2.remove(found.as_str());
    };
    
    // Anything left is an addition.
    for (k,_) in map2 {
        diffs.push(Diff::Add(k.clone()));
    };
    
    // TODO Add check to make sure both maps are empty.
    
    diffs
}

fn handle_map1_empty(map: &HashMap<String,String>) -> Vec<Diff> {
    let mut diffs: Vec<Diff> = Vec::new();

    for (k,_) in map {
        diffs.push(Diff::Add(k.clone()));
    };

    diffs
}

fn handle_map2_empty(map: &HashMap<String,String>) -> Vec<Diff> {
    let mut diffs: Vec<Diff> = Vec::new();

    for (k,_) in map {
        diffs.push(Diff::Remove(k.clone()));
    };

    diffs
}

fn handle_map1_larger(map1: &mut HashMap<String,String>, map2: &mut HashMap<String,String>) -> Vec<Diff> {

    let mut diffs: Vec<Diff> = Vec::new();

    // Go through map1, match any that don't exist in map2. 
    let mut not_found: Vec<String> = Vec::new();

    let iter = map1.into_iter();
    
    for (k, _) in iter {
        if !map2.contains_key(k) {
            not_found.push(k.clone());
        };
    };
    
    for i in not_found {
        diffs.push(Diff::Remove(i.clone()));
        map1.remove(&i);
    }
    
    // TODO Add check for maps equal size.
    
    let mut eql_result = handle_equal_len(map1, map2);
    
    diffs.append(&mut eql_result);
    
    diffs
}

fn handle_map2_larger(map1: &mut HashMap<String,String>, map2: &mut HashMap<String,String>) -> Vec<Diff> {

    let mut diffs: Vec<Diff> = Vec::new();

    // Go through map1, match any that don't exist in map2. 
    let mut not_found: Vec<String> = Vec::new();

    let iter = map2.into_iter();
    
    for (k, _) in iter {
        if !map1.contains_key(k) {
            not_found.push(k.clone());
        };
    };

    for i in not_found {
        diffs.push(Diff::Add(i.clone()));
        map2.remove(&i);
    }

    // TODO Add check for maps equal size.
    let mut eql_result = handle_equal_len(map1, map2);

    diffs.append(&mut eql_result);

    diffs
}

fn compare_hashes(file1: &FileInfo, file2: &FileInfo) -> bool {
    file1.hash == file2.hash
}