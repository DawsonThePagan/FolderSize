use std::{env, fs, process, path::Path};
use fs_extra::dir::get_size;

const RET_OK: i32 = 0;
const RET_ERR_ARG: i32 = 1;
const RET_ERR_IO: i32 = 2;

const VERSION: &str = "V1.0";

const ARG_HELP: &str = "/help";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Not enough arguments.");
        process::exit(RET_ERR_ARG)
    }

    if args[1].to_lowercase() == ARG_HELP {
        println!("FolderSize {VERSION}\n");
        println!("Print out the size of folders recursively from the command line\n");
        println!("Arguments:");
        println!(" - /help = Optional, show the help text");
        println!(" - {{path}} = Get folder sizes from this folder\n");
        println!("Returns:");
        println!(" - 0 = OK, operation successful");
        println!(" - 1 = Error, Incorrect arguments");
        println!(" - 2 = Error, IO");
        process::exit(RET_OK)
    } 

    if visit_dirs(Path::new(args[1].as_str())) {
        process::exit(RET_OK)
    }
    else {
        process::exit(RET_ERR_IO)
    }
}


/// Read directories recursively, get the size of each one
fn visit_dirs(dir: &Path) -> bool {
    // Make sure we were given a dir
    if dir.is_dir() {
        // Calculate size of current dir
        let path = match dir.to_str() {
            Some(x) => x.to_string(),
            None => {
                println!("Could not convert path into something usable");
                return false;
            }
        };
        let size = match get_size(dir) {
            Ok(x) => x,
            Err(e) => {
                println!("Could not get size of dir {path}, {e}");
                return false;
            }
        } / 1000000;

        // Print out path and size
        println!("{path}");
        println!("{size}MB");

        // Walk the dir and look for sub dirs
        let read = match fs::read_dir(dir) {
            Ok(x) => x,
            Err(e) => {
                println!("Could not walk {path}, {e}");
                return false; 
            }
        };

        for entry in read {
            let checked = match entry {
                Ok(x) => x,
                Err(e) => {
                    println!("Could verify dir entry, {e}");
                    return false;
                }
            };
            let path = checked.path();
            if path.is_dir() {
                println!();
                visit_dirs(&path);
            }
        }
    } else {
        return false
    }
    true
}