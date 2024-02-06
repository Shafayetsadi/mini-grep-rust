
use std::env;
use std::process;
use minigrep::Args;




fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: grep [PATTERN] [FILE]");
        process::exit(0);
    }
    else if args.len() > 3 {
        println!("Error: Too many arguments!");
        process::exit(0);
    }

    let args = Args::parse(&args);

    let pattern = &args.pattern;
    let path = &args.path;

    let reader = minigrep::read_file(path).unwrap();

    minigrep::grep(pattern, reader);
}
