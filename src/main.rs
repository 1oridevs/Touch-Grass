mod token;
mod lexer;
mod parser;
mod ast;

use std::io::Write;

fn main() {
    println!("Touch Grass Programming Language v0.2.0");
    println!("Because you clearly need to...");

    loop {
        let mut input = String::new();
        print!("🌱 >> ");
        std::io::stdout().flush().unwrap();
        
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "exit" {
                    println!("Finally... touch grass my friend!");
                    break;
                }
                
                let mut lexer = lexer::Lexer::new(input);
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
            Err(error) => println!("Error: {}", error),
        }
    }
}
