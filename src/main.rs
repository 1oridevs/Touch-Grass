mod token;
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
                println!("You entered: {}", input.trim());
            }
            Err(error) => println!("error: {}", error),
        }
    }
}