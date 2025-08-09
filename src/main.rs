#[allow(unused_imports)]
use std::io::{self, Write};
mod modules;
use modules::tokenizer::*;
use modules::prossess::*;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let _bytes = io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        let tokens = match tokenizer(input) {
            Ok(tokens) => tokens,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
                
        };
        
       match prossess(tokens) {
        Ok(v) => {
            if v.as_str() == "exit" {
                break;
            }else{
                println!("{}",v)
            }
        },
        Err(e) => println!("{}",e),
       }
    }
}
