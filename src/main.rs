mod token;
mod lexer;
mod parser;
mod ast;

use std::env;
use std::fs;
use std::io::{self, Write};

/// Processes a source string by lexing, parsing, and printing the resulting AST.
fn run_source(source: String) {
    let mut lexer = lexer::Lexer::new(source);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token();
        if token == token::Token::EOF {
            break;
        }
        tokens.push(token);
    }

    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse();
    println!("AST: {:#?}", ast);
}

/// Runs the interactive REPL.
fn run_repl() {
    println!("Touch Grass Programming Language v0.2.0");
    println!("Because you clearly need to...");

    loop {
        let mut input = String::new();
        print!("🌱 >> ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "exit" {
                    println!("Finally... touch grass my friend!");
                    break;
                }
                run_source(input);
            }
            Err(error) => println!("Error: {}", error),
        }
    }
}

/// Entry point: if a file path is provided as an argument, run that file; otherwise, start the REPL.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // A file path was provided: read and run the file
        let filename = &args[1];
        match fs::read_to_string(filename) {
            Ok(contents) => {
                println!("Running file: {}", filename);
                run_source(contents);
            },
            Err(e) => eprintln!("Error reading file {}: {}", filename, e),
        }
    } else {
        // No file provided: launch the interactive REPL
        run_repl();
    }
}
