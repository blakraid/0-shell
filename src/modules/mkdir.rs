use std::fs;

pub fn mkdir(args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        return Err("mkdir: missing operand".to_string());
    }

    let mut errors = Vec::new();
    let mut success_count = 0;

    for dir_name in args {
        match fs::create_dir(dir_name) {
            Ok(_) => {
                success_count += 1;
            }
            Err(e) => {
                let error_msg = match e.kind() {
                    std::io::ErrorKind::AlreadyExists => {
                        format!("mkdir: cannot create directory '{}': File exists", dir_name)
                    }
                    std::io::ErrorKind::PermissionDenied => {
                        format!(
                            "mkdir: cannot create directory '{}': Permission denied",
                            dir_name
                        )
                    }
                    std::io::ErrorKind::NotFound => {
                        format!(
                            "mkdir: cannot create directory '{}': No such file or directory",
                            dir_name
                        )
                    }
                    _ => {
                        format!("mkdir: cannot create directory '{}': {}", dir_name, e)
                    }
                };
                errors.push(error_msg);
            }
        }
    }

    if errors.is_empty() {
        Ok(String::new())
    } else if success_count > 0 {
        Err(errors.join("\n"))
    } else {
        Err(errors.join("\n"))
    }
}
