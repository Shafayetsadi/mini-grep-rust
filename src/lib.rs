use std::fs::File;
use std::{io, process};
use std::io::BufRead;

pub struct Args {
    pub pattern: String,
    pub path: String,
}

impl Args {
    pub fn parse(args: &Vec<String>) -> Args {
        let pattern = if args.len() == 3 {
            args[1].clone()
        } else {
            "".to_string()
        };
        let path = if args.len() == 3 {
            args[2].clone()
        } else {
            args[1].clone()
        };
        Args { pattern, path }
    }
}

pub fn read_file(path: &String) -> io::Result<io::BufReader<File>> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            println!("Error: Unable to read file! Please check the file path and file extension.");
            process::exit(0);
        }
    };
    let reader = io::BufReader::new(file);
    Ok(reader)
}

pub fn grep(pattern: &String, reader: io::BufReader<File>) {
    let mut flag = false;
    for line in reader.lines() {
        let line = line.expect("unable to read line");
        if pattern.is_empty() || line.contains(pattern) {
            println!("{}", line);
            flag = true;
        }
    }
    if !flag {
        println!("No match found!");
    }
}