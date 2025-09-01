use std::fs;
use std::io::{self};

pub fn cat(args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        return input_echo();
    }

    let mut output = String::new();
    let mut errors = Vec::new();

    for file_path in args {
        if file_path == "-" {
            match input_echo() {
                Ok(content) => output.push_str(&content),
                Err(error_msg) => errors.push(error_msg),
            }
        } else {
            match read_file(file_path) {
                Ok(content) => output.push_str(&content),
                Err(error_msg) => errors.push(error_msg),
            }
        }
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    Ok(output)
}

fn input_echo() -> Result<String, String> {
    let mut full_output = String::new();
    let stdin = io::stdin();

    loop {
        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(0) => break, 
            Ok(_) => {
                let trimmed_line = line.trim_end();
                println!("{}", trimmed_line);

                full_output.push_str(&line);
            }
            Err(_) => break,
        }
    }

    Ok(full_output)
}

fn read_file(file_path: &str) -> Result<String, String> {
    match fs::read_to_string(file_path) {
        Ok(content) => Ok(content),
        Err(e) => {
            let error_msg = match e.kind() {
                std::io::ErrorKind::NotFound => {
                    format!("cat: {}: No such file or directory", file_path)
                }
                std::io::ErrorKind::PermissionDenied => {
                    format!("cat: {}: Permission denied", file_path)
                }
                _ => {
                    format!("cat: {}: {}", file_path, e)
                }
            };
            Err(error_msg)
        }
    }
}