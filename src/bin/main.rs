use std::fs::{File, Metadata, read_dir};
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use std::io::{BufReader};
use std::io::prelude::*;
use std::{io, env};
use std::path::Path;
use std::iter::Map;
use crate::Command::Error;

#[derive(Debug)]
pub struct MapCommand {
    path: String,
    output: String
}

#[derive(Debug)]
pub struct CompareCommand {
    dir_1: String,
    dir_2: String
}

#[derive(Debug)]
pub struct CommandError {
    message: String,
    hint: String
}

#[derive(Debug)]
pub enum Command {
    Map(MapCommand),
    Compare(CompareCommand),
    Error(CommandError)
}

impl MapCommand {
    pub fn create(path: String, output: String) -> MapCommand {
        MapCommand {
            path,
            output
        }
    }
}

impl CompareCommand {
    pub fn create(dir_1: String, dir_2: String) -> CompareCommand {
        CompareCommand {
            dir_1,
            dir_2
        }
    }
}

impl CommandError {
    pub fn create(message: String, hint: String) -> CommandError {
        CommandError {
            message,
            hint
        }
    }
}

fn get_args() -> Command {
    let args: Vec<String> = env::args().collect();

    println!("Args: {:?}", args);
    
    let run = "run".to_string();
    
    let args_len = args.len();
    
    if args_len > 2 {
        match &args[1] {
            command => {
                match command.as_ref() {
                    "map" => {
                        if args_len >= 4 {
                            Command::Map(MapCommand::create(args[2].clone(), args[3].clone()))
                        }
                        else {
                            Command::Error(CommandError::create(
                                "Too few args for `map` command".to_string(),
                                "Syntax: `dmap map [path] [output]`.".to_string()))
                        }
                    }
                    "compare" => {
                        if args_len >= 4 {
                            Command::Compare(CompareCommand::create(args[2].clone(), args[3].clone()))
                        }
                        else {
                            Command::Error(CommandError::create(
                                "Too few args for `compare` command".to_string(),
                                "Syntax: `dmap compare [dir_1] [dir_2]`.".to_string()))
                        }
                    }
                    _ => {
                        Command::Error(CommandError::create(
                            format!("Unknown command: `{}`", command),
                            "Known commands include `map` and `compare`.".to_string()))
                        
                    }
                }
            }
            _ => {
                Command::Error(CommandError::create(
                    "Unknown command".to_string(),
                    "Try `dmap map [path] [output]` or `dmap compare [result1] [result2]`".to_string()))
            }
        }  
    }
    else {
        Command::Error(CommandError::create(
            "Missing command".to_string(),
            "Try `dmap map [path] [output]` or `dmap compare [result1] [result2]`".to_string()))
    }
} 

fn run_command(command: Command) -> Result<(), &'static str> {
    
    match command {
        Command::Map(comm) => {
            println!("Mapping directory `{}`...", comm.path);
            
            dmap::map(comm.path.as_ref(), comm.output.as_ref());
            
            // read_directory(comm.path.as_ref());
        }
        Command::Compare(comm) => {
            println!("Comparing directories `{}` and `{}`...", comm.dir_1, comm.dir_2);
            
            dmap::compare(comm.dir_1.as_ref(), comm.dir_2.as_ref());
        }
        Error(error) => {
            println!("Command error!");
            println!("\tMessage: {}", error.message);
            println!("\tHint: {}", error.hint);
        }
    };
    
    Ok(())
}

fn main() {
    
    let command = get_args();
    
    // println!("Command: {:?}", command);

    run_command(command);

    // read_directory("/home/max/Projects/dmap/".as_ref());
    // hash_file("/home/max/Data/HelloWorld.txt");
    // println!("Hello, world!");
}