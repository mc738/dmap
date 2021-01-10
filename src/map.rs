use std::fs::{File, read_dir};
use std::io::{Read, Write};
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use std::path::{Path};
use std::io;
use crate::common::{DirectoryInfo, FileInfo, InputType};
use crate::common;
use std::collections::HashMap;
use rlog::Log;

pub struct DMap {
    base_path: String,
    dir: DirectoryInfo,
    //signature: String,
}

impl DMap {
    
    pub fn create_from_input(input: InputType) -> Result<DMap, &'static str> {
        match input {
            InputType::Directory(p) => DMap::create(p),
            InputType::Map(p) => DMap::load(p)
        }
    }
    
    pub fn create(path: &Path) -> Result<DMap, &'static str> {
        
        // TODO handle this better!
        let base_path = path.display().to_string();
        
        match map_directory(path, path) {
            Ok(dir) => Ok(DMap {
                base_path,
                dir,
            }),
            Err(_) => Err("Could not create map")
        }
    }
    
    pub fn load(path: &Path) -> Result<DMap, &'static str> {

        let base_path = path.display().to_string();
        
        // TODO Remove unwrap.
        let mut file = File::open(path).unwrap();
        
        let mut json = String::new();
        
        match file.read_to_string(&mut json) {
            Ok(_) => {
                // TODO Remove unwrap.
                let dir: DirectoryInfo = serde_json::from_str(json.as_str()).unwrap();
                Ok(DMap {
                    base_path,
                    dir,
                })
            }
            Err(_) => Err("Could not parse map")
        }
    }
    
    pub fn get_hash(&self) -> String {
        self.dir.hash.clone()
    }
    
    pub fn get_base_path(&self) -> String { self.base_path.clone() }
    
    pub fn flatten(&self) -> HashMap<String, String> {
        self.dir.flatten()
    }
    
    pub fn save(&self, path: &Path) -> Result<(), &'static str> {
        let json = serde_json::to_string(&self.dir).unwrap();

        // TODO remove unwrap.
        let mut output = File::create(path).unwrap();
        match output.write_all(json.as_ref()) {
            Ok(_) => {
                
                Log::print_success(String::from("dmap"), String::from("Mapped successfully"));
                
                Ok(())
            }
            Err(_) => Err("Could not save map.")
        }

        
    }
}

fn map_directory(path: &Path, base_path: &Path) -> io::Result<DirectoryInfo> {
    let mut entries = read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.
    // This may not be needed, tests pass without it, but it makes the map nicer.
    entries.sort();

    let mut files: Vec<FileInfo> = Vec::new();
    let mut children: Vec<DirectoryInfo> = Vec::new();

    for entry in entries {
        let file = File::open(&entry);

        match file {
            Ok(mut f) => {
                let mut data: Vec<u8> = Vec::new();
                let metadata = f.metadata()?;

                if metadata.is_file() {
                    match f.read_to_end(&mut data) {
                        Ok(_) => {
                            let hash = common::create_hash(data);

                            // TODO clean up this, or make a helper.
                            let fi = FileInfo::create(entry.strip_prefix(base_path).expect("").to_str().expect("").parse().unwrap(), hash);

                            files.push(fi);
                        }
                        Err(_) => {}
                    }
                } else {
                    let dir = map_directory(entry.as_path(), base_path)?;
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
