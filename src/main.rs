mod token;
mod lexer;

use std::io::Write;

fn main() {
    println!("Touch Grass Programming Language v0.1.0");
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
                loop {
                    let token = lexer.next_token();
                    println!("{:?}", token);
                    if token == token::Token::EOF {
                        break;
                    }
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}