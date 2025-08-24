use std::fs;

pub fn cat(args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        return Err("cat: missing file operand".to_string());
    }

    let mut all_contents = Vec::new();
    let mut errors = Vec::new();

    for file_name in args {
        match read_single_file(file_name) {
            Ok(contents) => {
                all_contents.push(contents);
            }
            Err(error_msg) => {
                errors.push(error_msg);
            }
        }
    }

    let mut result = String::new();

    if !all_contents.is_empty() {
        result.push_str(&all_contents.join("\n"));
    }

    if !errors.is_empty() {
        if !result.is_empty() {
            result.push('\n');
        }
        result.push_str(&errors.join("\n"));
    }

    if result.is_empty() {
        Ok(String::new())
    } else {
        Ok(result)
    }
}

fn read_single_file(file_name: &str) -> Result<String, String> {
    match fs::read_to_string(file_name) {
        Ok(contents) => {
            Ok(contents)
        }
        Err(e) => {
            let error_msg = match e.kind() {
                std::io::ErrorKind::NotFound => {
                    format!("cat: {}: No such file or directory", file_name)
                }
                std::io::ErrorKind::PermissionDenied => {
                    format!("cat: {}: Permission denied", file_name)
                }
                std::io::ErrorKind::IsADirectory => {
                    format!("cat: {}: Is a directory", file_name)
                }
                _ => {
                    format!("cat: {}: {}", file_name, e)
                }
            };
            Err(error_msg)
        }
    }
}
