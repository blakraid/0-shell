use std::fs;
use std::path::Path;

pub fn cp(args: &[String]) -> Result<String, String> {
    if args.len() < 2 {
        return Err("cp: missing file operand\nUsage: cp SOURCE... DEST".to_string());
    }

    let (sources, destination) = args.split_at(args.len() - 1);
    let dest = &destination[0];
    let dest_path = Path::new(dest);

    if sources.len() > 1 && !dest_path.is_dir() {
        return Err(format!("cp: target '{}' is not a directory", dest));
    }

    let mut errors = Vec::new();
    let mut success_count = 0;

    for source in sources {
        match copy_single_file(source, dest) {
            Ok(_) => success_count += 1,
            Err(error_msg) => errors.push(error_msg),
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

fn copy_single_file(source: &str, destination: &str) -> Result<(), String> {
    let source_path = Path::new(source);
    let dest_path = Path::new(destination);

    if !source_path.exists() {
        return Err(format!(
            "cp: cannot stat '{}': No such file or directory",
            source
        ));
    }

    if source_path.is_dir() {
        return Err(format!(
            "cp: -r not specified; omitting directory '{}'",
            source
        ));
    }

    let final_dest = if dest_path.is_dir() {
        match source_path.file_name() {
            Some(file_name) => dest_path.join(file_name),
            None => return Err(format!("cp: invalid source file name '{}'", source)),
        }
    } else {
        dest_path.to_path_buf()
    };

    if let Some(parent) = final_dest.parent() {
        if !parent.exists() {
            if let Err(e) = fs::create_dir_all(parent) {
                return Err(format!(
                    "cp: cannot create directory '{}': {}",
                    parent.display(),
                    e
                ));
            }
        }
    }

    match fs::copy(source, &final_dest) {
        Ok(_) => Ok(()),
        Err(e) => {
            let error_msg = match e.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    format!(
                        "cp: cannot create regular file '{}': Permission denied",
                        final_dest.display()
                    )
                }
                std::io::ErrorKind::NotFound => {
                    format!(
                        "cp: cannot create regular file '{}': No such file or directory",
                        final_dest.display()
                    )
                }
                _ => {
                    format!(
                        "cp: cannot copy '{}' to '{}': {}",
                        source,
                        final_dest.display(),
                        e
                    )
                }
            };
            Err(error_msg)
        }
    }
}
