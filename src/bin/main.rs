use std::fs::{File, Metadata, read_dir};
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use std::io::{BufReader};
use std::io::prelude::*;
use std::io;
use std::path::Path;


fn crawl_directory() {
    
    // This will basically be a recursive call to `read_directory` 
    
    
}

fn read_directory(path: &Path) -> io::Result<()> {
    let mut entries = read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();
    
    println!("Directory: {:?}", entries);
    // The entries have now been sorted by their path.

    for entry in entries {
        
        let file = File::open(&entry);
        
        match file {
            Ok(mut f) => {
                let mut data: Vec<u8> = Vec::new();
                let metadata = f.metadata()?;
                
                if metadata.is_file() {
                    f.read_to_end(&mut data);

                    let hash = create_hash(data);

                    println!("Path: {:?}", entry);
                    println!("\tHash: {}", hash);
                    println!("\tMetadata: {:?}", metadata);    
                }
                else {
                    println!("Path: {:?}", &entry);
                    println!("\t******* Is directory");
                    read_directory(entry.as_path());
                }       
            },
            Err(_) => ()
        };
        //hash_file(entry.as_path());
    };
    
    Ok(())
}

fn create_hash(mut data: Vec<u8>) -> String {
    let mut hasher = Sha256::new();
    hasher.input(data.as_slice());
    hasher.result_str()
}


fn hash_file(path: &str) {
    let file = File::open(path);

    match file {
        Ok(mut f) => {

            // Hash the file.
            let mut hasher = Sha256::new();

            let mut data: Vec<u8> = Vec::new();
            
            let metadata = f.metadata();
            
            f.read_to_end(&mut data);
            
            hasher.input(data.as_slice());

            let result = hasher.result_str();

            println!("Hash: {}", result);
            println!("Metadata: {:?}", metadata);
        }
        Err(_) => println!("Path '{}' unreadable", path)
    }
}


fn main() {
    read_directory("/home/max/Projects/dmap/".as_ref());
    // hash_file("/home/max/Data/HelloWorld.txt");
    // println!("Hello, world!");
}