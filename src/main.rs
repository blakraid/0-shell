#[allow(unused_imports)]
use std::io::{self, Write};
mod modules;
use modules::prossess::*;
use modules::tokenizer::*;

fn main() {
    loop {
        print!("$ ");

        if let Err(e) = io::stdout().flush() {
            eprintln!("Failed to flush stdout: {}", e);
            break;
        }

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                
                println!(); 
                break;
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to read line: {}", e);
                break;
            }
        }

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let tokens = match tokenizer(input) {
            Ok(tokens) => tokens,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };
        // println!("Tokens: {:?}", tokens);
        match prossess(tokens, input) {
            Ok(v) => {
                if v.as_str() == "exit" {
                    break;
                } else {
                    print!("{}", v)
                }
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}
