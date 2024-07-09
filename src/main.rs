mod scanner;

use std::{env, io};
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    let success = if args.len() == 1 {
        repl()
    } else if args.len() == 2 {
        run_file(&args[1])
    } else {
        eprintln!("Usage rust-lox [path]");
        false
    };

    if !success {
        std::process::exit(64);
    }
}

fn repl() -> bool {
    let mut line = String::new();
    let stdin = io::stdin();

    loop {
        print!("> ");

        match stdin.read_line(&mut line) {
            Ok(0) => return true,
            Ok(_) => println!("interpret({line})"),
            Err(err) => {
                eprintln!("Error reading line: {}.", err.to_string());
                return false;
            }
        }
    }
}

fn run_file(path: &str) -> bool {
   let source = read_file(path);
    match source {
        Err(_) => return false,
        _ => {}
    }

    let source = source.unwrap();
    println!("interpret({source})");

    return true
}

fn read_file(path: &str) -> io::Result<String> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Could not open file {path}.");
            return Err(err)
        },
    };

    let mut contents = String::new();
    return match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(err) => {
            eprintln!("Could not read file {path}.");
            Err(err)
        }
    }
}
