mod token;
mod lexer;
mod parser;
mod ast;
mod interpreter;

use std::env;
use std::fs;
use std::io::{self, Write};
use interpreter::Interpreter;

/// Processes a source string: lexes, parses, then interprets the code.
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
    
    // Instead of printing the AST, run the interpreter.
    let mut interp = Interpreter::new();
    interp.interpret(ast);
}

/// Runs the interactive REPL.
fn run_repl() {
    println!("Touch Grass Programming Language v0.3.0");
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

/// Entry point: if a file path is provided, run that file; otherwise, start the REPL.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let filename = &args[1];
        match fs::read_to_string(filename) {
            Ok(contents) => {
                println!("Running file: {}", filename);
                run_source(contents);
            },
            Err(e) => eprintln!("Error reading file {}: {}", filename, e),
        }
    } else {
        run_repl();
    }
}
