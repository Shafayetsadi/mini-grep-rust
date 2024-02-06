use std::io;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::process;

fn read_file(path: &str) -> io::Result<io::BufReader<File>> {
    let file = File::open(path).expect("file not found");
    let reader = io::BufReader::new(file);
    Ok(reader)
}

fn grep(pattern: &str, reader: io::BufReader<File>) {
    let mut flag = false;
    for line in reader.lines() {
        let line = line.expect("unable to read line");
        if line.contains(pattern) {
            println!("{}", line); flag = true;
        }
    }
    if !flag {
        println!("No match found!");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} PATTERN FILE", args[0]);
        process::exit(1);
    }

    let pattern = &args[1];
    let path = &args[2];

    let reader = read_file(path).expect("unable to read file");
    grep(pattern, reader);
}
