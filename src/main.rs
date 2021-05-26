#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

mod clock;
mod environment;
mod expr;
mod interpreter;
mod literal;
mod lox_callable;
mod lox_function;
mod parse_error;
mod parser;
mod scan_error;
mod scanner;
mod stmt;
mod token;
mod token_type;
mod value;

use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

use crate::interpreter::Interpreter;
use crate::parser::Parser;
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
    let scan_result = Scanner::scan_tokens(source);

    if scan_result.is_err() {
        eprintln!("{}", scan_result.unwrap_err());
        // code 65: incorrect input data
        std::process::exit(65);
    }

    let tokens = scan_result.unwrap();

    let mut parser = Parser::new(tokens.clone());
    let parse_result = parser.parse();

    if let Err(errors) = parse_result {
        for error in errors {
            eprintln!("{}", error);
        }
        std::process::exit(65);
    }

    let statements = parse_result.unwrap();
    Interpreter::new().interpret(statements.clone());

    for token in tokens {
        println!("{:?}", token);
    }

    for statement in statements {
        println!("{:?}", statement);
    }
}
