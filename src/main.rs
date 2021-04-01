#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

mod scanner;
mod token;
mod token_type;

use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

use crate::scanner::Scanner;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: rlow [script]");
            // exit code from https://www.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+4.3-RELEASE&format=html
            std::process::exit(64);
        }
    }
}

fn run_prompt() {
    for line in io::stdin().lock().lines() {
        run(&line.unwrap());
    }
}

fn run_file(path: &str) {
    let file_content = fs::read_to_string(path);

    match file_content {
        Ok(source) => run(&source),
        Err(e) => println!("{}: {}!", path, e),
    }
}

fn run(source: &str) {
    let tokens = Scanner::scan_tokens(source);

    for token in tokens {
        println!("{:?}", token);
    }
}