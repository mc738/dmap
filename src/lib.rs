use std::path::Path;
use rlog::Log;
use crate::common::InputType;
use crate::map::DMap;
use crate::diff::DiffReport;

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
pub fn map(path: &Path, output: &Path) -> Result<(), &'static str> {
    // Map the directory and sub directories.
    // TODO add excludes/ignores.
    let map = DMap::create(path)?;
    
    map.save(output)?;
    
    Ok(())
}

pub fn diff(path1: InputType, path2: InputType) -> Result<DiffReport, &'static str> {
    let map1 = DMap::create_from_input(path1)?;
    let map2 = DMap::create_from_input(path2)?;
    
    let diff = DiffReport::calc_diff(map1, map2);

    Log::print_success(String::from("dmap"), format!("Diff: {:?}", diff));

    Ok(diff)
}