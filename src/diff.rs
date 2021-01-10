use crate::common::{DirectoryInfo, FileInfo};
use std::fs::File;
use serde::de::Unexpected::Map;
use std::collections::HashMap;
use std::borrow::Borrow;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Diff {
    Add(String),
    Remove(String),
    Changed(String)
}

pub fn calc_diff(dir1: DirectoryInfo, dir2: DirectoryInfo) -> Vec<Diff> {
    // Check if the maps are the same length
    let mut map1 = dir1.flatten();
    let mut map2 = dir2.flatten();
    
    let mut diffs: Vec<Diff> = Vec::new();

    let pair = (map1.len(), map2.len());

    let mut file_diff = match pair {
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
    };
    
    //diffs.append(&mut dir_diff);
    diffs.append(&mut file_diff);
    
    diffs.sort();
    
    diffs
    
}

fn handle_equal_len(map1: &HashMap<String,String>, map2: &mut HashMap<String,String>) -> Vec<Diff> {
    
    let mut diffs: Vec<Diff> = Vec::new();

    // We only need to record what has been found in map2.
    // We will cycle on map1, anything not found in map2 will be marked as `removed`.
    // If we all key a record of what has been found in map2 and then remove them.
    // The ones left will be new additions.
    let mut found_in_map2: Vec<String> = Vec::with_capacity(map1.len());
     
    // Run through map1 to match as much as possible.
    for (k,v) in map1 {
        // If file exists in both it is a change.
        println!("{}", k);
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
    // This is important because map2 could now be larger.
    // *** Start ***
    // dir_1
    // |_foo
    // |_bar
    // dir_2
    // |_baz
    //
    // *** After ***
    // dir_1
    // dir_2
    // |_baz <- Baz would currently be missed.
    //
    // However it seems to work, so... (tests 4,5,6).
    // *** The comment is left for reference ***
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