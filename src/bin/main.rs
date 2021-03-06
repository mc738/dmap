use std::env;
use crate::Command::Error;
use dmap::common::InputType;
use std::process::exit;
use rlog::Log;

#[derive(Debug)]
pub struct MapCommand {
    path: String,
    output: String
}

#[derive(Debug)]
pub struct DiffCommand {
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
    Diff(DiffCommand),
    Error(CommandError)
}

#[derive(Debug)]
pub enum PathType {
    Directory(String),
    Map(String)
}

impl MapCommand {
    pub fn create(path: String, output: String) -> MapCommand {
        MapCommand {
            path,
            output
        }
    }
}

impl DiffCommand {
    pub fn create(dir_1: String, dir_2: String) -> DiffCommand {
        DiffCommand {
            dir_1,
            dir_2,
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

    Log::print_debug(String::from("dmap"), format!("Args: {:?}", args));
    let args_len = args.len();
    
    if args_len > 2 {

        match args[1].as_str() {
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
            "diff" => {
                if args_len >= 4 {

                    let comm = DiffCommand::create(args[2].clone(), args[3].clone());


                    Command::Diff(comm)
                }
                else {
                    Command::Error(CommandError::create(
                        "Too few args for `diff` command".to_string(),
                        "Syntax: `dmap compare [dir_1] [dir_2]`.".to_string()))
                }
            }
            _ => {
                Command::Error(CommandError::create(
                    format!("Unknown command: `{}`", args[1]),
                    "Known commands include `map` and `diff`.".to_string()))

            }
        }
 
    }
    else {
        Command::Error(CommandError::create(
            "Missing command".to_string(),
            "Try `dmap map [path] [output]` or `dmap diff [path1/map1] [path2/map2]`".to_string()))
    }
} 

fn run_command(command: Command) -> Result<(), &'static str> {
    
    match command {
        Command::Map(comm) => {
            Log::print_info(String::from("dmap"), format!("Mapping directory `{}`", comm.path));

            dmap::map(comm.path.as_ref(), comm.output.as_ref())
            
            // read_directory(comm.path.as_ref());
        }
        Command::Diff(comm) => {
            Log::print_info(String::from("dmap"), format!("Comparing directories `{}` and `{}`", comm.dir_1, comm.dir_2));
          
            let input_1 = InputType::create(comm.dir_1.as_str())?;
            let input_2 = InputType::create(comm.dir_2.as_str())?;

            dmap::diff(input_1, input_2)?;
            
            Ok(())
        }
        Error(error) => {
            
            // TODO push this upstream to `rlog`.
            let space = "                                      ";
            
            let message = format!("Command error! Exiting with code 2.\n{}Message: {}\n{}Hint: {}", space, error.message, space, error.hint);

            Log::print_error(String::from("dmap"), message);
            exit(2)
        }
    }
}

fn main() {
    
    let command = get_args();
    
    match run_command(command) {
        Ok(_) => {},
        Err(e) => {
                Log::print_error(String::from("dmap"), format!("Could not run command, message: `{}`. Exiting with code 1.", e));
                exit(1)
            }
            
    };
}


#[cfg(test)]
mod tests {
    use crate::{create_path_type, PathType};

    #[test]
    fn dir_PathType_test_() {
        
        let expected = PathType::Directory(String::from("/home/Test"));
        
        let actual = create_path_type("/home/Test");
        
        
        assert_eq!(expected, actual);
    }

    fn file_PathType_test_() {

        let expected = PathType::Directory(String::from("/home/foo/bar.txt"));

        let actual = create_path_type("/home/foo/bar.txt");


        assert_eq!(expected, actual);
    }

    #[test]
    fn dmap_PathType_test() {
        panic!("Make this test fail");
    }
}

