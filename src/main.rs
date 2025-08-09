#[allow(unused_imports)]
use std::io::{self, Write};
mod modules;
use modules::tokenizer::tokenizer;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let bytes = io::stdin().read_line(&mut input).unwrap();

        if bytes == 0 {
            println!("\nExiting shell.");
            break;
        }

        let input = input.trim();

        if input == "exit" {
            break;
        }

        match tokenizer(input) {
            Ok(tokens) => println!("{:?}", tokens),
            Err(e) => eprintln!("Tokenizer error: {}", e),
        }
    }
}