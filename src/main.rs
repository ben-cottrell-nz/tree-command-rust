use std::env;
use std::fmt::Debug;
use std::fs::{self, DirEntry, FileType};
use std::path;
use std::path::Path;

fn display_tree_for_directory(path_str: &str, depth: u32) {
    let BRANCH_SEP = "-";
    let OPEN_SEP = "+";
    let SPACE_SEP = "  ";
    let mut current_line: String = String::new();
    let mut _depth = depth;
    if _depth > 0 {
        for i in 0.._depth {
            current_line += SPACE_SEP;
        }
    }
    current_line += &*format!("{}-- ", OPEN_SEP);
    current_line += Path::new(path_str).file_name().unwrap().to_str().unwrap();
    println!("{}", current_line);
    let path_reader = Path::new(path_str);
    for entry in path_reader.read_dir().expect("read_dir failed") {
        if let Ok(entry) = entry {
            let file_type = entry.file_type().unwrap();
            if file_type.is_dir() {
                display_tree_for_directory(entry.path().to_str().unwrap(),
                         _depth+1);
            } else if file_type.is_file() {
                for i in 0.._depth {
                    print!("{}{}", SPACE_SEP, SPACE_SEP);
                }
                println!("├{} {}", BRANCH_SEP, entry.file_name().to_str().unwrap());
            }
        }
    }
}

fn process_args() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: tree [dir]");
    } else {
        if Path::new(&args[1]).is_dir() {
            display_tree_for_directory(&args[1], 0);
        } else {
            println!("invalid dir: {}",&args[1]);
        }
    }
}

fn main() {
    process_args()
}