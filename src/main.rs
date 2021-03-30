#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use std::env;

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
    // do something
}

fn run_file(path: &str) {
    // do something
}
