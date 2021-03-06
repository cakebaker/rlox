#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

mod clock;
mod environment;
mod error;
mod expr;
mod interpreter;
mod literal;
mod lox_callable;
mod lox_function;
mod parser;
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
            println!("Usage: rlox [script]");
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
    let scan_result = Scanner::scan(source);

    if scan_result.is_err() {
        eprintln!("{}", scan_result.unwrap_err());
        // code 65: incorrect input data
        std::process::exit(65);
    }

    let tokens = scan_result.unwrap();

    let parse_result = Parser::new().parse(tokens.clone());

    if let Err(errors) = parse_result {
        for error in errors {
            eprintln!("{}", error);
        }
        std::process::exit(65);
    }

    let statements = parse_result.unwrap();
    let interpret_result = Interpreter::new().interpret(statements.clone());

    if interpret_result.is_err() {
        eprintln!("{}", interpret_result.unwrap_err());
    }

    for token in tokens {
        println!("{:?}", token);
    }

    for statement in statements {
        println!("{:?}", statement);
    }
}
